use crate::palette::ColorPallete;
use crate::{
    cart::Cart,
    runtime::{wasm_runtime::WasmRuntime, Runtime},
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs;
use std::io::Cursor;
use std::io::{Read, Write};

pub struct Wars8Binary {
    path: String,
    name: String,
    binary: Vec<u8>,
    sprites: Vec<[[ColorPallete; 8]; 8]>,
    map: Vec<u8>,
}

impl Wars8Binary {
    pub const MAGIC: u32 = 0x28061969;

    pub fn new(path: &String) -> Wars8Binary {
        let metadata = match fs::metadata(&path) {
            Ok(md) => md,
            Err(why) => panic!("Unable to open file {}, reason: {}", path, why),
        };

        if !metadata.is_file() {
            panic!("{} is not a file!", path);
        }

        if metadata.len() >= u32::MAX as u64 {
            panic!(
                "{} is {} bytes too big! (total: {}, limit {})",
                path,
                metadata.len() - u32::MAX as u64,
                metadata.len(),
                u32::MAX
            );
        }

        let name: String = match path.rfind('/') {
            Some(pos) => (path[(pos + 1)..]).to_string(),
            None => path.clone(),
        };

        let mut reader = Cursor::new(match fs::read(&path) {
            Ok(bs) => bs,
            Err(why) => panic!("Unable to read {}, reason: {}", &path, why),
        });

        fn _corrupt(path: &String, reason: &str) -> ! {
            panic!("{} is invalid or corrupt, reason: {}", path, reason);
        }

        match reader.read_u32::<BigEndian>() {
            Ok(xc) => {
                if reader.read_u32::<BigEndian>().unwrap() != Wars8Binary::MAGIC {
                    _corrupt(path, "Magic invalid");
                }
            }
            Err(_) => _corrupt(path, "Magic invalid"),
        };

        let binary_size = match reader.read_u32::<LittleEndian>() {
            Ok(bs) => bs,
            Err(_) => _corrupt(path, "Binary size missing"),
        };

        let mut binary = vec![0; binary_size as usize];
        if let Err(why) = reader.read(binary.as_mut_slice()) {
            _corrupt(path, "Unable to read binary");
        }

        let sprite_count = match reader.read_u32::<LittleEndian>() {
            Ok(sc) => {
                if sc >= 128 {
                    panic!("Sprite count {} over limit", 128 - sc);
                }
                sc
            }
            Err(_) => 0,
        };

        let mut sprites: Vec<[[ColorPallete; 8]; 8]> = Vec::new();
        for sprite in 0..sprite_count {
            let mut sprite_buffer = [0u8; 8 * 8];
            if let Err(why) = reader.read(&mut sprite_buffer) {
                _corrupt(path, format!("Unable to read sprite {}", sprite).as_str());
            }

            for row in 0..8 {
                for col in 0..8 {
                    sprites[sprite as usize][row][col] =
                        ColorPallete::from(sprite_buffer[(row * 8) + col] as i32);
                }
            }
        }

        let map_count = match reader.read_u32::<LittleEndian>() {
            Ok(mc) => mc,
            Err(_) => 0,
        };

        let mut map = vec![0; map_count as usize];
        if let Err(why) = reader.read(map.as_mut_slice()) {
            _corrupt(path, "Unable to read map");
        }

        Wars8Binary {
            path: path.clone(),
            name,
            binary,
            sprites,
            map,
        }
    }
}

impl Cart for Wars8Binary {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> u32 {
        self.binary().len() as u32
    }

    fn binary(&self) -> &[u8] {
        &self.binary
    }

    fn get_sprite(&self, idx: i32) -> Option<[[ColorPallete; 8]; 8]> {
        if self.sprites.len() > idx as usize {
            Some(self.sprites[idx as usize])
        } else {
            None
        }
    }

    fn get_spritesheet(&self) -> [i32; 128*32] {
        [0; 128 * 32]
    }

    fn get_map_cell(&self, cellx: i32, celly: i32) -> u8 {
        let idx = (cellx + (celly * 128)) as usize;
        if self.map.len() > idx {
            self.map[idx]
        } else {
            0
        }
    }

    fn save(&self) -> Result<(), ()> {
        if self.binary().len() > u32::MAX as usize {
            println!(
                "Binary too large! {} bytes over limit, still going to save...",
                self.binary().len() - u32::MAX as usize
            );
        }

        let metadata = match fs::metadata(&self.path) {
            Ok(md) => md,
            Err(why) => panic!("Unable to open file {}, reason: {}", &self.path, why),
        };

        if metadata.is_dir() {
            panic!("{} is a directory!", &self.path);
        }

        let mut out_buf: Vec<u8> = Vec::new();
        out_buf.write_u32::<BigEndian>(Wars8Binary::MAGIC);

        out_buf.write_u32::<LittleEndian>(self.binary.len() as u32);
        out_buf.write(&self.binary);

        out_buf.write_u32::<LittleEndian>(self.sprites.len() as u32);
        for sprite in &self.sprites {
            for row in 0..8 {
                for col in 0..8 {
                    out_buf.push(i32::from(sprite[row][col]) as u8);
                }
            }
        }

        out_buf.write_u32::<LittleEndian>(self.map.len() as u32);
        out_buf.write(&self.map);

        fs::write(&self.path, out_buf);

        Ok(())
    }

    fn create_runtime(&self) -> Box<dyn Runtime> {
        Box::new(WasmRuntime::new(self.binary()))
    }
}

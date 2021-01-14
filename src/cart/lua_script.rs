use std::fs;

use crate::palette::ColorPallete;
use crate::runtime::lua_runtime::LuaRuntime;

use super::Cart;

pub struct LuaScript {
    path: String,
    name: String,
    script: Vec<u8>,
}

impl LuaScript {
    pub fn new(path: &String) -> Self {
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

        let script = match fs::read(&path) {
            Ok(bin) => bin,
            Err(why) => panic!("Unable to read {}, reason: {}", path, why),
        };

        LuaScript {
            path: path.clone(),
            name,
            script,
        }
    }
}

impl Cart for LuaScript {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> u32 {
        self.script.len() as u32
    }

    fn binary(&self) -> &[u8] {
        &self.script[..]
    }

    fn get_sprite(&self, idx: i32) -> Option<[[crate::palette::ColorPallete; 8]; 8]> {
        None
    }

    fn get_map_cell(&self, cellx: i32, celly: i32) -> u8 {
        0
    }

    fn save(&self) -> Result<(), ()> {
        match std::fs::write(&self.path, &self.script) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    fn create_runtime(&self) -> Box<dyn crate::runtime::Runtime> {
        Box::new(LuaRuntime::new(&self.script[..]))
    }
}

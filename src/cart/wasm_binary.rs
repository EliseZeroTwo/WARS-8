use crate::cart::Cart;
use std::fs;

pub struct WasmBinary {
    name: String,
    binary: Vec<u8>,
}

impl WasmBinary {
    pub fn new(path: String) -> WasmBinary {
        let metadata = match fs::metadata(&path) {
            Ok(md) => md,
            Err(why) => panic!("Unable to open file {}, reason: {}", path, why),
        };

        if !metadata.is_file() {
            panic!("{} is not a file!", path);
        }

        if metadata.len() >= u32::MAX as u64 {
            panic!("{} is {} bytes too big! ({} total)", path, metadata.len() - u32::MAX as u64, metadata.len());
        }

        let name: String = match path.rfind('/') {
            Some(pos) => {
                (path[(pos + 1)..]).to_string()
            },
            None => {
                path.clone()
            },
        };

        let binary = match fs::read(&path) {
            Ok(bin) => bin,
            Err(why) => panic!("Unable to read {}, reason: {}", path, why),
        };

        WasmBinary {
            name,
            binary
        }
    }
}

impl Cart for WasmBinary {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> u32 {
        self.binary().len() as u32
    }

    fn binary(&self) -> &[u8] {
        &self.binary
    }
}
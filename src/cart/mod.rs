pub mod lua_script;
pub mod p8_script;
pub mod wars_8_binary;
pub mod wasm_binary;

use lua_script::LuaScript;
use p8_script::P8Script;

use crate::palette::ColorPallete;
use crate::{runtime::Runtime, CART};

use self::{wars_8_binary::Wars8Binary, wasm_binary::WasmBinary};
pub trait Cart: Send + Sync {
    fn name(&self) -> String;
    fn size(&self) -> u32;
    fn binary(&self) -> &[u8];
    fn get_sprite(&self, idx: i32) -> Option<[[ColorPallete; 8]; 8]>;
    fn get_spritesheet(&self) -> [i32; 128*32];
    fn get_map_cell(&self, cellx: i32, celly: i32) -> u8;
    fn save(&self) -> Result<(), ()>;
    fn create_runtime(&self) -> Box<dyn Runtime>;
}

impl dyn Cart {
    pub fn load(path: &String) -> Box<dyn Cart + Send + Sync> {
        let ext = match path.rfind('.') {
            Some(idx) => path[(idx + 1)..].to_string(),
            None => String::new(),
        };

        match ext.to_lowercase().as_str() {
            "lua" => Box::new(LuaScript::new(&path)),
            "p8" => Box::new(P8Script::new(&path)),
            "rs8" => Box::new(Wars8Binary::new(&path)),
            "wasm" => Box::new(WasmBinary::new(&path)),
            _ => panic!("Unknown format: {}", ext),
        }
    }
}

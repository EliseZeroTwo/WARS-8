pub mod lua_script;
pub mod wars_8_binary;
pub mod wasm_binary;

use lua_script::LuaScript;

use crate::palette::ColorPallete;
use crate::{runtime::Runtime, CART};

use self::{wars_8_binary::Wars8Binary, wasm_binary::WasmBinary};
pub trait Cart: Send + Sync {
    fn name(&self) -> String;
    fn size(&self) -> u32;
    fn binary(&self) -> &[u8];
    fn get_sprite(&self, idx: i32) -> Option<[[ColorPallete; 8]; 8]>;
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
            "wasm" => Box::new(WasmBinary::new(&path)),
            "rs8" => Box::new(Wars8Binary::new(&path)),
            "lua" => Box::new(LuaScript::new(&path)),
            _ => panic!("Unknown format: {}", ext),
        }
    }
}

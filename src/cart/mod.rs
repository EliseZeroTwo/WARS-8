pub mod wasm_binary;

pub trait Cart {
    fn name(&self) -> String;
    fn size(&self) -> u32;
    fn binary(&self) -> &[u8];
}
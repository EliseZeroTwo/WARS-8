pub mod wasm_runtime;

pub enum Runtimes {
    None,
    Wasm,
}

pub trait Runtime {
    fn peek(&mut self, addr: u32) -> u8;
    fn poke(&mut self, addr: u32, val: u8);
    fn init(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct NoRuntime();
impl Runtime for NoRuntime {
    fn peek(&mut self, addr: u32) -> u8 {
        0
    }
    fn poke(&mut self, addr: u32, val: u8) {}
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn draw(&mut self) {}
}

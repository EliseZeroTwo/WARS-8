pub mod lua_runtime;
pub mod wasm_runtime;

pub enum Runtimes {
    None,
    Wasm,
    Lua,
}

pub trait Runtime {
    fn init(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
}
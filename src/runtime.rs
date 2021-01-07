pub enum Runtimes {
    None,
    Wasm,
}

pub trait Runtime {
    fn init(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct NoRuntime();
impl Runtime for NoRuntime {
    fn init(&mut self) { }
    fn update(&mut self) { }
    fn draw(&mut self) { }
}
use core::panic;
use std::{u32, usize};

use crate::runtime::Runtime;
use wasmtime::*;

pub struct WasmCallerWrapper<'a>(Caller<'a>);

impl<'a> WasmCallerWrapper<'_> {
    pub fn new(caller: Caller<'_>) -> WasmCallerWrapper {
        let wrapper = WasmCallerWrapper(caller);
        wrapper
    }

    pub fn get_memory(&self) -> Memory {
        let memory = match self.0.get_export("memory").unwrap() {
            Extern::Memory(mem) => mem,
            _ => panic!("invalid memory type exported"),
        };
        memory
    }

    pub fn peek(&self, addr: i32) -> u8 {
        let mut res = 0u8;
        let memory = self.get_memory();
        if (addr as usize) < memory.data_size() {
            unsafe {
                res = self.get_memory().data_unchecked()[addr as usize];
            }
        }
        res
    }

    pub fn poke(&self, addr: i32, val: u8) {
        let memory = self.get_memory();
        if (addr as usize) < memory.data_size() {
            unsafe {
                self.get_memory().data_unchecked_mut()[addr as usize] = val;
            }
        }
    }
}

pub struct WasmRuntime {
    engine: Engine,
    pub store: Option<Store>,
    pub module: Option<Module>,
    instance: Option<Instance>,
    init: Option<Func>,
    update: Option<Func>,
    draw: Option<Func>,
    memory: Option<Memory>,
}

impl WasmRuntime {
    pub fn new(binary: &[u8]) -> WasmRuntime {
        let mut rt = WasmRuntime {
            engine: Engine::new(Config::new().interruptable(true)),
            store: None,
            module: None,
            instance: None,
            init: None,
            update: None,
            draw: None,
            memory: None,
        };

        rt.store = Some(Store::new(&rt.engine));
        let module = Module::new(&rt.engine, binary);
        if let Err(why) = module {
            panic!("Invalid Binary! Reason (from WASMTime): {}", why);
        }
        rt.module = Some(module.unwrap());

        rt
    }

    pub fn update_api(&mut self, api: &[Extern]) {
        self.instance = Some(
            Instance::new(
                &self.store.as_ref().unwrap(),
                &self.module.as_ref().unwrap(),
                api,
            )
            .unwrap(),
        );
        self.memory = self.instance.as_ref().unwrap().get_memory("memory");
        if self.memory.is_none() {
            panic!("Memory region `memory` in cartridge not defined!");
        }

        if self.memory.as_ref().unwrap().data_size() > u32::MAX as usize {
            panic!(
                "Memory region `memory` is {:#X} pages which is above the maximum size of {:#X}",
                self.memory.as_ref().unwrap().size(),
                (u32::MAX / 64000)
            );
        }

        self.init = Some(
            self.instance
                .as_ref()
                .unwrap()
                .get_func("_init")
                .expect("`_init` was not an exported function"),
        );
        self.update = Some(
            self.instance
                .as_ref()
                .unwrap()
                .get_func("_update")
                .expect("`_update` was not an exported function"),
        );
        self.draw = Some(
            self.instance
                .as_ref()
                .unwrap()
                .get_func("_draw")
                .expect("`_draw` was not an exported function"),
        );
    }
}

impl Runtime for WasmRuntime {
    fn peek(&mut self, addr: u32) -> u8 {
        if addr as usize > self.memory.as_ref().unwrap().data_size() {
            unsafe {
                return self.memory.as_ref().unwrap().data_unchecked()[addr as usize];
            }
        }
        panic!(
            "Attempted to read from {:#X} which is outside of memory bounds of {:#X}",
            addr,
            self.memory.as_ref().unwrap().data_size()
        );
    }

    fn poke(&mut self, addr: u32, val: u8) {
        if addr as usize > self.memory.as_ref().unwrap().data_size() {
            unsafe {
                self.memory.as_ref().unwrap().data_unchecked_mut()[addr as usize] = val;
            }
        } else {
            panic!(
                "Attempted to write {:#X} to {:#X} which is outside of memory bounds of {:#X}",
                val,
                addr,
                self.memory.as_ref().unwrap().data_size()
            );
        }
    }

    fn init(&mut self) {
        self.init.as_ref().unwrap().get0::<()>().unwrap()().unwrap();
    }

    fn update(&mut self) {
        self.update.as_ref().unwrap().get0::<()>().unwrap()().unwrap();
    }

    fn draw(&mut self) {
        self.draw.as_ref().unwrap().get0::<()>().unwrap()().unwrap();
    }
}

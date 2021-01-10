use crate::api;

use core::panic;
use std::{u32, usize};

macro_rules! func_wrap {
    ($wasm_runtime:expr, $func:expr) => {
        Func::wrap(&$wasm_runtime.store.as_ref().unwrap(), $func).into()
    };
}

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

        let mut import_vec: Vec<Extern> = Vec::new();
        let mut missing_import_vec: Vec<String> = Vec::new();
        for import in rt.module.as_ref().unwrap().imports() {
            match import.name() {
                "cls" => import_vec.push(func_wrap!(rt, api::gfx::cls)),
                "rect" => import_vec.push(func_wrap!(rt, api::gfx::rect)),
                "rectfill" => import_vec.push(func_wrap!(rt, api::gfx::rectfill)),
                "pget" => import_vec.push(func_wrap!(rt, api::gfx::pget)),
                "pset" => import_vec.push(func_wrap!(rt, api::gfx::pset)),
                "print" => import_vec.push(func_wrap!(rt, api::gfx::print)),
                "printh" => import_vec.push(func_wrap!(rt, api::gfx::printh)),
                
                "btn" => import_vec.push(func_wrap!(rt, api::input::btn)),
                "btnp" => import_vec.push(func_wrap!(rt, api::input::btnp)),
                "key" => import_vec.push(func_wrap!(rt, api::input::key)),

                "abs" => import_vec.push(func_wrap!(rt, api::math::abs)),
                "atan2" => import_vec.push(func_wrap!(rt, api::math::atan2)),
                "band" => import_vec.push(func_wrap!(rt, api::math::band)),
                "bnot" => import_vec.push(func_wrap!(rt, api::math::bnot)),
                "bor" => import_vec.push(func_wrap!(rt, api::math::bor)),
                "bxor" => import_vec.push(func_wrap!(rt, api::math::bxor)),
                "cos" => import_vec.push(func_wrap!(rt, api::math::cos)),
                "flr" => import_vec.push(func_wrap!(rt, api::math::flr)),
                "maxf" => import_vec.push(func_wrap!(rt, api::math::maxf)),
                "max" => import_vec.push(func_wrap!(rt, api::math::max)),
                "mid" => import_vec.push(func_wrap!(rt, api::math::mid)),
                "min" => import_vec.push(func_wrap!(rt, api::math::min)),
                "minf" => import_vec.push(func_wrap!(rt, api::math::minf)),
                "rnd" => import_vec.push(func_wrap!(rt, api::math::rnd)),
                "sgn" => import_vec.push(func_wrap!(rt, api::math::sgn)),
                "shl" => import_vec.push(func_wrap!(rt, api::math::shl)),
                "shr" => import_vec.push(func_wrap!(rt, api::math::shr)),
                "sin" => import_vec.push(func_wrap!(rt, api::math::sin)),
                "sqrt" => import_vec.push(func_wrap!(rt, api::math::sqrt)),
                "sqrtf" => import_vec.push(func_wrap!(rt, api::math::sqrtf)),
                "srand" => import_vec.push(func_wrap!(rt, api::math::srand)),

                "exit" => import_vec.push(func_wrap!(rt, api::misc::exit)),
                "save" => import_vec.push(func_wrap!(rt, api::misc::exit)),
                "load" => import_vec.push(func_wrap!(rt, api::misc::load)),
                "unload" => import_vec.push(func_wrap!(rt, api::misc::unload)),
                _ => missing_import_vec.push(import.name().to_owned()),
            }
            println!("Attempting to import {}", import.name());
        }

        if missing_import_vec.len() != 0 {
            panic!(format!(
                "Missing {} imports: {:?}",
                missing_import_vec.len(),
                missing_import_vec
            ).as_str());
        }

        rt.instance = Some(
            Instance::new(
                &rt.store.as_ref().unwrap(),
                &rt.module.as_ref().unwrap(),
                &import_vec[..],
            )
            .unwrap(),
        );
        rt.memory = rt.instance.as_ref().unwrap().get_memory("memory");
        if rt.memory.is_none() {
            panic!("Memory region `memory` in cartridge not defined!");
        }

        if rt.memory.as_ref().unwrap().data_size() > u32::MAX as usize {
            panic!(
                "Memory region `memory` is {:#X} pages which is above the maximum size of {:#X}",
                rt.memory.as_ref().unwrap().size(),
                (u32::MAX / 64000)
            );
        }

        rt.init = Some(
            rt.instance
                .as_ref()
                .unwrap()
                .get_func("_init")
                .expect("`_init` was not an exported function"),
        );
        rt.update = Some(
            rt.instance
                .as_ref()
                .unwrap()
                .get_func("_update")
                .expect("`_update` was not an exported function"),
        );
        rt.draw = Some(
            rt.instance
                .as_ref()
                .unwrap()
                .get_func("_draw")
                .expect("`_draw` was not an exported function"),
        );

        rt
    }

    pub fn update_api(&mut self) {
        
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

use crate::runtime::Runtime;
use wasmtime::*;

pub struct WasmRuntime {
    engine: Engine,
    pub store: Option<Store>,
    pub module: Option<Module>,
    instance: Option<Instance>,
    init: Option<Func>,
    update: Option<Func>,
    draw: Option<Func>,
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
        self.instance = Some(Instance::new(&self.store.as_ref().unwrap(), &self.module.as_ref().unwrap(), api).unwrap());
        self.init = Some(self.instance.as_ref().unwrap().get_func("_init").expect("`_init` was not an exported function"));
        self.update = Some(self.instance.as_ref().unwrap().get_func("_update").expect("`_update` was not an exported function"));
        self.draw = Some(self.instance.as_ref().unwrap().get_func("_draw").expect("`_draw` was not an exported function"));
    }
}

impl Runtime for WasmRuntime {
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
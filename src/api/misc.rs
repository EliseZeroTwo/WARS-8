use wasmtime::Caller;

use crate::{CART, CART_TO_LOAD, cart::Cart, config::Config, runtime::wasm_runtime::WasmCallerWrapper, utils::read_cstr};

pub fn exit() {
    std::process::exit(0); // lol
}

pub fn save() -> i32 {
    match CART.lock().unwrap().as_ref() {
        Some(boxed_cart) => {
            match boxed_cart.save() {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0,
    }
}

pub fn unload() {
    *CART.lock().unwrap() = None;
}

pub fn load(caller: Caller, string_addr: i32) {
    let caller_wrapper = WasmCallerWrapper::new(caller);
    let mut str = read_cstr(&caller_wrapper, string_addr);
    str = str.replace("..", "");
    let path = std::path::Path::new(&Config::get_config_dir_or_create().unwrap()).join(str);
    if !path.is_file() {
        panic!("{} is not a file!", path.to_str().unwrap().to_string());
    }

    let path = path.to_str().unwrap().to_string();
    *CART.lock().unwrap() = Some(Cart::load(&path));
    *CART_TO_LOAD.lock().unwrap() = true;
    println!("Loaded {}", path);
}
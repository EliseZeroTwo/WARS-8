use mlua::{Function, Lua};

use crate::api;
use crate::runtime::Runtime;
pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new(script: &[u8]) -> Self {
        let lua = Lua::new();

        // GFX
        lua.globals().set("cls", lua.create_function(|_, color: i32| {
            api::gfx::cls(color);
            Ok(())
        }).unwrap()).unwrap();

        lua.globals().set("rect", lua.create_function(|_, args: (i32, i32, i32, i32, i32)| {
            api::gfx::rect(args.0, args.1, args.2, args.3, args.4);
            Ok(())
        }).unwrap()).unwrap();

        lua.globals().set("rectfill", lua.create_function(|_, args: (i32, i32, i32, i32, i32)| {
            api::gfx::rectfill(args.0, args.1, args.2, args.3, args.4);
            Ok(())
        }).unwrap()).unwrap();

        lua.globals().set("pget", lua.create_function(|_, args: (i32, i32)| {
            Ok(api::gfx::pget(args.0, args.1))
        }).unwrap()).unwrap();

        lua.globals().set("pset", lua.create_function(|_, args: (i32, i32, i32)| {
            api::gfx::pset(args.0, args.1, args.2);
            Ok(())
        }).unwrap()).unwrap();

        lua.globals().set("print", lua.create_function(|_, args: (String, i32, i32, i32)| {
            api::gfx::print(args.0, args.1, args.2, args.3);
            Ok(())
        }).unwrap()).unwrap();

        lua.globals().set("printh", lua.create_function(|_, string: String| {
            api::gfx::printh(string);
            Ok(())
        }).unwrap()).unwrap();

        // Input
        lua.globals().set("btn", lua.create_function(|_, args: (i32, i32)| {
            Ok(api::input::btn(args.0, args.1) != 0)
        }).unwrap()).unwrap();

        lua.globals().set("btnp", lua.create_function(|_, args: (i32, i32)| {
            Ok(api::input::btnp(args.0, args.1) != 0)
        }).unwrap()).unwrap();

        lua.globals().set("key", lua.create_function(|_, _: ()| {
            Ok(api::input::key())
        }).unwrap()).unwrap();

        // Math
        lua.globals().set("abs", lua.create_function(|_, x: f32| {
            Ok(api::math::abs(x))
        }).unwrap()).unwrap();

        lua.globals().set("atan2", lua.create_function(|_, (x, y): (f32, f32)| {
            Ok(api::math::atan2(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("band", lua.create_function(|_, (x, y): (i32, i32)| {
            Ok(api::math::band(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("bnot", lua.create_function(|_, x: i32| {
            Ok(api::math::bnot(x))
        }).unwrap()).unwrap();
        
        lua.globals().set("bor", lua.create_function(|_, (x, y): (i32, i32)| {
            Ok(api::math::bor(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("bxor", lua.create_function(|_, (x, y): (i32, i32)| {
            Ok(api::math::bxor(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("cos", lua.create_function(|_, x: f32| {
            Ok(api::math::cos(x))
        }).unwrap()).unwrap();

        lua.globals().set("flt", lua.create_function(|_, x: f32| {
            Ok(api::math::flr(x))
        }).unwrap()).unwrap();

        lua.globals().set("max", lua.create_function(|_, (x, y): (f32, f32)| {
            Ok(api::math::maxf(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("mid", lua.create_function(|_, (x, y, z): (f32, f32, f32)| {
            Ok(api::math::midf(x, y, z))
        }).unwrap()).unwrap();

        lua.globals().set("minf", lua.create_function(|_, (x, y): (f32, f32)| {
            Ok(api::math::minf(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("rnd", lua.create_function(|_, x: f32| {
            Ok(api::math::rndf(x))
        }).unwrap()).unwrap();

        lua.globals().set("sgn", lua.create_function(|_, x: f32| {
            Ok(api::math::sgnf(x))
        }).unwrap()).unwrap();

        lua.globals().set("shl", lua.create_function(|_, (x, y): (i32, i32)| {
            Ok(api::math::shl(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("shr", lua.create_function(|_, (x, y): (i32, i32)| {
            Ok(api::math::shr(x, y))
        }).unwrap()).unwrap();

        lua.globals().set("sin", lua.create_function(|_, x: f32| {
            Ok(api::math::sin(x))
        }).unwrap()).unwrap();

        lua.globals().set("sqrt", lua.create_function(|_, x: f32| {
            Ok(api::math::sqrtf(x))
        }).unwrap()).unwrap();

        lua.globals().set("srand", lua.create_function(|_, x: i32| {
            Ok(api::math::srand(x))
        }).unwrap()).unwrap();

        // Misc
        lua.globals().set("exit", lua.create_function(|_, _: ()| {
            api::misc::exit();
            Ok(())
        }).unwrap()).unwrap();

        lua.globals().set("save", lua.create_function(|_, _: ()| {
            Ok(api::misc::save())
        }).unwrap()).unwrap();
        
        lua.globals().set("load", lua.create_function(|_, str: String| {
            api::misc::load(str);
            Ok(())
        }).unwrap()).unwrap();

        lua.globals().set("unload", lua.create_function(|_, _: ()| {
            api::misc::unload();
            Ok(())
        }).unwrap()).unwrap();
        lua.load(script).exec().unwrap();

        LuaRuntime{
            lua
        }
    }

}

impl Runtime for LuaRuntime {
    fn init(&mut self) {
        match self.lua.globals().get::<&str, Function>("_init") {
            Ok(func) => func,
            Err(_) => panic!("Missing _init function"),
        }.call::<_, ()>(()).unwrap();
    }

    fn update(&mut self) {
        match self.lua.globals().get::<&str, Function>("_update") {
            Ok(func) => func,
            Err(_) => panic!("Missing _update function"),
        }.call::<_, ()>(()).unwrap();
    }
    
    fn draw(&mut self) {
        match self.lua.globals().get::<&str, Function>("_draw") {
            Ok(func) => func,
            Err(_) => panic!("Missing _draw function"),
        }.call::<_, ()>(()).unwrap();
    }
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut negative: Vec<i32> = Vec::new();
    let mut positive: Vec<i32> = Vec::new();
    nums.iter().for_each( |x| {
        if *x  < 0 {
            negative.push(x.pow(2));
        } else {
            positive.push(x.pow(2));
        }
    });

    negative.reverse();
    negative.append(&mut positive);
    negative
}
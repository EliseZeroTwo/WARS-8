use mlua::{Function, Lua, MultiValue, Table, Value};

use crate::runtime::Runtime;
use crate::{api, draw_state, get_sprite_flag, set_sprite_flag};
pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new(script: &[u8]) -> Self {
        let lua = Lua::new();

        // Audio
        lua.globals()
            .set(
                "music",
                lua.create_function(
                    |_, (n, fade_len, channel_mask): (i32, Option<i32>, Option<u32>)| {
                        api::music::music(n, fade_len.unwrap_or(0), channel_mask.unwrap_or(0));
                        Ok(())
                    },
                )
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "sfx",
                lua
                    .create_function(
                        |_,
                         (n, channel, offset, length): (
                            i32,
                            Option<i32>,
                            Option<i32>,
                            Option<i32>,
                        )| {
                            api::music::sfx(
                                n,
                                channel.unwrap_or(-1),
                                offset.unwrap_or(0),
                                length.unwrap_or(0),
                            );
                            Ok(())
                        },
                    )
                    .unwrap(),
            )
            .unwrap();

        // GFX
        lua.globals()
            .set(
                "camera",
                lua.create_function(|_, (x, y): (Option<i32>, Option<i32>)| {
                    api::gfx::camera(x.unwrap_or(0), y.unwrap_or(0));
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "circ",
                lua.create_function(
                    |_, (x, y, r, color): (i32, i32, Option<i32>, Option<i32>)| {
                        api::gfx::circ(
                            x,
                            y,
                            r.unwrap_or(4),
                            color.unwrap_or(i32::from(draw_state::get_pen_color(None))),
                        );
                        Ok(())
                    },
                )
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "circfill",
                lua.create_function(
                    |_, (x, y, r, color): (i32, i32, Option<i32>, Option<i32>)| {
                        api::gfx::circfill(
                            x,
                            y,
                            r.unwrap_or(4),
                            color.unwrap_or(i32::from(draw_state::get_pen_color(None))),
                        );
                        Ok(())
                    },
                )
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "cls",
                lua.create_function(|_, color: Option<i32>| {
                    api::gfx::cls(color.unwrap_or(i32::from(draw_state::get_pen_color(None))));
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "rect",
                lua.create_function(|_, args: (f32, f32, f32, f32, f32)| {
                    api::gfx::rect(
                        args.0 as i32,
                        args.1 as i32,
                        args.2 as i32,
                        args.3 as i32,
                        args.4 as i32,
                    );
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "rectfill",
                lua.create_function(|_, args: (f32, f32, f32, f32, f32)| {
                    api::gfx::rectfill(
                        args.0 as i32,
                        args.1 as i32,
                        args.2 as i32,
                        args.3 as i32,
                        args.4 as i32,
                    );
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "pal",
                lua.create_function(|_, (c0, c1, p): (Option<i32>, Option<i32>, Option<i32>)| {
                    if c0.is_none() || c1.is_none() {
                        api::gfx::clrpal();
                    } else {
                        api::gfx::pal(c0.unwrap(), c1.unwrap(), p.unwrap_or(0));
                    }
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "palt",
                lua.create_function(|_, (c, t): (Option<i32>, Option<bool>)| {
                    let t = match t.unwrap_or(false) {
                        false => 0,
                        true => 1,
                    };
                    api::gfx::palt(c.unwrap_or(-1), t);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "pget",
                lua.create_function(|_, args: (i32, i32)| Ok(api::gfx::pget(args.0, args.1)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "pset",
                lua.create_function(|_, args: (i32, i32, i32)| {
                    api::gfx::pset(args.0, args.1, args.2);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "print",
                lua.create_function(|_, args: (String, i32, i32, i32)| {
                    api::gfx::print(args.0, args.1, args.2, args.3);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "printh",
                lua.create_function(|_, string: String| {
                    api::gfx::printh(string);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "spr",
                lua.create_function(
                    |_,
                     (idx, x, y, w, h, flip_x, flip_y): (
                        f32,
                        f32,
                        f32,
                        Option<f32>,
                        Option<f32>,
                        Option<bool>,
                        Option<bool>,
                    )| {
                        let flip_x = match flip_x.unwrap_or(false) {
                            false => 0,
                            true => 1,
                        };

                        let flip_y = match flip_y.unwrap_or(false) {
                            false => 0,
                            true => 1,
                        };

                        api::gfx::spr(
                            None,
                            idx as i32,
                            x as i32,
                            y as i32,
                            w.unwrap_or(1.0),
                            h.unwrap_or(1.0),
                            flip_x,
                            flip_y,
                        );
                        Ok(())
                    },
                )
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "sspr",
                lua.create_function(
                    |_,
                     (sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y): (
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        Option<i32>,
                        Option<i32>,
                        Option<bool>,
                        Option<bool>,
                    )| {
                        let flip_x = match flip_x.unwrap_or(false) {
                            false => 0,
                            true => 1,
                        };

                        let flip_y = match flip_y.unwrap_or(false) {
                            false => 0,
                            true => 1,
                        };

                        api::gfx::sspr(
                            sx,
                            sy,
                            sw,
                            sh,
                            dx,
                            dy,
                            dw.unwrap_or(1),
                            dh.unwrap_or(1),
                            flip_x,
                            flip_y,
                        );
                        Ok(())
                    },
                )
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "map",
                lua.create_function(
                    |_,
                     (cel_x, cel_y, sx, sy, cel_w, cel_h, layer): (
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        Option<i32>,
                    )| {
                        api::gfx::map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer.unwrap_or(0));
                        Ok(())
                    },
                )
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "map",
                lua.create_function(
                    |_,
                     (cel_x, cel_y, sx, sy, cel_w, cel_h, layer): (
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        Option<i32>,
                    )| {
                        api::gfx::map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer.unwrap_or(0));
                        Ok(())
                    },
                )
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "mset",
                lua.create_function(|_, (x, y, val): (i32, i32, u8)| {
                    api::gfx::mset(x, y, val);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "mget",
                lua.create_function(|_, (x, y): (i32, i32)| Ok(api::gfx::mget(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "fset",
                lua.create_function(|_, (sprite_idx, f, val): (u8, Option<u8>, bool)| {
                    set_sprite_flag(
                        None,
                        sprite_idx as i32,
                        f,
                        match val {
                            false => 0,
                            true => 1,
                        },
                    );
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "fget",
                lua.create_function(|_, (sprite_idx, f): (u8, Option<u8>)| {
                    Ok(get_sprite_flag(None, sprite_idx as i32, f) != 0)
                })
                .unwrap(),
            )
            .unwrap();

        // Input
        lua.globals()
            .set(
                "btn",
                lua.create_function(|_, (idx, player): (i32, Option<i32>)| {
                    Ok(api::input::btn(idx, player.unwrap_or(0)) != 0)
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "btnp",
                lua.create_function(|_, (idx, player): (i32, Option<i32>)| {
                    Ok(api::input::btnp(idx, player.unwrap_or(0)) != 0)
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "key",
                lua.create_function(|_, _: ()| Ok(api::input::key()))
                    .unwrap(),
            )
            .unwrap();

        // Math
        lua.globals()
            .set(
                "abs",
                lua.create_function(|_, x: f32| Ok(api::math::abs(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "atan2",
                lua.create_function(|_, (x, y): (f32, f32)| Ok(api::math::atan2(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "band",
                lua.create_function(|_, (x, y): (i32, i32)| Ok(api::math::band(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "bnot",
                lua.create_function(|_, x: i32| Ok(api::math::bnot(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "bor",
                lua.create_function(|_, (x, y): (i32, i32)| Ok(api::math::bor(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "bxor",
                lua.create_function(|_, (x, y): (i32, i32)| Ok(api::math::bxor(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "cos",
                lua.create_function(|_, x: f32| Ok(api::math::cos(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "flr",
                lua.create_function(|_, x: f32| Ok(api::math::flr(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "max",
                lua.create_function(|_, (x, y): (f32, f32)| Ok(api::math::maxf(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "mid",
                lua.create_function(|_, (x, y, z): (f32, f32, f32)| Ok(api::math::midf(x, y, z)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "min",
                lua.create_function(|_, (x, y): (f32, f32)| Ok(api::math::minf(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "rnd",
                lua.create_function(|_, x: f32| Ok(api::math::rndf(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "sgn",
                lua.create_function(|_, x: f32| Ok(api::math::sgnf(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "shl",
                lua.create_function(|_, (x, y): (i32, i32)| Ok(api::math::shl(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "shr",
                lua.create_function(|_, (x, y): (i32, i32)| Ok(api::math::shr(x, y)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "sin",
                lua.create_function(|_, x: f32| Ok(api::math::sin(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "sqrt",
                lua.create_function(|_, x: f32| Ok(api::math::sqrtf(x)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "srand",
                lua.create_function(|_, x: i32| Ok(api::math::srand(x)))
                    .unwrap(),
            )
            .unwrap();

        // Misc
        lua.globals()
            .set(
                "exit",
                lua.create_function(|_, _: ()| {
                    api::misc::exit();
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "save",
                lua.create_function(|_, _: ()| Ok(api::misc::save()))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "load",
                lua.create_function(|_, str: String| {
                    api::misc::load(str);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "unload",
                lua.create_function(|_, _: ()| {
                    api::misc::unload();
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        // Lua specific string api

        lua.globals()
            .set(
                "sub",
                lua.create_function(|_, (str, start, end): (String, Option<u32>, Option<u32>)| {
                    Ok(
                        str[start.unwrap_or(0) as usize..end.unwrap_or(str.len() as u32) as usize]
                            .to_string(),
                    )
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "tostr",
                lua.create_function(|_, (val, hex): (f32, Option<bool>)| {
                    let hex = hex.unwrap_or(false);
                    if hex {
                        todo!();
                    } else {
                        Ok(val.to_string())
                    }
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "tonum",
                lua.create_function(|_, val: String| Ok(val.parse::<f32>().unwrap_or(0.0)))
                    .unwrap(),
            )
            .unwrap();

        // Lua Table function
        lua.globals()
            .set(
                "add",
                lua.create_function(|_, (table, value): (Table, Value)| {
                    let mut idx = table.len().unwrap();
                    while table.contains_key(idx).unwrap() {
                        idx += 1;
                    }
                    table.set(idx, value).unwrap();
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "del",
                lua.create_function(|_, (table, value): (Table, Value)| {
                    for idx in 0..table.len().unwrap() {
                        if table.contains_key(idx).unwrap() {
                            let val: Value = table.get(idx).unwrap();
                            if val == value {
                                return Ok(val);
                            }
                        }
                    }
                    Ok(Value::Nil)
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "all",
                lua.create_function(|lua, table: Table| {
                    Ok(MultiValue::from_vec(vec![
                        Value::Function(
                            lua.create_function(|_, (tbl, i): (Table, i32)| {
                                let i = i + 1;
                                if tbl.contains_key(i).unwrap_or(false) {
                                    Ok(tbl.get::<i32, Value>(i).unwrap())
                                } else {
                                    Ok(Value::Nil)
                                }
                            })
                            .unwrap(),
                        ),
                        Value::Table(table),
                        Value::Integer(0),
                    ]))
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "count",
                lua.create_function(|_, table: Table| Ok(table.len().unwrap()))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "foreach",
                lua.create_function(|_, (table, function): (Table, Function)| {
                    for idx in 0..table.len().unwrap() {
                        if table.contains_key(idx).unwrap() {
                            function.call::<Value, ()>(table.get(idx).unwrap()).unwrap();
                        }
                    }
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        let x = script.iter().map(|f| *f as char).collect::<String>();
        std::fs::write("./last.lua", x);

        lua.load(script).exec().unwrap();

        LuaRuntime { lua }
    }
}

impl Runtime for LuaRuntime {
    fn init(&mut self) {
        match self.lua.globals().get::<&str, Function>("_init") {
            Ok(func) => func,
            Err(_) => panic!("Missing _init function"),
        }
        .call::<_, ()>(())
        .unwrap();
    }

    fn update(&mut self) {
        match self.lua.globals().get::<&str, Function>("_update") {
            Ok(func) => func,
            Err(_) => panic!("Missing _update function"),
        }
        .call::<_, ()>(())
        .unwrap();
    }

    fn draw(&mut self) {
        match self.lua.globals().get::<&str, Function>("_draw") {
            Ok(func) => func,
            Err(_) => panic!("Missing _draw function"),
        }
        .call::<_, ()>(())
        .unwrap();
    }
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut negative: Vec<i32> = Vec::new();
    let mut positive: Vec<i32> = Vec::new();
    nums.iter().for_each(|x| {
        if *x < 0 {
            negative.push(x.pow(2));
        } else {
            positive.push(x.pow(2));
        }
    });

    negative.reverse();
    negative.append(&mut positive);
    negative
}

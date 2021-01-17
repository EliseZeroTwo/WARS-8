use std::{fs, io::{Cursor, Read, Write}};
use regex::Regex;

use crate::{MEM, palette::ColorPalette, runtime::lua_runtime::LuaRuntime, set_map, set_sprite};

use super::Cart;

pub struct P8Script {
    path: String,
    name: String,
    script: Vec<u8>,
    sprites: Vec<[[ColorPalette; 8]; 8]>,
    spritesheet: [i32; 128*128],
    map: [i32; 128*64],
}

impl P8Script {
    pub fn new(path: &String) -> Self {
        let metadata = match fs::metadata(&path) {
            Ok(md) => md,
            Err(why) => panic!("Unable to open file {}, reason: {}", path, why),
        };

        if !metadata.is_file() {
            panic!("{} is not a file!", path);
        }

        if metadata.len() >= u32::MAX as u64 {
            panic!(
                "{} is {} bytes too big! (total: {}, limit {})",
                path,
                metadata.len() - u32::MAX as u64,
                metadata.len(),
                u32::MAX
            );
        }

        let name: String = match path.rfind('/') {
            Some(pos) => (path[(pos + 1)..]).to_string(),
            None => path.clone(),
        };

        let data = match fs::read_to_string(&path) {
            Ok(bs) => bs,
            Err(why) => panic!("Unable to read {}, reason: {}", &path, why),
        };

        let lines = data.split('\n').collect::<Vec<&str>>();

        let mut lua_start = -1;
        let mut lua_end = lines.len() as i32;
        
        let mut gfx_start = -1;
        let mut gfx_end = lines.len() as i32;
        
        let mut gff_start = -1;
        let mut gff_end = lines.len() as i32;
        
        let mut map_start = -1;
        let mut map_end = lines.len() as i32;



        let p8_cart_sec_regex = Regex::new(r"__[a-zA-Z]*__$").unwrap();

        for x in 0..lines.len() {
            if lua_start != -1 && p8_cart_sec_regex.is_match(lines[x]) {
                lua_end = x as i32;
                break;
            }

            if lines[x] == "__lua__" {
                lua_start = x as i32 + 1;
            }
        }

        for x in 0..lines.len() {
            if gfx_start != -1 && p8_cart_sec_regex.is_match(lines[x]) {
                gfx_end = x as i32;
                break;
            }

            if lines[x] == "__gfx__" {
                gfx_start = x as i32 + 1;
            }
        }

        for x in 0..lines.len() {
            if map_start != -1 && p8_cart_sec_regex.is_match(lines[x]) {
                map_end = x as i32;
                break;
            }

            if lines[x] == "__map__" {
                map_start = x as i32 + 1;
            }
        }

        for x in 0..lines.len() {
            if gff_start != -1 && p8_cart_sec_regex.is_match(lines[x]) {
                gff_end = x as i32;
                break;
            }

            if lines[x] == "__gff__" {
                gff_start = x as i32 + 1;
            }
        }

        let if_return_regex = Regex::new(r"if\s*\([^)]*\)\s*return").unwrap();
        let plus_equals_regex = Regex::new(r"(\S+)\s*\+=").unwrap();
        let minus_equals_regex = Regex::new(r"(\S+)\s*-=").unwrap();
        let mul_equals_regex = Regex::new(r"(\S+)\s*\*=").unwrap();
        let div_equals_regex = Regex::new(r"(\S+)\s*/=").unwrap();
        let mod_equals_regex = Regex::new(r"(\S+)\s*%=").unwrap();
        let pow_equals_regex = Regex::new(r"(\S+)\s*\^=").unwrap();
        let mut script: Vec<u8> = Vec::new();
        for x in lua_start..lua_end {
            let mut line = lines[x as usize].to_owned();
            let mut line_clone = line.clone();
            
            for x in plus_equals_regex.captures_iter(&line_clone) {
                line = line.replace("+=",format!("= {} +", &x[1]).as_str());
            }

            for x in minus_equals_regex.captures_iter(&line_clone) {
                line = line.replace("-=",format!("= {} -", &x[1]).as_str());
            }

            for x in mul_equals_regex.captures_iter(&line_clone) {
                line = line.replace("*=",format!("= {} *", &x[1]).as_str());
            }

            for x in div_equals_regex.captures_iter(&line_clone) {
                line = line.replace("/=",format!("= {} /", &x[1]).as_str());
            }

            for x in mod_equals_regex.captures_iter(&line_clone) {
                line = line.replace("%=",format!("= {} %", &x[1]).as_str());
            }

            for x in pow_equals_regex.captures_iter(&line_clone) {
                line = line.replace("^=",format!("= {} ^", &x[1]).as_str());
            }

            if if_return_regex.is_match(&line_clone) {
                line = line.replace("return", "then\nreturn\nend");
            }
            
            line = line.replace("!=", "~=");

            for ch in line.as_bytes() {
                script.push(*ch);
            }
            script.push('\n' as u8);
        }

        let mut mem_lock = MEM.lock().unwrap();

        let mut spritesheet = [0; 128*128];
        let mut sprites: Vec<[[ColorPalette; 8]; 8]> = Vec::new();
        if gfx_start != -1 {
            for x in 0..(gfx_end - gfx_start) {
                let mut ctr = 0;
                for ch in lines[(gfx_start + x) as usize].as_bytes() {
                    spritesheet[((x * 128) + ctr) as usize] = (*ch as char).to_digit(16).unwrap() as i32;
                    ctr += 1;
                }
            }

            
            let amnt = 16 * ((gfx_end - gfx_start) / 8);
            for x in 0..amnt {
                sprites.push([[ColorPalette::Black; 8]; 8]);
                let base = (8 * (x % 16)) + ((8 * 128) * (x / 16));
                for row in 0..8 {
                    for col in 0..8 {
                        let color = ColorPalette::from(spritesheet[(base + col + (row * 128)) as usize]);
                        sprites[x as usize][row as usize][col as usize] = color;
                    }
                }
            }
        }

        for x in 0..sprites.len() {
            set_sprite(Some(&mut mem_lock), x as i32, sprites[x]);
        }

        let mut map = [0; (128 * 64)];
        if map_start != -1 {
            for idx in 0..(map_end-map_start) {
                let mut ctr = 0;
                for ch in lines[(map_start + idx) as usize].as_bytes() {
                    let val = (*ch as char).to_digit(16).unwrap() as u8;
                    map[((idx * 128) + ctr) as usize] = val as i32;
                    set_map(Some(&mut mem_lock), ctr, idx, val);
                    ctr += 1;
                }
            }
        }

        P8Script {
            path: path.clone(),
            name,
            script,
            sprites,
            spritesheet,
            map,
        }
    }
}

impl Cart for P8Script {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> u32 {
        self.script.len() as u32
    }

    fn binary(&self) -> &[u8] {
        &self.script[..]
    }

    fn save(&self) -> Result<(), ()> {
        match std::fs::write(&self.path, &self.script) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    fn create_runtime(&self) -> Box<dyn crate::runtime::Runtime> {
        Box::new(LuaRuntime::new(&self.script[..]))
    }
}

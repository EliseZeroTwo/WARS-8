use directories::ProjectDirs;
use sdl2::keyboard::Scancode;
use serde::{Deserialize, Serialize};
use std::fs;

// Lin: /home/elise/.config/WARS-8/config.json
// Win: C:\Users\elise\AppData\Roaming\headpat\WARS-8\config\config.json
// Mac: /Users/elise/Library/Application Support/services.headpat.WARS-8/config.json

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerKeyBindings {
    pub left: i32,
    pub right: i32,
    pub up: i32,
    pub down: i32,
    pub o: i32,
    pub x: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings {
    pub player1: PlayerKeyBindings,
    pub player2: PlayerKeyBindings,
    pub quit: i32,
    pub pause: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoSettings {
    pub fullscreen: bool,
    pub borderless: bool,
    pub fps: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub video: VideoSettings,
    pub keys: KeyBindings,
}

impl Config {
    pub fn new() -> Config {
        Config {
            video: VideoSettings {
                fullscreen: false,
                borderless: false,
                fps: true,
            },
            keys: KeyBindings {
                player1: PlayerKeyBindings {
                    left: Scancode::Left as i32,
                    right: Scancode::Right as i32,
                    up: Scancode::Up as i32,
                    down: Scancode::Down as i32,
                    o: Scancode::Z as i32,
                    x: Scancode::X as i32,
                },
                player2: PlayerKeyBindings {
                    left: Scancode::S as i32,
                    right: Scancode::F as i32,
                    up: Scancode::E as i32,
                    down: Scancode::D as i32,
                    o: Scancode::Tab as i32,
                    x: Scancode::Q as i32,
                },
                quit: Scancode::Escape as i32,
                pause: Scancode::Grave as i32,
            },
        }
    }

    pub fn get_config_dir_or_create() -> Option<String> {
        match ProjectDirs::from("services", "headpat", "WARS-8") {
            Some(proj_dirs) => {
                let config_dir = proj_dirs.config_dir();
                if !config_dir.exists() {
                    if let Err(why) = fs::create_dir_all(config_dir) {
                        panic!("Not able to create config directory! Reason: {}", why);
                    }
                }

                match config_dir.to_str() {
                    Some(path) => Some(path.to_string()),
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn get_config_or_create() -> Config {
        let mut config: Config = Config::new();
        let path_str = match Config::get_config_dir_or_create() {
            Some(ps) => ps,
            None => "./wars8".to_string(),
        };

        let path = std::path::Path::new(&path_str);

        let path = path.join("config.json");
        if !path.exists() {
            if let Err(why) = fs::write(path, serde_json::to_string_pretty(&config).unwrap()) {
                panic!("Not able to create config! Reason: {}", why);
            }
        } else {
            match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(cfg) => config = cfg,
                    Err(_) => {
                        fs::remove_file(&path).unwrap();
                        if let Err(why) =
                            fs::write(&path, serde_json::to_string_pretty(&config).unwrap())
                        {
                            panic!("Not able to create config! Reason: {}", why);
                        }
                    }
                },
                Err(why) => panic!("Unable to read config file! Reason: {}", why),
            }
        }
        config
    }

    pub fn save_config(&self) {
        if let Some(proj_dirs) = ProjectDirs::from("services", "headpat", "WARS-8") {
            let path = proj_dirs.config_dir();
            if !path.exists() {
                if let Err(why) = fs::create_dir_all(path) {
                    panic!("Not able to create config directory! Reason: {}", why);
                }
            }

            let path = path.join("config.json");
            fs::remove_file(&path).unwrap();

            if let Err(why) = fs::write(path, serde_json::to_string_pretty(self).unwrap()) {
                panic!("Not able to create config! Reason: {}", why);
            }
        }
    }
}

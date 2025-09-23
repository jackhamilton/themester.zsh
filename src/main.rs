use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use rand::SeedableRng;
use std::env;
use rand::seq::IndexedRandom;
use rand::rngs::StdRng;
use freezable_trait::freezable;

#[derive(Clone, Debug)]
#[freezable]
struct Config {
    cache_file_location: String,
    term_env_var_name: String,
    nvim_theme_env_var_name: String,
    nvim_plugin_env_var_name: String,
    write_term_lua: bool,
    term_lua_path: String,
    theme: Vec<Theme>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_file_location: "~/.config/themester/.themecache".into(),
            term_env_var_name: "TERM_THEME".into(),
            nvim_theme_env_var_name: "NVIM_THEME".into(),
            nvim_plugin_env_var_name: "NVIM_THEME_PLUGIN".into(),
            write_term_lua: true,
            term_lua_path: "~/.config/wezterm/dynamic_theme.lua".into(),
            theme: vec![
                Theme::default()
            ]
        }
    }
}

#[derive(Clone, Debug)]
#[freezable]
struct Theme {
    term: String,
    nvim_plugin: String,
    nvim_themename: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            term: "plugin-name".into(),
            nvim_plugin: "nvim-plugin-name".into(),
            nvim_themename: "nvim-plugin-colorscheme".into()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments! Try -h for help.");
    }
    let arg = args[1].clone();
    match arg.as_str() {
        "-h" => help(),
        "-r" => randomize_theme(),
        "-l" => load_cache(),
        _ => help(),
    }
}

fn help() {
    println!("Arguments:\n\t-h for help (this)\n\teval $(themester -r) to randomize your theme\
        \n\teval $(themester -l) in your .zshrc to load the last session's theme environment variables");
    std::process::exit(0);
}

fn load_cache() {
    let config: Config = toml_configurator::get_config("themester".into());
    let expanded = shellexpand::tilde(&config.cache_file_location).into_owned().to_string();
    let file_path = Path::new(&expanded);
    let cache_contents = fs::read_to_string(file_path).expect("echo \"Could not read cache file!\"");

    println!("{cache_contents}");
}

fn randomize_theme() {
    let config: Config = toml_configurator::get_config("themester".into());

    let export_str = randomize(&config);
    let expanded = shellexpand::tilde(&config.cache_file_location).into_owned().to_string();
    let file_path = Path::new(&expanded);
    match OpenOptions::new().create(true).truncate(true).write(true).open(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }.unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    fs::write(file_path, export_str.clone()).expect("echo \"Unable to write theme lua file.\"");

    println!("{export_str}");
}

fn randomize(config: &Config) -> String {
    let retries = 10;
    let mut export_string = String::new();
    if config.theme.is_empty() {
        println!("echo \"Error: no themes!\"");
        return String::new();
    };
    let mut rng = StdRng::from_os_rng();
    if let Some(existing) = env::var_os(config.term_env_var_name.clone()) {
        for _ in 0..retries {
            if let Some(chosen) = config.theme.iter().map(|item| item.term.clone()).collect::<Vec<String>>().choose(&mut rng) {
                if **chosen != *existing {
                    export_string = format!("{}export {}=\"{}\"", export_string, config.term_env_var_name, chosen);
                    write_term_themefile_lua(config.write_term_lua, config.term_lua_path.clone(), String::from(chosen));
                    if let Some(nvim) = config.theme.iter().find(|item| item.term == *chosen) {
                        export_string = format!("{}\nexport {}=\"{}\"", export_string, config.nvim_theme_env_var_name, nvim.nvim_themename);
                        export_string = format!("{}\nexport {}=\"{}\"", export_string, config.nvim_plugin_env_var_name, nvim.nvim_plugin);
                    }
                    return export_string;
                }
            }
        }
    } else if let Some(chosen) = config.theme.iter().map(|item| item.term.clone()).collect::<Vec<String>>().choose(&mut rng) {
        export_string = format!("{}export {}={}", export_string, config.term_env_var_name, chosen);
        write_term_themefile_lua(config.write_term_lua, config.term_lua_path.clone(), String::from(chosen));
        if let Some(nvim) = config.theme.iter().find(|item| item.term == *chosen) {
            export_string = format!("{}\nexport {}=\"{}\"", export_string, config.nvim_theme_env_var_name, nvim.nvim_themename);
            export_string = format!("{}\nexport {}=\"{}\"", export_string, config.nvim_plugin_env_var_name, nvim.nvim_plugin);
            return export_string;
        }
    }
    String::new()
}

fn write_term_themefile_lua(write: bool, path: String, key: String) {
    if write {
        let expanded = shellexpand::tilde(&path).into_owned().to_string();
        let file_path = Path::new(&expanded);
        fs::write(file_path, format!("return \"{key}\"")).expect("echo \"Unable to write theme lua file.\"")
    }
}

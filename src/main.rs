use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use rand::SeedableRng;
use toml::Table;
use std::env;
use rand::seq::IndexedRandom;
use rand::rngs::StdRng;

#[derive(Clone)]
struct Config {
    cache_file_location: Option<String>,
    term_env_var_name: Option<String>,
    nvim_theme_env_var_name: Option<String>,
    nvim_plugin_env_var_name: Option<String>,
    write_term_lua: Option<bool>,
    term_lua_path: Option<String>,
    themes: Vec<Theme>,
}

#[derive(Clone)]
struct Theme {
    term: String,
    nvim_plugin: String,
    nvim_themename: String,
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
    setup_config_file();
    let config = load_config();

    if let Some(cache_path) = config.cache_file_location {
        let expanded = shellexpand::tilde(&cache_path).into_owned().to_string();
        let file_path = Path::new(&expanded);
        let cache_contents = fs::read_to_string(file_path).expect("echo \"Could not read cache file!\"");
        println!("{}", cache_contents);
    }
}

fn randomize_theme() {
    setup_config_file();
    let config = load_config();
    let copy = config.clone();

    let export_str = randomize(copy);
    if let Some(cache_path) = config.cache_file_location {
        let expanded = shellexpand::tilde(&cache_path).into_owned().to_string();
        let file_path = Path::new(&expanded);
        match OpenOptions::new().create(true).truncate(true).write(true).open(file_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }.unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        fs::write(file_path, export_str.clone()).expect("echo \"Unable to write theme lua file.\"")
    }
    println!("{}", export_str);
}

fn randomize(config: Config) -> String {
    let retries = 10;
    let mut export_string = String::new();
    if let Some(term_env) = config.term_env_var_name {
        if config.themes.is_empty() {
            println!("echo \"Error: no themes!\"");
            return String::new();
        };
        let mut rng = StdRng::from_os_rng();
        if let Some(existing) = env::var_os(term_env.clone()) {
            for _ in 0..retries {
                if let Some(chosen) = config.themes.iter().map(|item| item.term.clone()).collect::<Vec<String>>().choose(&mut rng) {
                    if **chosen != *existing {
                        export_string = format!("{}export {}=\"{}\"", export_string, term_env, chosen);
                        write_term_themefile_lua(config.write_term_lua, config.term_lua_path, String::from(chosen));
                        if let Some(nvim) = config.themes.iter().find(|item| item.term == *chosen) {
                            if let Some(nvim_env) = config.nvim_theme_env_var_name {
                                if let Some(nvim_plug) = config.nvim_plugin_env_var_name {
                                    export_string = format!("{}\nexport {}=\"{}\"", export_string, nvim_env, nvim.nvim_themename);
                                    export_string = format!("{}\nexport {}=\"{}\"", export_string, nvim_plug, nvim.nvim_plugin);
                                }
                            }
                        }
                        return export_string;
                    }
                }
            }
        } else if let Some(chosen) = config.themes.iter().map(|item| item.term.clone()).collect::<Vec<String>>().choose(&mut rng) {
            export_string = format!("{}export {}={}", export_string, term_env, chosen);
            write_term_themefile_lua(config.write_term_lua, config.term_lua_path, String::from(chosen));
            if let Some(nvim) = config.themes.iter().find(|item| item.term == *chosen) {
                if let Some(nvim_env) = config.nvim_theme_env_var_name {
                    if let Some(nvim_plug) = config.nvim_plugin_env_var_name {
                        export_string = format!("{}\nexport {}=\"{}\"", export_string, nvim_env, nvim.nvim_themename);
                        export_string = format!("{}\nexport {}=\"{}\"", export_string, nvim_plug, nvim.nvim_plugin);
                        return export_string;
                    }
                }
            }
        }
    }
    return String::new();
}

fn write_term_themefile_lua(write: Option<bool>, path: Option<String>, key: String) {
    if let Some(should_write) = write {
        if let Some(some_path) = path {
            if should_write {
                let expanded = shellexpand::tilde(&some_path).into_owned().to_string();
                let file_path = Path::new(&expanded);
                fs::write(file_path, format!("return \"{}\"", key)).expect("echo \"Unable to write theme lua file.\"")
            }
        }
    }
}

fn setup_config_file() {
    let dir_path_string = shellexpand::tilde("~/.config/themester/").into_owned().to_string();
    let dir_path = Path::new(&dir_path_string);
    let dir_exists = fs::metadata(dir_path).is_ok();
    let config_path_string = shellexpand::tilde("~/.config/themester/config.toml").into_owned().to_string();
    let config_path = Path::new(&config_path_string);
    let config_file_exists = fs::metadata(config_path).is_ok();
    if !dir_exists {
        if let Err(why) = fs::create_dir(dir_path) {
            println!("! {:?}", why.kind());
        }
    }
    if !config_file_exists {
        touch(config_path).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        let default_config = "cache_file_location = \"~/.config/themester/.themecache\"\n\n\
            term_env_var_name = \"TERM_THEME\"\n\
            nvim_plugin_env_var_name = \"NVIM_THEME_PLUGIN\"\n\
            nvim_theme_env_var_name = \"NVIM_THEME\"\n\n\
            # write_term_lua = true\n\
            # term_lua_path = \"~/.config/wezterm/dynamic_theme.lua\"\n\n\
            [[theme]]\n\
            term = \"nordfox\"\n\
            nvim_plugin = \"nordic\"\n\
            nvim_themename = \"nordic\"\n\n\
            [[theme]]\n\
            term = \"tokyonight-night\"\n\
            nvim_plugin = \"tokyonight\"\
            nvim_themename = \"tokyonight-night\"\n";
        fs::write(config_path, default_config).expect("echo \"Unable to write config file.\"")
    }
}

fn load_config() -> Config {
    let config_path_string = shellexpand::tilde("~/.config/themester/config.toml").into_owned().to_string();
    let config_path = Path::new(&config_path_string);
    let config_contents = fs::read_to_string(config_path).expect("echo \"Could not read config.toml!\"");
    let config = config_contents.parse::<Table>().expect("echo \"Could not parse config.toml!\"");
    let mut write_term_lua: Option<bool> = None;
    if config.contains_key("write_term_lua") {
        write_term_lua = config["write_term_lua"].as_bool();
    }
    let mut cache_file_location: Option<String> = None;
    if config.contains_key("cache_file_location") {
        let cache_file_location_str = config["cache_file_location"].as_str();
        if let Some(unwrap) = cache_file_location_str {
            cache_file_location = Some(unwrap.to_string());
        }
    }
    let mut term_lua_path: Option<String> = None;
    if config.contains_key("term_lua_path") {
        let term_lua_path_str = config["term_lua_path"].as_str();
        if let Some(unwrap) = term_lua_path_str {
            term_lua_path = Some(unwrap.to_string());
        }
    }
    let mut term_env_var_name: Option<String> = None;
    if config.contains_key("term_env_var_name") {
        let term_env_var_name_str = config["term_env_var_name"].as_str();
        if let Some(unwrap) = term_env_var_name_str {
            term_env_var_name = Some(unwrap.to_string());
        }
    }
    let mut nvim_theme_env_var_name: Option<String> = None;
    if config.contains_key("nvim_theme_env_var_name") {
        let nvim_theme_env_var_name_str = config["nvim_theme_env_var_name"].as_str();
        if let Some(unwrap) = nvim_theme_env_var_name_str {
            nvim_theme_env_var_name = Some(unwrap.to_string());
        }
    }
    let mut nvim_plugin_env_var_name: Option<String> = None;
    if config.contains_key("nvim_plugin_env_var_name") {
        let nvim_plugin_env_var_name_str = config["nvim_plugin_env_var_name"].as_str();
        if let Some(unwrap) = nvim_plugin_env_var_name_str {
            nvim_plugin_env_var_name = Some(unwrap.to_string());
        }
    }
    let themes = config["theme"].as_array().expect("echo \"No themes found!\"");
    let mut themes_arr = Vec::new();
    for theme in themes {
        if let Some(theme_table) = theme.as_table() {
            if let Some(term) = theme_table["term"].as_str() {
                if let Some(nvim_plugin) = theme_table["nvim_plugin"].as_str() {
                if let Some(nvim_theme) = theme_table["nvim_themename"].as_str() {
                    let theme = Theme {
                        term: term.to_string(),
                        nvim_plugin: nvim_plugin.to_string(),
                        nvim_themename: nvim_theme.to_string(),
                    };
                    themes_arr.push(theme);
                }
                }
            }
        } else {
            println!("echo \"Invalid table found in theme configuration!\"")
        }
    }
    Config {
        cache_file_location,
        term_env_var_name,
        nvim_theme_env_var_name,
        nvim_plugin_env_var_name,
        write_term_lua,
        term_lua_path,
        themes: themes_arr,
    }
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).truncate(false).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

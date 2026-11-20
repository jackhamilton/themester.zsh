mod caching;
mod models;
mod randomize;
mod write;

use cli_builder_macros::cli_builder;
use std::env;
use toml_configurator::configurator_macros::config_builder;

use crate::caching::load_cache;
use crate::models::{LocalData, Theme};

config_builder! {
    cache_file_location: String = "~/.config/themester/.themecache",
    // Zero means it will never randomize
    hours_per_randomization: i32 = 0,
    theme: Vec<Theme> = vec![Theme::default()],

    lua_configured_terminals {
        write_term_lua: bool = false,
        term_lua_path: String = "~/.config/wezterm/dynamic_theme.lua",
    },

    ghostty {
        write_ghostty_config: bool = false,
        ghostty_config_path: String = "~/.config/themester/ghostty.conf",
    },

    environment {
        term_env_var_name: String = "TERM_THEME",
        nvim_theme_env_var_name: String = "NVIM_THEME",
        nvim_plugin_env_var_name: String = "NVIM_THEME_PLUGIN",
    },
}

cli_builder! {
    default = load_cache;
    commands = [
        CLICommand {
            short_keyword: Some("l"),
            keyword: "load",
            description: "Loads the last theme",
            command: load_cache
        },
        CLICommand {
            short_keyword: Some("r"),
            keyword: "randomize",
            description: "Randomize the current theme",
            command: randomize
        }
    ];
    flags = [];
}

fn randomize(_runtime_options: &Vec<CLIFlagType>) {
    randomize::randomize_theme(Some(CONFIG.clone()));
}

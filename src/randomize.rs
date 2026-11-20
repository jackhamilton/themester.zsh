use std::env;
use std::fs::{self, OpenOptions};
use std::path::Path;

use freezable_trait::Freezable;
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;

use crate::write::write_terminal_theme_files;
use crate::{Config, LocalData};

pub fn randomize_theme(config: Option<Config>) {
    let config: Config = config
        .or_else(|| toml_configurator::get_config("themester".into()))
        .expect("Could not resolve config");

    let export_str = randomize(&config);
    let expanded = shellexpand::tilde(&config.cache_file_location)
        .into_owned()
        .to_string();
    let file_path = Path::new(&expanded);
    match OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file_path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
    .unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    fs::write(file_path, export_str.clone()).expect("echo \"Unable to write theme lua file.\"");

    let mut data = LocalData::default();
    data.last_randomized = chrono::offset::Utc::now();
    data.write_to_file_str("~/.config/themester/.localdata");

    println!("{export_str}");
}

pub fn randomize(config: &Config) -> String {
    let retries = 10;
    let mut export_string = String::new();
    if config.theme.is_empty() {
        println!("echo \"Error: no themes!\"");
        return String::new();
    };
    let mut rng: StdRng = rand::make_rng();
    if let Some(existing) = env::var_os(config.environment.term_env_var_name.clone()) {
        for _ in 0..retries {
            if let Some(chosen) = config
                .theme
                .iter()
                .map(|item| item.term.clone())
                .collect::<Vec<String>>()
                .choose(&mut rng)
                && **chosen != *existing
            {
                export_string = format!(
                    "{}export {}=\"{}\"",
                    export_string, config.environment.term_env_var_name, chosen
                );
                write_terminal_theme_files(config, chosen);
                if let Some(nvim) = config.theme.iter().find(|item| item.term == *chosen) {
                    export_string = format!(
                        "{}\nexport {}=\"{}\"",
                        export_string, config.environment.nvim_theme_env_var_name, nvim.nvim_themename
                    );
                    if let Some(plug) = &nvim.nvim_plugin {
                        export_string = format!(
                            "{}\nexport {}=\"{}\"",
                            export_string, config.environment.nvim_plugin_env_var_name, plug
                        );
                    } else {
                        export_string = format!(
                            "{}\nexport {}=\"none\"",
                            export_string, config.environment.nvim_plugin_env_var_name
                        );
                    }
                }
                return export_string;
            }
        }
    } else if let Some(chosen) = config
        .theme
        .iter()
        .map(|item| item.term.clone())
        .collect::<Vec<String>>()
        .choose(&mut rng)
    {
        export_string = format!(
            "{}export {}=\"{}\"",
            export_string, config.environment.term_env_var_name, chosen
        );
        write_terminal_theme_files(config, chosen);
        if let Some(nvim) = config.theme.iter().find(|item| item.term == *chosen) {
            export_string = format!(
                "{}\nexport {}=\"{}\"",
                export_string, config.environment.nvim_theme_env_var_name, nvim.nvim_themename
            );
            if let Some(plug) = &nvim.nvim_plugin {
                export_string = format!(
                    "{}\nexport {}=\"{}\"",
                    export_string, config.environment.nvim_plugin_env_var_name, plug
                );
            } else {
                export_string = format!(
                    "{}\nexport {}=\"none\"",
                    export_string, config.environment.nvim_plugin_env_var_name
                );
            }
            return export_string;
        }
    }
    String::new()
}

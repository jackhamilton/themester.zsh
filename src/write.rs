use std::{fs, path::Path};

use crate::Config;

pub fn write_terminal_theme_files(config: &Config, term_theme: &str) {
    if config.lua_configured_terminals.write_term_lua {
        let expanded = shellexpand::tilde(&config.lua_configured_terminals.term_lua_path).into_owned().to_string();
        let file_path = Path::new(&expanded);
        fs::write(file_path, format!("return \"{term_theme}\""))
            .expect("echo \"Unable to write theme lua file.\"")
    }

    if config.ghostty.write_ghostty_config {
        let ghostty_theme = config
            .theme
            .iter()
            .find(|theme| theme.term == term_theme)
            .and_then(|theme| theme.ghostty_theme.as_deref());
        let contents = ghostty_theme
            .map(|theme| format!("theme = {theme}\n"))
            .unwrap_or_default();
        let expanded = shellexpand::tilde(&config.ghostty.ghostty_config_path)
            .into_owned()
            .to_string();
        fs::write(Path::new(&expanded), contents).expect("Unable to write Ghostty theme config file.");
    }
}

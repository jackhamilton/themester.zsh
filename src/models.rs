use chrono::{DateTime, Utc};
use freezable_trait::freezable;

#[derive(Debug)]
#[freezable]
pub struct Theme {
    #[default("plugin-name")]
    pub term: String,

    pub ghostty_theme: Option<String>,
    pub nvim_plugin: Option<String>,

    #[default("nvim-plugin-colorscheme")]
    pub nvim_themename: String,
}

#[freezable]
pub struct LocalData {
    pub last_randomized: DateTime<Utc>,
}

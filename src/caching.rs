use std::{fs, path::Path};

use chrono::TimeDelta;
use freezable_trait::Freezable;

use crate::{CLIFlagType, Config, LocalData, randomize::randomize_theme};

pub fn load_cache(_runtime_args: &Vec<CLIFlagType>) {
    let config: Config = toml_configurator::get_config("themester".into());

    let local_data = LocalData::from_file_str("~/.config/themester/.localdata");
    if config.hours_per_randomization > 0
        && let Some(delta) = TimeDelta::new((config.hours_per_randomization * 3600).into(), 0)
        && let Some(cutoff) = local_data.last_randomized.checked_add_signed(delta)
        && cutoff < chrono::offset::Utc::now()
    {
        randomize_theme(Some(config.clone()));
        return;
    }

    let expanded = shellexpand::tilde(&config.cache_file_location)
        .into_owned()
        .to_string();
    let file_path = Path::new(&expanded);
    let cache_contents =
        fs::read_to_string(file_path).expect("echo \"Could not read cache file!\"");

    println!("{cache_contents}");
}

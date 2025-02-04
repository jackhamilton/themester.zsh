# themester.zsh


Integrate your nvim and terminal theme, and randomize them between a preselected list of options. Publishes a .lua file containing a theme string for your terminal, and publishes environment variables which can be read by either your terminal emulator or anything else you'd like to theme.

https://github.com/user-attachments/assets/eb0adf04-4138-4a9a-9af5-79794c6c7a6a


Designed specifically to integrate wezterm and nvim's themes, though it can be used generally.

Pair with themester.nvim for a prebaked nvim integration.

## Setup

Copy the themester release binary to any location you can add to your PATH, then add it to your path.
For example:

```zsh
sudo cp target/release/themester /usr/local/bin/
#If /usr/local/bin is not already in your path. You could also put this in .zshenv or any file sourced form your zshrc.
echo "\nexport PATH=\"$PATH:/usr/local/bin/" > ~/.zshrc
```

Add a line sourcing the themester cache file to your .zshrc:

```zsh
echo "\neval $(themester -l)" > ~/.zshrc
```

Then optionally add an alias for eval $(themester -r), which shuffles your theme by exporting
new environment variables and writing the theme lua file if that setting is enabled.

```zsh
echo "\nalias theme=\"eval $(themester -r)\"" > ~/.zshrc
```

Then, run themester -r and setup the config file it generates at ~/.config/themester as described in the config format section of this readme.

## How it works

Running

```zsh
themester -r
```

outputs bash code to set two environment variables, the names of which can be set in your config.toml (described below), to the name of a random theme picked from your config. Piping this to eval with

```zsh
eval $(themester -r)
```

will set those env variables in your current session, enabling for example nvim instances started in that session with themester.nvim configured to access the randomized theme name stored in the environment variable. It also writes the theme lua file if that option is on.

```zsh
eval $(themester -l)
```

simply outputs the last randomization, pulled from the cache file. Hence, if you run that at zsh startup, you'll always have local environment variables reflecting the last theme shuffle. Note: does not write the theme file.

## Config format

A config will be generated for you at ~/.config/themester if one is not present the first time you run "themester -r".

```toml
cache_file_location = "~/.config/themester/.themecache"
```

This is the location at which to store the themester cache file. It can be anywhere that the process will have write permissions for.

```toml
term_env_var_name = "TERM_THEME"
nvim_plugin_env_var_name = "NVIM_THEME_PLUGIN"
nvim_theme_env_var_name = "NVIM_THEME"
```

These describe the name to use for the environment variables we're storing theme names in.

```toml
write_term_lua = true
term_lua_path = "~/.config/wezterm/dynamic_theme.lua"
```

themester.zsh has the ability to write a lua file with the provided name containing a return "[TERM_THEME]" statement, which can then be read by terminal emulators like wezterm. Wezterm particularly will reload its config if any files it sources are updated, which means that's all you need to do in order for that integration to work.

```lua
-- This could go in your wezterm conf if using the default generated file name in order to update the wezterm theme when themester triggers.
local theme = require('dynamic_theme')
config.color_scheme = "" .. theme
```
^ An example wezterm config using this functionality.

```toml
[[theme]]
term = "tokyonight-night"
nvim_plugin = "tokyonight"
nvim_themename = "tokyonight-night"

[[theme]]
term = "toykonight_moon"
nvim_plugin = "tokyonight"
nvim_themename = "tokyonight-moon"
```

Finally, these blocks (you can have as many as you like, but must have at least one) describe the themes to randomize between when themester -r is called. You will still need to install these themes in your terminal emulator and in nvim. "term" is the theme name to set in your terminal config, "nvim_plugin" is the plugin that contains the theme you want for nvim (themester.nvim will call require("theme").setup() for you automatically, so the theme can be lazy loaded if it doesn't require priority startup like gruvbox), and "nvim_themename" is the colorscheme name to set. These will be spat out to the corresponding environment variables set in the config above.

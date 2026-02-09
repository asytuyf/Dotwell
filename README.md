# Dotwell

> still in development 🚧

a TUI for managing dotfiles and rice themes. switch between configs with one keystroke.

## what it does

browse your dotfiles, preview them, install them. press `i` and boom - new theme applied.

built this for managing my NixOS rices but works with anything.

## install

```bash
git clone https://github.com/asytuyf/dotwell.git
cd dotwell
cargo install --path .
```

now you can run `dwell` from anywhere.

## usage

```bash
dwell           # launch TUI
dwell --list    # list all themes
```

**keys:**
- `b` browse
- `j/k` navigate
- `i` install
- `q` quit

## how it works

add a `dotwell.toml` to your dotfiles:

```toml
name = "htop blue theme"
description = "blue colors for htop"
category = "themes"
dependencies = ["htop"]
files = ["htoprc", "install.sh"]

[compiler]
type = "make"
```

dotwell finds them automatically in `~/.config`, `~/dotfiles`, or `/etc/nixos`.

## example

```
~/dotfiles/htop-themes/
├── blue/
│   ├── htoprc
│   ├── install.sh
│   └── dotwell.toml
├── red/
└── green/
```

run dotwell, pick one, press `i`. done.

## status

works: browsing, installing themes with make/bash
todo: better gcc/cargo/nix support, backups, filtering

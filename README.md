# Sxtetris

A terminal Tetris game written in Rust, built with [ratatui](https://github.com/ratatui-org/ratatui) and [tokio](https://github.com/tokio-rs/tokio).

https://github.com/shixinhuang99/sxtetris/assets/31186725/1eb95c10-e57e-4239-8142-95800e1afca8

## Features

- Classic Tetris gameplay in your terminal
- Smooth animations and responsive controls
- Background music and sound effects
- High score tracking
- Cross-platform compatibility

## Installation

### Cargo

```sh
cargo install sxtetris --locked
```

### AUR (Arch Linux)

You can install `sxtetris` from the [AUR](https://aur.archlinux.org/packages/sxtetris) by using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers):

```sh
paru -S sxtetris
```

### Binary Releases

Download pre-built binaries from the [latest releases](https://github.com/shixinhuang99/sxtetris/releases).

## System Requirements

1. **Terminal Size**: Suggested minimum terminal size: `105x45`
2. **Color Support**: Terminal must support 24-bit color (true color)
3. **Recommended Terminals**:
   - macOS: `iTerm2`
   - Linux: `GNOME Terminal`
   - Windows: `Windows Terminal`

## Usage

```sh
sxtetris
```

To see the save file location:

```sh
sxtetris --show-save-path
```

## Audio Sources

All audio files are located in the `src/global/assets` directory. Files from [freesound](https://freesound.org) have been trimmed for optimal gameplay experience.

| File          | Author          | Source                                                                               |
| ------------- | --------------- | ------------------------------------------------------------------------------------ |
| bg_music.mp3  | Stable Audio    | [Stable Audio](https://stableaudio.com/1/share/a6ae7a5b-9acf-4082-9032-7b32a9b76c96) |
| game_over.wav | themusicalnomad | [FreeSound](https://freesound.org/people/themusicalnomad/sounds/253886/)             |
| lock.wav      | Mellau          | [FreeSound](https://freesound.org/people/Mellau/sounds/506054/)                      |
| menu.wav      | Christopherderp | [FreeSound](https://freesound.org/people/Christopherderp/sounds/342200/)             |
| move.wav      | aphom000        | [FreeSound](https://freesound.org/people/aphom000/sounds/623175/)                    |
| clear.mp3     | B_Lamerichs     | [FreeSound](https://freesound.org/people/B_Lamerichs/sounds/193123/)                 |

## Troubleshooting

### Linux ALSA Error

If you encounter the following error when using `cargo install`:

```
The system library `alsa` required by crate `alsa-sys` was not found.
The file `alsa.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
The PKG_CONFIG_PATH environment variable is not set.

HINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `alsa.pc`.
```

Run these commands and try installing again:

```sh
sudo apt-get update
sudo apt-get install libasound2-dev
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/lib/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig
```

## Disclaimer

This project is an open-source implementation of Tetris designed to run in the terminal. Tetris is a registered trademark, and all rights belong to its respective owners. This project is intended solely for educational and research purposes and should not be used for commercial purposes.

By using this project, you agree to use it exclusively for non-commercial educational and research purposes. The developer assumes no responsibility for any infringement of intellectual property rights.

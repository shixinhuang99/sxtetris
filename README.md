# Sxtetris

A terminal Tetris game written in Rust, based on [ratatui](https://github.com/ratatui-org/ratatui) and [tokio](https://github.com/tokio-rs/tokio)

https://github.com/shixinhuang99/sxtetris/assets/31186725/1eb95c10-e57e-4239-8142-95800e1afca8

## Installation

### Cargo

```sh
cargo install sxtetris --locked
```

### AUR

You can install `sxtetris` from the [AUR](https://aur.archlinux.org/packages/sxtetris) with using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers).

```sh
paru -S sxtetris
```

### Binary releases

Download the [latest release binary](https://github.com/shixinhuang99/sxtetris/releases)

## Note

1. Suggested minimal terminal size: `176x49`.
2. The terminal needs to support 24 bit color(true color), such as the `iTerm2`.
3. If you are a macOS user, please do not use the built-in terminal, as its forced line spacing makes the characters in the game look weird. It is recommended to use more modern terminals such as `iTerm2`.
4. Use the `--show-save-path` argument at launch game to see where the archive is located.
5. If you are a linux user and get the following error when using `cargo install`:

```
The system library `alsa` required by crate `alsa-sys` was not found.
The file `alsa.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
The PKG_CONFIG_PATH environment variable is not set.

HINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `alsa.pc`.
```

you can try running the following command and install again:

```sh
sudo apt-get update
sudo apt-get install libasound2-dev
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/lib/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig
```

## Sources of music and sound effects

All audio files are in the `src/global/assets` directory, files from [freesound](https://freesound.org) are trimmed

| file          | author          | source                                                                 |
| ------------- | --------------- | ---------------------------------------------------------------------- |
| bg_music.mp3  | Stable Audio    | <https://stableaudio.com/1/share/a6ae7a5b-9acf-4082-9032-7b32a9b76c96> |
| game_over.wav | themusicalnomad | <https://freesound.org/people/themusicalnomad/sounds/253886/>          |
| lock.wav      | Mellau          | <https://freesound.org/people/Mellau/sounds/506054/>                   |
| menu.wav      | Christopherderp | <https://freesound.org/people/Christopherderp/sounds/342200/>          |
| move.wav      | aphom000        | <https://freesound.org/people/aphom000/sounds/623175/>                 |
| clear.mp3     | B_Lamerichs     | <https://freesound.org/people/B_Lamerichs/sounds/193123/>              |

## Disclaimer

This project is an open-source Tetris game running in the terminal. The Tetris game is a registered trademark, and all rights belong to its respective owner. This project is intended for educational and research purposes only and should not be used for any commercial purposes.

By using this project, you agree to use it solely for non-commercial educational and research purposes. The developer is not responsible for any infringement.

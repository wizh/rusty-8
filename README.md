# Rusty-8
rusty-8 is a [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator. It uses [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) for audio, display and input.

## Requirements
The SDL2 framework needs to be installed - see instructions [here](https://github.com/AngryLawyer/rust-sdl2#requirements).

## Building the project
```
$ git clone https://github.com/wizh/rusty-8.git
$ cd rusty-8
$ cargo build
```

## Running games
```
$ cargo run roms/${GAME}
```
Ex:
```
$ cargo run roms/PONG
```

# Images
## Chip8 emulator using Rust

Chip8 emulator written in rust using sdl2 for graphics, audio and input.
Very fun little project to work on if anyone wants to get started with emulator development.

![Screenshot](assets/cpong.png)

## Helpful resources

- Cowgod's [chip8 technical reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [How to write an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Errata for Cowgod's technical reference](https://github.com/gulrak/cadmium/wiki/CTR-Errata)
- I also used [Mikezaby's chip8 repository](https://github.com/mikezaby/chip-8.rs) as a reference though I didn't use any of his code.

### Prerequisites

- Rust

- SDL2 development libraries

On Arch:
```
sudo pacman -S sdl2
```

### Usage

You can find public domain games [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html)

```
cargo run --release -- path/to/rom
```

If no path is provided, runs pong by default

```
cargo run --release
```


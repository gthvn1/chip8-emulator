# Chip8 Emulator

## What?
- Yet another *chip8* emulator in rust
- We are using Raylib so you need to build `libraylib.a` and put it under a newly created `raylib/` dir.
    - to build Raylib check the official [raylib](https://www.raylib.com/).
    - if you want to put it elsewhere you will need to modify [build.rs](https://github.com/gthvn1/chip8-emulator/blob/master/build.rs)
- To run it: `cargo run -- <ROMS>`
    - See [Timendus Chip8 test suite](https://github.com/Timendus/chip8-test-suite) to have some ROMS
- For more logs set `RUST_LOG=debug` (or info, ...)

## Notes
- We are using [hw randr](https://doc.rust-lang.org/core/arch/x86/fn._rdrand16_step.html)
    - So it only works on x86_64
- [Changelog](https://github.com/gthvn1/chip8-emulator/blob/master/Changelog.md)

## Todo
- [x] pass corax test
- [ ] play pong
    - display is white/black instead of black/white
    - keyboard is not working

## Links
- [Awesome CHIP-8](https://chip-8.github.io/links/)
- [Writing CHIP-8 emulator in C](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Chip8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Timendus Chip8 test suite](https://github.com/Timendus/chip8-test-suite)
- [Chip8 emulator on wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [Writing a Chip8 Emulator](http://craigthomas.ca/blog/2014/06/21/writing-a-chip-8-emulator-part-1/)
- [Inline emulator](https://chip-8.vercel.app/)

## Screenshots
### New ones with Raylib
#### Corax + opcode test
![](https://github.com/gthvn1/chip8-emulator/blob/master/screenshots/corax.png)
#### Timendus splash screen
![](https://github.com/gthvn1/chip8-emulator/blob/master/screenshots/timendus_raylib.png)
### Old ones using minibf
#### Timendus splash screen
![](https://github.com/gthvn1/chip8-emulator/blob/master/screenshots/timendus.png)
#### Drawing numbers...
![](https://github.com/gthvn1/chip8-emulator/blob/master/screenshots/drawing_numbers.png)


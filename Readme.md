# Chip8 Emulator

## What?
- Another *chip8* emulator in rust
- We are now using Raylib so you need to build `libraylib.a` and put it under a newly created `raylib/` dir.
    - to build Raylib check the official [raylib](https://www.raylib.com/).
- Only instructions used to display Timendus spash screen are implemented
- To run it: `cargo run --bin emulator <ROMS>`
    - See Timendus link below for some testing ROMS
- To debug prepend `RUST_LOG=debug`
- [Changelog](https://github.com/gthvn1/chip8-emulator/blob/master/Changelog.md)

## Todo
- Next to implement for pong is `FX15`
- But before:
    - `FX15` is dealing with timer. We don't have any notion of timer yet.
    - So we need to introduce it and we need to change our code
    - We will use another frontend than minibf that doesn't have any FPS.
    - Thus we will redisign a bit our emulator to work with Raylib...

## Links

- [Chip8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Timendus Chip8 test suite](https://github.com/Timendus/chip8-test-suite)
- [Chip8 emulator on wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [Writing a Chip8 Emulator](http://craigthomas.ca/blog/2014/06/21/writing-a-chip-8-emulator-part-1/)
- [Inline emulator](https://chip-8.vercel.app/)

## Screenshots

### Timendus splash screen
![](https://github.com/gthvn1/chip8-emulator/blob/master/screenshots/timendus.png)

### Drawing numbers...
![](https://github.com/gthvn1/chip8-emulator/blob/master/screenshots/drawing_numbers.png)


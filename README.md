## Chirp-8
A simple CHIP-8 emulator written in Rust. It may not be blazingly fast :fire:, but it works well enough.

It supports some options such as using old behaviors that became mostly obsolete. You can also modify the
number of instructions per seconds (ips) that emulator can process.

## Usage
If you want to run this emulator, you need to compile it yourself.
You can do it using `cargo`:
```sh
$ cargo build --release
```

This will put final executable in `/target/release/chirp-8`.
Then you need a ROM to emulate. You can find a lot of them
on the Internet.

Here's the emulator output with `--help`:
```
Usage: chirp-8 [OPTIONS] [ROM]

Arguments:
  [ROM]  Path to the ROM for emulator to run [default: rom.ch8]

Options:
  -i, --ips <IPS>                    # of instructions per second that emulator will execute [default: 700]
  -s, --old-shift-behavior           Enable old shift (8XY6 & 8XYE) behavior
  -j, --new-jump-behavior            Enable new jump with offset (BNNN) behavior
  -m, --old-store-load-behavior      Enable old store/load (FX55/FX65) behavior
  -o, --index-overflow               Set VF when index overflows 0x1000
  -t, --timer-period <TIMER_PERIOD>  Sound/delay timer perioid in milliseconds [default: 16]
  -c, --scale <SCALE>                Scale for the display, the size is determined by (64 * scale) x (32 * scale) [default: 10]
  -v, --volume <VOLUME>              Volume (0 - 100), higher values will be identical to 100 [default: 50]
  -h, --help                         Print help
```

## Motivations
- I wanted to learn more about emulators.
- CHIP-8 seemed like a good choice, as it's simple
and there're a lot of resources I can use.

I began writing this some time ago, I implemented ~6 basic instructions and got bored.
Later I was looking for an idea for a project and came across it on my laptop. So I continued
and a few days later it was done.

The code probably could be better in some aspects, but I'm happy with it.

## License

Chirp-8 is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.


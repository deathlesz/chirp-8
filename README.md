## Chirp-8
A simple CHIP-8 emulator written in Rust. It may not be blazingly fast :fire:, but it works well enough.

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

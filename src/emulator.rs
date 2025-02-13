use color_eyre::{
    eyre::{bail, eyre, Context as _},
    Result,
};
use rand::Rng as _;
use rodio::Sink;
use sdl2::{keyboard::Scancode, EventPump, Sdl};

use crate::{
    display::Display,
    instruction::{Instruction, IntExt as _},
    memory::{Memory, RegIdx, Registers},
    sound::SawWave,
    Args,
};

pub struct Chip8 {
    args: Args,

    memory: Memory,
    display: Display,
    stack: Vec<u16>,
    event_pump: EventPump,

    regs: Registers,
    index: u16,

    delay_timer: u8,
    sound_timer: u8,

    pc: u16,
}

impl Chip8 {
    pub fn new(rom: &[u8], context: Sdl, args: Args) -> Result<Self> {
        let scale = args.scale;

        Ok(Self {
            args,

            memory: Memory::new(rom),
            display: Display::new(&context, scale)?,
            event_pump: context
                .event_pump()
                .map_err(|_| eyre!("failed to initialize event pump"))?,
            stack: Vec::new(),

            regs: Registers::new(),
            index: 0,

            sound_timer: 0,
            delay_timer: 0,

            pc: 512,
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.display.update()?;

        let mut now = std::time::Instant::now();
        let mut keys = [false; 16];
        let mut rng = rand::rng();
        let (_stream, stream_handle) =
            rodio::OutputStream::try_default().wrap_err("failed to initialize sound")?;
        let sink = Sink::try_new(&stream_handle)?;

        sink.pause();

        sink.set_volume(self.args.volume.min(100) as f32 / 100.0 * 0.025);
        sink.append(SawWave::new(440.0, 48000));

        loop {
            for event in self.event_pump.poll_iter() {
                use sdl2::event::Event;

                match event {
                    Event::Quit { .. } | Event::AppTerminating { .. } => return Ok(()),
                    Event::KeyDown {
                        scancode: Some(scancode),
                        ..
                    } => {
                        if let Some(idx) = scancode_to_key(scancode) {
                            keys[idx as usize] = true
                        }
                    }
                    Event::KeyUp {
                        scancode: Some(scancode),
                        ..
                    } => {
                        if let Some(idx) = scancode_to_key(scancode) {
                            keys[idx as usize] = false
                        }
                    }
                    _ => {}
                }
            }

            match self.fetch_and_decode()? {
                Instruction::ClearScreen => self.display.clear()?,
                Instruction::Jump(nnn) => self.pc = nnn,
                Instruction::SetIm(vx, nn) => self.regs[vx] = nn,
                Instruction::AddIm(vx, nn) => self.regs[vx] = self.regs[vx].wrapping_add(nn),
                Instruction::SetIndex(nnn) => self.index = nnn,
                Instruction::Draw(vx, vy, n) => self.draw(vx, vy, n)?,
                Instruction::Call(addr) => {
                    self.stack.push(self.pc);
                    self.pc = addr;
                }
                Instruction::Return => match self.stack.pop() {
                    Some(pc) => self.pc = pc,
                    None => bail!("invalid return; call stack is empty"),
                },
                Instruction::SkipEqIm(vx, nn) => self.skip_if(|s| s.regs[vx] == nn),
                Instruction::SkipNeIm(vx, nn) => self.skip_if(|s| s.regs[vx] != nn),
                Instruction::SkipEq(vx, vy) => {
                    self.skip_if(|s| s.regs[vx] == s.regs[vy]);
                }
                Instruction::SkipNe(vx, vy) => {
                    self.skip_if(|s| s.regs[vx] != s.regs[vy]);
                }
                Instruction::Set(vx, vy) => self.regs[vx] = self.regs[vy],
                Instruction::Or(vx, vy) => self.regs[vx] |= self.regs[vy],
                Instruction::And(vx, vy) => self.regs[vx] &= self.regs[vy],
                Instruction::Xor(vx, vy) => self.regs[vx] ^= self.regs[vy],
                Instruction::Add(vx, vy) => {
                    let (result, overflow) = self.regs[vx].overflowing_add(self.regs[vy]);

                    (self.regs[vx], self.regs[RegIdx::FLAG]) = (result, overflow as u8);
                }
                Instruction::Sub(vx, vy) => {
                    let (x, y) = (self.regs[vx], self.regs[vy]);

                    self.regs[vx] = x.wrapping_sub(y);
                    self.regs[RegIdx::FLAG] = (x >= y) as u8;
                }
                Instruction::SubOpp(vx, vy) => {
                    let (x, y) = (self.regs[vx], self.regs[vy]);

                    self.regs[vx] = y.wrapping_sub(x);
                    self.regs[RegIdx::FLAG] = (y >= x) as u8;
                }
                Instruction::Shr(vx, vy) => {
                    if self.args.old_shift_behavior {
                        self.regs[vx] = self.regs[vy];
                    }

                    self.regs[RegIdx::FLAG] = self.regs[vx] & 1;
                    self.regs[vx] >>= 1;
                }
                Instruction::Shl(vx, vy) => {
                    if self.args.old_shift_behavior {
                        self.regs[vx] = self.regs[vy];
                    }

                    self.regs[RegIdx::FLAG] = (self.regs[vx] & 0b10000000) >> 7;
                    self.regs[vx] <<= 1;
                }
                Instruction::JumpV0(nnn) => {
                    let reg = RegIdx::new(if self.args.new_jump_behavior {
                        nnn.nibble2()
                    } else {
                        0
                    });

                    self.pc = self.regs[reg] as u16 + nnn;
                }
                Instruction::RandAnd(vx, nn) => self.regs[vx] = rng.random::<u8>() & nn,
                Instruction::GetDelay(vx) => self.regs[vx] = self.delay_timer,
                Instruction::SetDelay(vx) => self.delay_timer = self.regs[vx],
                Instruction::SetSound(vx) => self.sound_timer = self.regs[vx],
                Instruction::AddIndex(vx) => {
                    self.index += self.regs[vx] as u16;
                    if self.args.index_overflow && self.index >= 4096 {
                        self.regs[RegIdx::FLAG] = 1;
                    }
                }
                Instruction::SetBcd(vx) => {
                    let x = self.regs[vx];

                    self.memory.write_u8(self.index, x / 100)?;
                    self.memory.write_u8(self.index + 1, (x / 10) % 10)?;
                    self.memory.write_u8(self.index + 2, x % 10)?;
                }
                Instruction::RegStore(vx) => {
                    for idx in 0..=vx.as_u8() {
                        self.memory
                            .write_u8(self.index + idx as u16, self.regs[RegIdx::new(idx)])?;
                    }

                    if self.args.old_store_load_behavior {
                        self.index += vx.as_u8() as u16 + 1;
                    }
                }
                Instruction::RegLoad(vx) => {
                    for i in 0..=vx.as_u8() {
                        self.regs[RegIdx::new(i)] = self.memory.read_u8(self.index + i as u16)?;
                    }

                    if self.args.old_store_load_behavior {
                        self.index += vx.as_u8() as u16 + 1;
                    }
                }
                Instruction::IndexCharacter(vx) => {
                    let x = self.regs[vx];

                    self.index = 0x50 + 5 * (x as u16 & 0x0F);
                }
                Instruction::GetKey(vx) => {
                    if let Some(key) = keys.iter().position(|&x| x) {
                        self.regs[vx] = key as u8;
                    } else {
                        self.pc -= 2; // do this instruction again
                    }
                }
                Instruction::SkipKeyEq(vx) => {
                    if keys[self.regs[vx] as usize] {
                        self.pc += 2;
                    }
                }
                Instruction::SkipKeyNe(vx) => {
                    if !keys[self.regs[vx] as usize] {
                        self.pc += 2;
                    }
                }
            }

            let millis = now.elapsed().as_millis();
            if millis > self.args.timer_period {
                let decrement = (millis / self.args.timer_period) as u8;

                self.delay_timer = self.delay_timer.saturating_sub(decrement);
                self.sound_timer = self.sound_timer.saturating_sub(decrement);

                now = std::time::Instant::now();

                // HACK:? if it's not here, if nothing is being drawn and you switch to another
                // window, the screen is being weird
                self.display.update()?;
            }

            if self.sound_timer > 0 {
                sink.play();
            } else {
                sink.pause();
            }

            std::thread::sleep(std::time::Duration::from_secs_f64(
                1f64 / self.args.ips as f64,
            ));
        }
    }

    fn fetch_and_decode(&mut self) -> Result<Instruction> {
        let inst = self.memory.read_u16(self.pc)?;
        self.pc += 2;

        Instruction::decode(inst).ok_or_else(|| eyre!("unknown instruction: {inst:04x}"))
    }

    fn skip_if(&mut self, pred: impl Fn(&Self) -> bool) {
        if pred(self) {
            self.pc += 2
        }
    }

    fn draw(&mut self, vx: RegIdx, vy: RegIdx, n: u8) -> Result<()> {
        let (x, y) = (
            self.regs[vx] % Display::WIDTH,
            self.regs[vy] % Display::HEIGHT,
        );
        self.regs[RegIdx::FLAG] = 0;

        for i in 0..n {
            let pixel = self.memory.read_u8(self.index + i as u16)?;
            for j in 0..8 {
                if (pixel & (0x80 >> j)) != 0 {
                    if x + j >= Display::WIDTH || y + i >= Display::HEIGHT {
                        continue;
                    }

                    if self.display.buffer[(x + j) as usize][(y + i) as usize] {
                        self.regs[RegIdx::FLAG] = 1
                    }

                    self.display.buffer[(x + j) as usize][(y + i) as usize] ^= true;
                }
            }
        }

        self.display.update()
    }
}

const fn scancode_to_key(scancode: Scancode) -> Option<u8> {
    match scancode {
        Scancode::Num1 => Some(0),
        Scancode::Num2 => Some(1),
        Scancode::Num3 => Some(2),
        Scancode::Num4 => Some(3),

        Scancode::Q => Some(4),
        Scancode::W => Some(5),
        Scancode::E => Some(6),
        Scancode::R => Some(7),

        Scancode::A => Some(8),
        Scancode::S => Some(9),
        Scancode::D => Some(10),
        Scancode::F => Some(11),

        Scancode::Z => Some(12),
        Scancode::X => Some(13),
        Scancode::C => Some(14),
        Scancode::V => Some(15),

        _ => None,
    }
}

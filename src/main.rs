use std::path::PathBuf;

use clap::Parser;
use color_eyre::{
    eyre::{eyre, Context as _},
    Result, Section as _,
};
use emulator::Chip8;

mod display;
mod emulator;
mod instruction;
mod memory;
mod sound;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let rom = std::fs::read(&args.rom).with_suggestion(|| "check if the ROM file exists")?;

    let context = sdl2::init().map_err(|_| eyre!("failed to initialize sdl2"))?;

    let emu = Chip8::new(&rom, context, args).with_context(|| "failed to create emulator")?;
    emu.run()
        .with_context(|| "an error occured during emulating")?;

    Ok(())
}

#[derive(Debug, Parser)]
/// A simple CHIP-8 emulator
pub struct Args {
    /// # of instructions per second that emulator will execute
    #[arg(short, long, default_value_t = 700)]
    pub ips: u64,
    /// Enable old shift (8XY6 & 8XYE) behavior
    #[arg(short = 's', long, default_value_t = false)]
    pub old_shift_behavior: bool,
    /// Enable new jump with offset (BNNN) behavior
    #[arg(short = 'j', long, default_value_t = false)]
    pub new_jump_behavior: bool,
    /// Enable old store/load (FX55/FX65) behavior
    #[arg(short = 'm', long, default_value_t = false)]
    pub old_store_load_behavior: bool,
    /// Set VF when index overflows 0x1000
    #[arg(short = 'o', long, default_value_t = false)]
    pub index_overflow: bool,
    /// Sound/delay timer perioid in milliseconds
    #[arg(short, long, default_value_t = 16)]
    pub timer_period: u128,
    /// Scale for the display, the size is determined by (64 * scale) x (32 * scale)
    #[arg(short = 'c', long, default_value_t = 10)]
    pub scale: u32,
    /// Volume (0 - 100), higher values will be identical to 100
    #[arg(short, long, default_value_t = 50)]
    pub volume: u8,
    /// Path to the ROM for emulator to run
    #[arg(default_value_os_t = PathBuf::from("rom.ch8"))]
    pub rom: PathBuf,
}

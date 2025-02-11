use color_eyre::{eyre::eyre, Result};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub struct Display {
    canvas: Canvas<Window>,
    scale: u32,
    pub buffer: [[bool; Self::HEIGHT as usize]; Self::WIDTH as usize],
    pub old_buffer: [[bool; Self::HEIGHT as usize]; Self::WIDTH as usize],
}

impl Display {
    pub const WIDTH: u8 = 64;
    pub const HEIGHT: u8 = 32;

    pub fn new(context: &Sdl, scale: u32) -> Result<Self> {
        let video = context
            .video()
            .map_err(|_| eyre!("failed to initialize video subsystem"))?;

        let window = video
            .window(
                "CHIRP-8",
                Self::WIDTH as u32 * scale,
                Self::HEIGHT as u32 * scale,
            )
            // .opengl()
            .position_centered()
            .build()
            .map_err(|_| eyre!("failed to create a window"))?;

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|_| eyre!("failed to create a canvas"))?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Self {
            canvas,
            scale,
            buffer: [[false; Self::HEIGHT as usize]; Self::WIDTH as usize],
            old_buffer: [[false; Self::HEIGHT as usize]; Self::WIDTH as usize],
        })
    }

    pub fn retire(&mut self) {
        self.old_buffer.copy_from_slice(&self.buffer);
    }

    pub fn clear(&mut self) -> Result<()> {
        self.retire();
        self.buffer = [[false; Self::HEIGHT as usize]; Self::WIDTH as usize];

        self.update()
    }

    pub fn update(&mut self) -> Result<()> {
        for i in 0..Self::WIDTH {
            for j in 0..Self::HEIGHT {
                // FIXME: causes some weird rendering bugs
                // if self.buffer[i as usize][j as usize] ^ self.old_buffer[i as usize][j as usize] {
                let rect = Rect::new(
                    i as i32 * self.scale as i32,
                    j as i32 * self.scale as i32,
                    self.scale,
                    self.scale,
                );

                if self.buffer[i as usize][j as usize] {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }

                self.canvas
                    .fill_rect(rect)
                    .map_err(|_| eyre!("failed to draw"))?;
                // }
            }
        }

        self.retire();
        self.canvas.present();

        Ok(())
    }
}

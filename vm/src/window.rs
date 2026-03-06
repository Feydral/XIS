use common::{hardware, math::mathi};
use minifb::WindowOptions;

pub struct Window {
    buffer: Vec<u32>,
    native_window: minifb::Window,
}

impl Window {
    pub const TITLE: &str = "Xis Virtual Machine";
    pub const WINDOW_WIDTH: u32 = 768;
    pub const WINDOW_HEIGHT: u32 = 768;

    const TARGET_FPS: u32 = 0;

    const WINDOW_OPTIONS: WindowOptions = WindowOptions {
        scale: minifb::Scale::X1,
        scale_mode: minifb::ScaleMode::AspectRatioStretch,
        borderless: false,
        title: true,
        resize: false,
        topmost: false,
        transparency: false,
        none: false,
    };

    pub fn new() -> Self {
        let mut native_window = minifb::Window::new(
            &Self::TITLE,
            Self::WINDOW_WIDTH as usize,
            Self::WINDOW_HEIGHT as usize,
            Self::WINDOW_OPTIONS
        ).unwrap();

        native_window.set_target_fps(Self::TARGET_FPS as usize);

        Self {
            buffer: vec![0u32; (hardware::SCREEN_WIDTH * hardware::SCREEN_WIDTH) as usize],
            native_window,
        }
    }

    pub fn update_buffer(&mut self) {
        if let Err(e) = self.native_window.update_with_buffer(&self.buffer, hardware::SCREEN_WIDTH as usize, hardware::SCREEN_WIDTH as usize) {
            eprintln!("Error updating buffer: {}", e);
        }
    }

    pub fn is_open(&self) -> bool {
        self.native_window.is_open()
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u16) {
        if x < hardware::SCREEN_WIDTH && y < hardware::SCREEN_HEIGHT {
            let index = mathi::xy_to_index(x, y, hardware::SCREEN_WIDTH, hardware::SCREEN_HEIGHT) as usize;

            let r4 = ((color >> 8) & 0xF) as u8;
            let g4 = ((color >> 4) & 0xF) as u8;
            let b4 = (color & 0xF) as u8;

            let r8 = (r4 << 4) | r4;
            let g8 = (g4 << 4) | g4;
            let b8 = (b4 << 4) | b4;

            self.buffer[index] = ((r8 as u32) << 16) | ((g8 as u32) << 8) | (b8 as u32);
        }
    }
}
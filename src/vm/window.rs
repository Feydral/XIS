use minifb::WindowOptions;

use crate::math::{mathf, numerics::float3::Float3};

pub struct Window {
    buffer: Vec<u32>,
    native_window: minifb::Window,
}

impl Window {
    pub const TITLE: &str = "Canvas";
    pub const WINDOW_WIDTH: usize = 768;
    pub const WINDOW_HEIGHT: usize = 768;

    pub const BUFFER_WIDTH: usize = Self::WINDOW_WIDTH / 3;
    pub const BUFFER_HEIGHT: usize = Self::WINDOW_HEIGHT / 3;

    const TARGET_FPS: usize = 64;

    const WINDOW_OPTIONS: WindowOptions = WindowOptions {
        scale: minifb::Scale::X1,
        scale_mode: minifb::ScaleMode::AspectRatioStretch,
        borderless: false,
        title: true,
        resize: false,
        topmost: true,
        transparency: false,
        none: false,
    };

    pub fn new() -> Self {
        let mut native_window = minifb::Window::new(
            &Self::TITLE,
            Self::WINDOW_WIDTH,
            Self::WINDOW_HEIGHT,
            Self::WINDOW_OPTIONS
        ).unwrap();

        native_window.set_target_fps(Self::TARGET_FPS);

        Self {
            buffer: vec![0u32; Self::BUFFER_WIDTH * Self::BUFFER_HEIGHT],
            native_window,
        }
    }

    pub fn update_buffer(&mut self, buffer: &[&[Float3]]) {
        for x in 0..Self::BUFFER_WIDTH {
            for y in 0..Self::BUFFER_HEIGHT {
                let idx = y * Self::BUFFER_WIDTH + x;
                self.buffer[idx] = mathf::float3_to_u32_rgb(buffer[x][y]);
            }
        }

        if let Err(e) = self.native_window.update_with_buffer(&self.buffer, Self::BUFFER_WIDTH, Self::BUFFER_HEIGHT) {
            eprintln!("Error updating buffer: {}", e);
        }
    }

    pub fn is_open(&self) -> bool {
        self.native_window.is_open()
    }
}
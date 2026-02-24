use minifb::WindowOptions;

use crate::math::{mathf, numerics::float3::Float3};

pub struct Window {
    buffer: Vec<u32>,
    native_window: minifb::Window,
}

impl Window {
    pub const TITLE: &str = "Xis Virtual Machine";
    pub const WINDOW_WIDTH: u32 = 768;
    pub const WINDOW_HEIGHT: u32 = 768;

    pub const BUFFER_WIDTH: u32 = Self::WINDOW_WIDTH / 3;
    pub const BUFFER_HEIGHT: u32 = Self::WINDOW_HEIGHT / 3;

    const TARGET_FPS: u32 = 64;

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
            Self::WINDOW_WIDTH as usize,
            Self::WINDOW_HEIGHT as usize,
            Self::WINDOW_OPTIONS
        ).unwrap();

        native_window.set_target_fps(Self::TARGET_FPS as usize);

        Self {
            buffer: vec![0u32; (Self::BUFFER_WIDTH * Self::BUFFER_HEIGHT) as usize],
            native_window,
        }
    }

    pub fn update_buffer(&mut self) {
        if let Err(e) = self.native_window.update_with_buffer(&self.buffer, Self::BUFFER_WIDTH as usize, Self::BUFFER_HEIGHT as usize) {
            eprintln!("Error updating buffer: {}", e);
        }
    }

    pub fn is_open(&self) -> bool {
        self.native_window.is_open()
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Float3) {
        if x < Window::BUFFER_WIDTH && y < Window::BUFFER_HEIGHT {
            let index = y as usize * Window::BUFFER_WIDTH as usize + x as usize;
            self.buffer[index] = mathf::float3_to_u32_rgb(color);
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Float3 {
        if x < Self::BUFFER_WIDTH && y < Self::BUFFER_HEIGHT {
            let index = y as usize * Self::BUFFER_WIDTH as usize + x as usize;
            mathf::u32_to_float3_rgb(self.buffer[index])
        } else {
            Float3::ZERO
        }
    }
}
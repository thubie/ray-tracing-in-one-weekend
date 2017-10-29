
extern crate minifb;
extern crate byteorder;

use minifb::{Key, WindowOptions, Window};
use byteorder::*;

const WIDTH: usize = 1280;
const HEIGHT: usize = 640;

//#[repr(C)]
struct Color {
    channels: [u8;4]
}

impl Color {
    #[inline]
    fn get_value_u32(&self) -> u32 {
        BigEndian::read_u32(&self.channels)
    }


    fn set_channels(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.channels = [alpha, red, green, blue];
    }

    fn set_red_channel_value(&mut self, value: u8) {
        self.channels[1] = value
    }

    fn set_green_channel_value(&mut self, value: u8) {
        self.channels[2] = value
    }

    fn set_blue_channel_value(&mut self, value: u8) {
        self.channels[3] = value
    }

    fn set_alpha_channel_value(&mut self, value: u8) {
        self.channels[0] = value
    }




}

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut pixel_color = Color { channels: [0; 4]};
        for j in (0..HEIGHT).rev() {
            for i in 0..WIDTH {
                let red: f32 = i as f32 / WIDTH as f32;
                let green: f32 = j as f32 / HEIGHT as f32;
                let blue: f32 = 0.2;

                pixel_color.set_channels((255.99 * red) as u8, (255.99 * green) as u8, (255.99 * blue) as u8, 0);
                buffer[(HEIGHT - j - 1) * WIDTH + i] = pixel_color.get_value_u32();
            }
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}


#![windows_subsystem = "windows"]

extern crate minifb;
extern crate perlin_noise as perlin;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use palette::{Gradient, LinSrgb};
use perlin::PerlinNoise;
use std::time::Duration;

const INITIAL_HEIGHT: usize = 150;
const INITIAL_WIDTH: usize = 250;
const COLOR_RANGE: f64 = 100f64;

fn main() {
   let mut size = (0, 0);
   let mut buffer: Vec<u32> = vec![0; INITIAL_WIDTH * INITIAL_HEIGHT];
   
   let mut window = Window::new(
      "PERLIN-NOISE-RS",
      INITIAL_WIDTH,
      INITIAL_HEIGHT,
      WindowOptions {
         borderless: false,
         title: true,
         resize: true,
         scale: Scale::X1,
         scale_mode: ScaleMode::AspectRatioStretch,
         topmost: true,
         transparency: false,
      },
   )
   .unwrap_or_else(|e| panic!("{}", e));
   
   window.limit_update_rate(Some(Duration::from_millis(20)));
   
   let perlin = PerlinNoise::new();
   let mut time = 0.0;
   let mut direction = 0.003;
   let grad1 = Gradient::with_domain(vec![
      (0.0, LinSrgb::new(2.0, 0.0, 36.0)),
      (35.0, LinSrgb::new(9.0, 9.0, 121.0)),
      (100.0, LinSrgb::new(0.0, 212.0, 255.0))
   ]);

   while window.is_open() && !window.is_key_down(Key::Escape) {
      let new_size = window.get_size();
      if new_size != size {
         size = new_size;
         buffer.resize(size.0 * size.1, 30249);
      }
      
      if time > 1.0 || time == 0.0 {
         direction = -direction;
      }
      
      time += direction;
      
      for y in 0..size.1 {
         let row_idx = y * size.0;
         for x in 0..size.0 {
            let res = perlin.get3d([x as f64 / size.0 as f64, y as f64 / size.1 as f64, time]);
            let color = grad1.get(res * 30f64);
            buffer[row_idx + x] =
            (((color.red * COLOR_RANGE) as u32) << 16)
            | (((color.green * COLOR_RANGE) as u32) << 8)
            | ((color.blue * COLOR_RANGE) as u32);
         }
      }
      
      if let Err(e) = window.update_with_buffer(&buffer, size.0, size.1) {
         eprintln!("Unable to update the window:\n{:?}", e);
      }
   }
}

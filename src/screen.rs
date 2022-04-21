use crate::traits::{BASE_WIDTH, BASE_HEIGHT, BUFFER_SIZE, ScreenDisplay};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// const BASE_WIDTH: u32 = 64;
// const BASE_HEIGHT: u32 = 32;
const COLOR_BG: Color = Color::RGB(0, 0, 0);
const COLOR_FG: Color = Color::RGB(255, 255, 255);

enum PixelColors {
    ColorBg,
    ColorFg,
}
/// A chip-8 screen is implemented here. The screen is created with a resolution of 800x600.
/// However, due to higher screen resolution and bigger screen, we can apply a scaling factor to
/// get a bigger display than usual. Each pixel should be scaled properly.
pub struct Screen {
    canvas:Canvas<Window>,
    scale_factor: u32
}

impl Screen {
    /// Return a screen canvas with the proper scaling factor implemented
    /// # Arguments
    /// * `scale_factor` - A unsigned integer specifying the scaling factor to apply to the screen
    /// ```
    pub fn new(sdl_context: &sdl2::Sdl, scale_factor: u32) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("CHIP-8 EMU", BASE_WIDTH*scale_factor, BASE_HEIGHT*scale_factor)
            .build()
            .unwrap();
        let canvas:Canvas<Window> = window.into_canvas()
            .present_vsync()
            .build().unwrap();
        Self {canvas, scale_factor}
    }

    fn show(&mut self) {
        self.canvas.present();
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: PixelColors) {
        let pixel = Rect::new(x as i32 * self.scale_factor as i32,
                              y as i32 *self.scale_factor as i32,
                              self.scale_factor, self.scale_factor);
        match color {
            PixelColors::ColorBg => self.canvas.set_draw_color(COLOR_BG),
            PixelColors::ColorFg => self.canvas.set_draw_color(COLOR_FG)
        }
        self.canvas.fill_rect(pixel).unwrap();
    }

}

impl ScreenDisplay for Screen {
    fn draw(&mut self, buffer: [u8; BUFFER_SIZE]) {
        for y in 0..BASE_HEIGHT{
            for x in 0..BASE_WIDTH {
                let index: usize = y as usize * BASE_WIDTH as usize + x as usize;
                let pixel_color = if buffer[index] == 1 {
                    PixelColors::ColorFg
                } else {
                    PixelColors::ColorBg
                };
                self.draw_pixel(x, y, pixel_color);
            }
        }
        self.show()
    }
}

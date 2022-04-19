use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const BASE_WIDTH: u32 = 64;
const BASE_HEIGHT: u32 = 32;
const COLOR_BG: Color = Color::RGB(0, 0, 0);
const COLOR_FG: Color = Color::RGB(255, 255, 255);
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
    pub fn new(scale_factor: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("CHIP-8 EMU", BASE_WIDTH*scale_factor, BASE_HEIGHT*scale_factor)
            .build()
            .unwrap();
        let canvas:Canvas<Window> = window.into_canvas()
            .present_vsync()
            .build().unwrap();
        Self {canvas, scale_factor}
    }

    pub fn reset(&mut self) {
        self.canvas.set_draw_color(COLOR_BG);
        self.canvas.clear();
    }

    pub fn show(&mut self) {
        self.canvas.present();
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32) {
        let pixel = Rect::new(x*self.scale_factor as i32,
                              y*self.scale_factor as i32,
                              self.scale_factor, self.scale_factor);
        self.canvas.set_draw_color(COLOR_FG);
        self.canvas.fill_rect(pixel).unwrap();
    }


}

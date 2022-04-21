pub const BASE_WIDTH: u32 = 64;
pub const BASE_HEIGHT: u32 = 32;
pub const BUFFER_SIZE: usize = BASE_WIDTH as usize * BASE_HEIGHT as usize;
pub trait ScreenDisplay {
    fn draw(&mut self, buffer: [u8; BUFFER_SIZE]);
}

use minifb::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

// Puts 3 color variables r, g and b in 32-bit format for the buffer 
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32{
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

struct Vector2 {
    x: u32,
    y: u32
}

impl Vector2 {
    fn new(x: u32, y: u32) -> Self{
        Vector2 {x, y}
    }
}

fn draw_triangle( position: Vector2, point1: u32, point2: u32) -> Vec<u32>{

}

fn main(){
    
    // Creation of the window
    let mut window: Window = Window::new(
        "EGP-Designer", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();


    // Creation of the buffer
    let mut buffer: Vec<u32> = vec![from_u8_rgb(0, 0, 0); WIDTH * HEIGHT ];

    // Loop while the window is open
    while window.is_open() {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }


}
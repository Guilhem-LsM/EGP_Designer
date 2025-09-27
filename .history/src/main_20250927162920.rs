use minifb::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32{
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main(){
    
    let mut window: Window = Window::new(
        "EGP-Designer", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();



    let mut buffer: Vec<u32> = vec![from_u8_rgb(0, 0, 0); WIDTH * HEIGHT ];

    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();


}
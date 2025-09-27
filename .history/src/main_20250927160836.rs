use minifb::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main(){

    let mut window: Window = Window::new(
        "EGP-Designer", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();



    let mut buffer: Vec<u32> = vec![0 as u32, WIDTH as u32, HEIGHT as u32];

    while true {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }


}
use minifb::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main(){

    let WIDTH_BUFFER = 640;
    let HEIGHT_BUFFER = 640;
    

    let mut window: Window = Window::new(
        "EGP-Designer", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();



    let mut buffer: Vec<u32> = vec![0 as u32, WIDTH_BUFFER, HEIGHT_BUFFER as u32];

    while true {
        window.update_with_buffer(&buffer, WIDTH_BUFFER as usize, HEIGHT_BUFFER as usize).unwrap();
    }


}
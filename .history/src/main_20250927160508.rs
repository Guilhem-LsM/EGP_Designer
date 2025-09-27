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

    let mut buffer = vec![0, WIDTH*HEIGHT];

    while true {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }


}
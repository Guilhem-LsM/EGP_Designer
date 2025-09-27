use minifb::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main(){

    let mut window: Window = Window::new(
        "Test", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();

    while true {
        window.update();
    }


}
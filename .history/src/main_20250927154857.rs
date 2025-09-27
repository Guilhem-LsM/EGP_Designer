use minifb::*;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;

fn main(){

    let mut window: ! = Window::new(
        "Test", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();

    window.update();


}
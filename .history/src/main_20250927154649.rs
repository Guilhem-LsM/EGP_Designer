use minifb::{Window, WindowOptions};

const WIDTH: unsize = 640;
const HEIGHT: unsize = 640;

fn main(){
    
    let mut window = match Window::new(
        "Test", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();


}
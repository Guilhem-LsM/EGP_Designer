use minifb::*;

fn main(){
    
    let mut window = match Window::new("Test", 640, 400, WindowOptions::default()).unwrap() {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    while true {}

}
use minifb_canvas::*;
use rand::*;
use std::time::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main(){
    
    // Creation of the window
    let mut window: Window = Window::new(
        "EGP-Designer", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    
    ).unwrap();

    let mut canvas_1:Canvas = Canvas::new(WIDTH as u32, HEIGHT as u32, Color::new(50, 50, 50));
    let mut shape_list:Vec<Shape> = vec![];
    let mut rng = rand::thread_rng();
    let range:i32 = 200;
    let mut start = Instant::now();
    for _i in 0..100 {
        shape_list.push(Shape::Triangle { 
            position: Vec2::new(rng.gen_range(WIDTH/4..WIDTH/4*3) as i32, rng.gen_range(HEIGHT/4..HEIGHT/4*3) as i32), 
            vertex_1: Vec2::new(rng.gen_range(-range..range) as i32, rng.gen_range(-range..range) as i32), 
            vertex_2: Vec2::new(rng.gen_range(-range..range) as i32, rng.gen_range(-range..range) as i32), 
            vertex_3: Vec2::new(rng.gen_range(-range..range) as i32, rng.gen_range(-range..range) as i32), 
            color: Color::new(rng.gen_range(0..255) as u8, rng.gen_range(0..255) as u8, rng.gen_range(0..255) as u8)
        });

    }   

    // Loop while the window is open
    while window.is_open() {
        start = Instant::now();
        canvas_1.draw_shapes(&shape_list);
        println!("{:?}",start.elapsed());
        window.update_with_buffer(canvas_1.as_buffer(), WIDTH, HEIGHT).unwrap();
    }


}
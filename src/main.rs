use minifb_canvas::*;

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
    canvas_1.push_shape(Shape::Triangle { 
        position: Vec2::new(320, 320), 
        vertex_1: Vec2::new(0, -200), 
        vertex_2: Vec2::new(100, 200), 
        vertex_3: Vec2::new(-100, 200), 
        color: Color::new(255, 0, 0)
    });

    // Loop while the window is open
    while window.is_open() {
        let pos:Vec2 = canvas_1.shapes_list[0].get_position();
        canvas_1.shapes_list[0].set_position(Vec2::add(pos, Vec2::new(1, 0)));
        canvas_1.clear_frame_buffer();
        canvas_1.draw_shapes();
        window.update_with_buffer(canvas_1.as_buffer(), WIDTH, HEIGHT).unwrap();
    }


}
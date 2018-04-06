#[macro_use]
extern crate glium;
extern crate rand;
extern crate image;
extern crate noise;
extern crate cgmath;

use glium::{glutin,Surface};
use std::{thread, time};

mod shaders;
mod camera;
mod landmass;
mod keyboard;

use keyboard::Keyboard;

use shaders::{VERTEX_SHADER_SRC,FRAGMENT_SHADER_SRC};
use camera::Camera;

fn main() {
    let height = 256;
    let width = 256;
    let params = landmass::MapParameters::new(height.clone(),width.clone(),2.0,8,0.8,1.5);
    let wm = landmass::WorldMap::new(params.clone());
    let obj = wm.as_model_object();

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("LANDMASS");
        // .with_decorations(false)
        // .with_fullscreen(Some(events_loop.get_primary_monitor()));
    //.with_decorations(false);        
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let gl_win = glutin::GlWindow::new(window,context,&events_loop).unwrap();
    gl_win.set_cursor_state(glutin::CursorState::Normal).unwrap();    
    let display = glium::Display::from_gl_window(gl_win).unwrap();
    
    let positions = glium::VertexBuffer::new(&display,&obj.vertices).unwrap();
    let normals = glium::VertexBuffer::new(&display,&(obj.normals.unwrap())).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &(obj.index.unwrap())).unwrap();
    
    let program = glium::Program::from_source(&display,
                                              &VERTEX_SHADER_SRC,
                                              &FRAGMENT_SHADER_SRC,
                                              None).unwrap();
    let light = [2.0f32, -1.0, 0.5f32];
    
    let mut closed = false;
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };
    let thirteen_millis = time::Duration::from_millis(1000/60);
    let mut cam = Camera::new([00.0, 10.0, 0.0],[0.0, 0.0, 1.0]);
    let mut keydown = Keyboard::new();
    let mut previous = (0.0,0.0);
    while !closed {
        let mut target = display.draw();
        let view = cam.view_matrix();
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            // let fov: f32 = 3.141592 / 3.0;
            let fov: f32 = 3.141592 / 2.0;
            let zfar = 2048.0;
            let znear = 0.01;

            let f = 1.0 / (fov / 2.0).tan();
            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };        
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-256.0, 0.0, -256.0, 1.0f32]
        ];

        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model,
                                u_light: light,
                                view: view,
                                perspective: perspective},
                    &params).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::CursorMoved{position,modifiers, ..} =>
                        {previous = mouse_move(previous,position,modifiers);},
                    glutin::WindowEvent::KeyboardInput{input, ..} =>{
                        keydown.key_input(input)},
                    _ => ()
                },
                _ => (),
            }
        });
        cam = simple_eval(&keydown, cam);
        keydown.reset_triggers();
        // println!("{:?}",cam);
        thread::sleep(thirteen_millis);
    }
}

fn mouse_move(_previous: (f64,f64), position: (f64,f64), _modifiers: glutin::ModifiersState) -> (f64,f64){
    // println!("{:?}, {:?}",position,modifiers);
    // let (prex,prey) = previous;
    // let (x,y) = position;
    // if x - prex < 0.0  {
    //     // println!("right");
    // } else if  x - prex > 0.0 {
    //     // println!("left");
    // };
    position
}

// add mouse information
fn simple_eval(keys: &Keyboard, cam: Camera)->Camera{
    let speed = 2.0;
    let mv = 0.5;
    let mut cam3 = cam.clone();
    cam3 = match keys.escape.trigger_pressed{
        true => {
            Camera::new([00.0, 10.0, 0.0],[0.0, 0.0, 1.0])},
        false => cam3            
    };
    cam3 = match keys.down.state{
        true => cam3.rotate(speed,0.0),
        false => cam3
    };
    cam3 = match keys.up.state{
        true => cam3.rotate(-speed,0.0),
        false => cam3
    };
    cam3 = match keys.right.state{
        true => cam3.rotate(0.0,speed),
        false => cam3
    };
    cam3 = match keys.left.state{
        true => cam3.rotate(0.0,-speed),
        false => cam3
    };
    cam3 = match keys.w.state{
        true => cam3.forward(mv),
        false => cam3
    };
    cam3 = match keys.s.state{
        true => cam3.forward(-mv),
        false => cam3
    };
    cam3 = match keys.a.state{
        true => cam3.right(-mv),
        false => cam3
    };
    cam3 = match keys.d.state{
        true => cam3.right(mv),
        false => cam3
    };    

    cam3

}

    
// Can look at keydown and and direction and determine how to update position
// similarily it could look at mousepos and determine how to update direction
// fn _move_camera ( _position: &mut [f32;3] , _direction: &mut [f32;3]){
//     let velocity = 10.0;
//     let (x,y) = keydown_to_v2(_keydown,_direction,velocity);
//     //_position[0]+=
    
// }

// Take this out and flesh out a keyboard class. The keyboard will be a state machine
// that the controler builds off of. 


// add functions to be called durring input stream

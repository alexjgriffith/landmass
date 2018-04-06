#[macro_use]
extern crate glium;
extern crate rand;
extern crate image;
extern crate noise;
extern crate cgmath;

use glium::{glutin,Surface};
use std::{thread, time};

mod camera;
mod landmass;

use camera::Camera;

fn main() {
    let height = 256;
    let width = 256;
    let params = landmass::MapParameters::new(height.clone(),width.clone(),2.0,8,0.8,1.5);
    let wm = landmass::WorldMap::new(params.clone());
    let obj = wm.as_model_object();

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Landmass");
    // .set_cursor_state(glutin::CursorState::Normal);
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window,context,&events_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display,&obj.vertices).unwrap();
    let normals = glium::VertexBuffer::new(&display,&(obj.normals.unwrap())).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &(obj.index.unwrap())).unwrap();
       
    let vertex_shader_src = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        flat out vec3 v_normal;
        out vec3 v_position;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;
        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position =   perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

    let fragment_shader_src = r#"
        #version 150
        flat in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;
        const vec3 ambient_color = vec3(0.1, 0.0, 0.1);
        const vec3 diffuse_color = vec3(0.3, 0.05, 0.1);
        const vec3 specular_color = vec3(1.0, 1.0, 1.0);        
        uniform vec3 u_light;
        void main() {
             float diffuse = max(dot(normalize(v_normal), normalize(u_light)),0.0);
             vec3 camera_dir = normalize(-v_position);
             vec3 half_direction = normalize(normalize(u_light)+camera_dir);
             float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
             color = vec4(ambient_color+diffuse*diffuse_color + specular*specular_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();
    let light = [-2.0f32, -1.0, 0.0f32];
    
    let mut closed = false;
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        .. Default::default()
    };
    let thirteen_millis = time::Duration::from_millis(1000/60);
    // let mut position = [0.0, 0.0, 0.0]; //[2.0, -1.0, 1.0];
    // let mut camerav = [1.0, 0.0, 1.0]; //&[-2.0, 1.0, 1.0]
    let mut cam = Camera::new([00.0, 50.0, 0.0],[0.0, 0.0, 1.0]);
    let mut keydown = Keyboard::new();
    let mut t = 0.1;
    while !closed {
        let mut target = display.draw();
        //cam = cam.rotate(t,t);
        let view = cam.view_matrix();//camera::view_matrix(&position, &camerav, &[0.0,1.0,0.0] );
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
            [0.0, 0.0, 0.0, 1.0f32]
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
                        mouse_move(position,modifiers),
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

fn simple_eval(keys: &Keyboard, cam: Camera)->Camera{
    let speed = 2.0;
    let mv = 0.5;
    let mut cam3 = cam.clone();
    cam3 = match keys.escape.trigger_pressed{
        true => {
            cam3.flip()},
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
#[allow(dead_code)]
#[derive(Debug,Clone)]
struct Key {
    state: bool,
    trigger_pressed: bool,
    trigger_release: bool,
//    modifier: glutin::ModifiersState
}


impl Key {
    fn new () -> Key{
        let state = false;
        let triggered = false;
        Key{state:false,trigger_pressed:false, trigger_release:false}
    }

    fn pressed(&mut self) {
        self.state = true ;
        self.trigger_pressed = true;
    }
    fn release(&mut self){
        self.state = false ;
        self.trigger_release = true;
    }
}

#[derive(Debug)]
struct Keyboard {
    down: Key,
    left: Key,
    right: Key,
    up: Key,
    w: Key,
    s: Key,
    a: Key,
    d: Key,    
    escape: Key,
}


fn mouse_move(position :(f64,f64),_modifiers: glutin::ModifiersState) {
    // println!("{:?}",position);
}

impl Keyboard {
    fn new () -> Keyboard{
        Keyboard{down: Key::new(),
                 left: Key::new(),
                 right: Key::new(),
                 up: Key::new(),
                 w: Key::new(),
                 s: Key::new(),
                 a: Key::new(),
                 d: Key::new(),                 
                 escape: Key::new()}
    }

    fn reset_triggers(&mut self){
        self.escape.trigger_pressed=false;
    }
    // this can be generalized with a macro
    fn key_input (&mut self,input: glutin::KeyboardInput) {    
        use glutin::VirtualKeyCode::*;
        use glutin::ElementState::*;
        match (input.virtual_keycode.unwrap(),input.state) {            
            (Escape,Pressed) => {
                self.escape.pressed();},
            (Escape,Released) => {
                self.escape.release();},            
            (Down,Pressed) => {
                self.down.pressed();
            },
            (Down,Released) =>{
                self.down.release();
            },        
            (Left,Pressed) => {
                self.left.pressed();
            },        
            (Left,Released) => {
                self.left.release();
            },
            (Right,Pressed) => {
                self.right.pressed();
            },
            (Right,Released) => {
                self.right.release();
            },
            (Up,Pressed) => {
                self.up.pressed();
            },
            (Up,Released) => {
                self.up.release();
            },
            (W,Pressed) => {
                self.w.pressed();
            },
            (W,Released) => {
                self.w.release();
            },
            (S,Pressed) => {
                self.s.pressed();
            },
            (S,Released) => {
                self.s.release();
            },
            
            (A,Pressed) => {
                self.a.pressed();
            },
            (A,Released) => {
                self.a.release();
            },
            (D,Pressed) => {
                self.d.pressed();
            },
            (D,Released) => {
                self.d.release();
            },


            (_,_) => ()
        }
    }

}

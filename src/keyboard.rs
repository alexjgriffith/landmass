use glium::glutin;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Key {
    pub state: bool,
    pub trigger_pressed: bool,
    pub trigger_release: bool,
//    modifier: glutin::ModifiersState
}


impl Key {
    pub fn new () -> Key{        
        Key{state:false,trigger_pressed:false, trigger_release:false}
    }

    pub fn pressed(&mut self) {
        self.state = true ;
        self.trigger_pressed = true;
    }
    pub fn release(&mut self){
        self.state = false ;
        self.trigger_release = true;
    }
}

#[derive(Debug)]
pub struct Keyboard {
    pub down: Key,
    pub left: Key,
    pub right: Key,
    pub up: Key,
    pub w: Key,
    pub s: Key,
    pub a: Key,
    pub d: Key,    
    pub escape: Key,
}

impl Keyboard {
    pub fn new () -> Keyboard{
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

    pub fn reset_triggers(&mut self){
        self.escape.trigger_pressed=false;
    }
    // this can be generalized with a macro
    pub fn key_input (&mut self,input: glutin::KeyboardInput) {    
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

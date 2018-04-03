implement_vertex!(Vertex, position);
implement_vertex!(Normal, normal);


#[derive(Debug,Clone,Copy)]
pub struct Vertex {
    pub position: (f32,f32,f32)
}

impl Vertex{
    pub fn uniform (&self) -> (f32,f32,f32){
        let (x,y,z) = self.position;
        let mag2 = x*x + y*y + z*z;
        let mag = mag2.sqrt();
        if mag == 0.0{
            (0.0,0.0,0.0)
        } else{
            (x/mag,y/mag,z/mag)
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Normal {
    pub normal: (f32,f32,f32)
}

#[derive(Debug,Clone)]
pub struct Object{
    pub vertices : Vec<Vertex>,
    pub normals  : Option<Vec<Normal>>,
    pub index    : Option<Vec<u16>>,
    pub material : Option<&'static str>,    
    pub bones    : Option<bool> // add later
}

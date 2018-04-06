use cgmath::{Deg,Rad,Vector3, Quaternion};
use cgmath::prelude::*;

#[derive(Debug,Copy,Clone)]
pub struct Camera{
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new(pos: [f32;3], dir: [f32;3]) -> Camera{
        let pos = Vector3::from(pos);
        let dir = Vector3::from(dir);
        let up = Vector3::new(0.0,1.0,0.0);
        Camera{pos,dir,up}
    }    
    
    pub fn view_matrix(&self) -> [[f32;4];4]{
        let f = self.dir.normalize();
        let s = self.up.cross(f);
        let s_norm = s.normalize();
        let u = f.cross(s_norm);
        let pos = self.pos;
        let p = [-pos[0] * s_norm[0] - pos[1] * s_norm[1] - pos[2] * s_norm[2],
                 -pos[0] * u[0] - pos[1] * u[1] - pos[2] * u[2],
                 -pos[0] * f[0] - pos[1] * f[1] - pos[2] * f[2]];
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0],      p[1], p[2], 1.0],
        ]
    }

    #[allow(dead_code)]
    pub fn flip(mut self) -> Camera{
        let p = Quaternion{s:0.0,v:self.dir}.normalize();        
        let quat = Quaternion::from_axis_angle(Vector3::new(0f32,0f32,1f32),
                                                Rad::from(Deg(90.0)));
        let quat_t = quat.conjugate();
        let Quaternion {s:_,v:vout} = quat * p * quat_t;
        self.dir =  vout;
        self        
    }
    
    pub fn rotate (mut self,pitch: f32,yaw: f32) -> Camera {

        let right = self.up.cross(self.dir);
        let yaw_q = Quaternion::from_axis_angle(self.up,
                                                Rad::from(Deg(yaw)));
        let pitch_q = Quaternion::from_axis_angle(right,
                                                  Rad::from(Deg(pitch))).normalize();
        
        self.dir = {
            let p = Quaternion{s:0.0,v:self.dir}.normalize();
            let quat = yaw_q ;
            let quat_t = quat.conjugate();
            let Quaternion {s:_,v:vout} = quat * p * quat_t;
            vout};

        self.dir = {
            let p = Quaternion{s:0.0,v:self.dir}.normalize();
            let quat = pitch_q ;
            let quat_t = quat.conjugate();
            let Quaternion {s:_,v:vout} = quat * p * quat_t;
            vout};

        

        // let rot = Vector3::from_axis_angle(Vector3::new(0f32,1f32,0f32),
        //                                      Rad::from(Deg(yaw)));
        // println!("{:?}",rot);
        // self.dir = self.dir.normalize();
        self
    }

    pub fn forward (mut self,step: f32) -> Camera {
        self.pos = self.pos + (self.dir * step);
        self
    }

    pub fn right (mut self, step: f32) -> Camera {
        let s = self.up.cross(self.dir);
        self.pos = self.pos + (s * step);
        self
    }
}

use rand;
use noise::{Perlin,Seedable,NoiseFn};
use image::{self,ImageBuffer};
use std::path::Path;

use std;

#[path = "../model/mod.rs"]
mod model;

#[derive(Debug,Clone,Copy)]
pub struct MapParameters {
    width: i32,
    height: i32,
    scale:f32,
    levels: i32,
    scale_ratio: f32,
    freq_ratio: f32,
}

impl MapParameters {
    pub fn new(width: i32, height: i32, scale: f32, levels: i32, scale_ratio: f32, freq_ratio: f32) -> MapParameters{
        MapParameters{width,height,scale,levels,scale_ratio,freq_ratio}
    }

    pub fn xy(&self,perlin: &Perlin, x: &f64,y: &f64) -> f64{
        let mut z:f64 = 0.0;
        let xp = *x / (self.width as f64);
        let yp = *y / (self.height as f64);
        for i in 0..self.levels{
            let f : f64 = self.freq_ratio.powi(i) as f64;  // lac^i
            let a : f64 = self.scale_ratio.powi(i) as f64; // per^i
            z+=a*perlin.get([xp*f,yp*f]) ;            
        }
        z
    }
}

#[derive(Debug)]
pub struct WorldMap{
    pub parameters: MapParameters,
    pub seed: u32,
    pub vertecies: Vec<model::object::Vertex>
}

impl WorldMap{
    pub fn new(params: MapParameters) -> WorldMap{
        let seed = rand::random();
        let perlin = Perlin::new();
        let perlin = perlin.set_seed(seed);
        let mut vertecies = Vec::with_capacity((params.width * params.height) as usize);
        for i in 0..params.width{
            for j in 0..params.height{
                let x: f32 = (j as f32) * params.scale;
                let z: f32 = (i as f32) * params.scale;
                let y: f32 = params.xy(&perlin,&(x as f64),&(z as f64)) as f32;
                vertecies.push(model::object::Vertex{position:(x,y*10.0,z)});
            }        
        }
        WorldMap{
            parameters: params,
            seed: seed,
            vertecies: vertecies
        }
    }
    
    fn height_map (&self) -> Vec<f32> {
        let mut vertecies = Vec::with_capacity((self.parameters.width *
                                                self.parameters.height) as usize);
        for i in 0..self.parameters.height{
            for j in 0..self.parameters.width{
                let model::object::Vertex{position: (_,_,z)}=
                    self.vertecies[(i*self.parameters.width + j) as usize];
                vertecies.push(z)
            }
        }
        vertecies
    }
    
    #[allow(dead_code)]
    pub fn visualize(&self,filename: std::string::String){
        let heights = normalize(self.height_map());
        let height = self.parameters.height;
        let width = self.parameters.width;
        let img = ImageBuffer::from_fn(height.clone() as u32,width.clone() as u32,|x,y|{
            image::Luma([heights[((width.clone() as u32)*y+x) as usize] as u8])
        });
        let name = filename;
        let path = Path::new(&name);
        img.save(path).unwrap();
    }
    
    pub fn as_model_object(&self) -> model::object::Model{
        let width = self.parameters.width;
        let height = self.parameters.height;
        // define indicies
        let mut ind = Vec::with_capacity(((width - 1)*(height - 1)*3*2) as usize);
        let mut norms = Vec::with_capacity((width * height ) as usize);
        let zero_norm = model::object::Normal{normal:(0.0f32,0.0,0.0)};
        for _i in 0..(width*height){
            norms.push(zero_norm);
        }
        for i in 0..(height -1){
            for j in 0..(width -1){
                let offset = width*i+j;
                let mut x = vec!((offset+1) as u16,
                                 (offset+width+1) as u16,
                                 offset as u16);
                let mut y = vec!(offset as u16,                                 
                                 (offset+width+1) as u16,
                                 (offset+width) as u16,);
                let mut normal = cross_product(&self.vertecies[x[0] as usize],
                                               &self.vertecies[x[1] as usize],
                                               &self.vertecies[x[2] as usize]);
                for n in x.clone(){                    
                    norms[n as usize]=normal;
                }
                for n in y.clone(){
                    norms[n as usize]=normal;
                }                                

                ind.append(&mut x);
                ind.append(&mut y);
            }
        }
        model::object::Model{
            vertices: self.vertecies.clone(),
            normals: Some(norms),
            index: Some(ind),
            material: None,
            bones: None,
        }
        
    }
}

fn cross_product(a: &model::object::Vertex, b: &model::object::Vertex, c: &model::object::Vertex) ->  model::object::Normal{
    let (ax,ay,az) = a.uniform();
    let (bx,by,bz) = b.uniform();
    let (cx,cy,cz) = c.uniform();
    let (abx,aby,abz) = (ax - bx, ay - by, az - bz);
    let (cbx,cby,cbz) = (cx - bx, cy - by, cz - bz);    
    let norm = ((aby*cbz - abz*cby),
                (abz*cbx - abx*cbz),
                (abx*cby - aby*cbx));
    model::object::Normal{normal:norm}
}

#[allow(dead_code)]
fn normalize(vector : Vec<f32>)->Vec<u8>{
    let max = vector.iter().cloned().fold(0./0., f32::max);
    let min = vector.iter().cloned().fold(0./0., f32::min);
    vector.iter().map(|v|{
        ((v - min) / (max - min) * 256.0) as u8
    }).collect::<Vec<u8>>()
}

#[allow(dead_code)]
fn batch_gen(height: i32, width: i32){
    let params = MapParameters::new(height.clone(),width.clone(),1.0,5,0.8,1.4);
    for i in 0..5{
        let wm = WorldMap::new(params.clone());
        let file = format!("test_{}.png",i.to_string());
        wm.visualize(file)
    }

}

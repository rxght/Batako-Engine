use std::sync::Arc;
use cgmath::Rad;
use cgmath::Vector3;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::{event_loop::EventLoop, window::Window};
use crate::graphics::Graphics;
use crate::graphics::bindable;
use crate::graphics::drawable::{DrawableEntry, self};

use self::drawables::Ubo;

mod drawables {
    pub mod triangle;
    pub mod cube;
    mod ubotest; pub use ubotest::*;
}

pub struct App
{
    start_time: std::time::Instant,
    cube: drawables::cube::Cube,
}

impl App
{
    pub fn new(gfx: &mut Graphics) -> Self
    {
        Self {
            start_time: std::time::Instant::now(),
            cube: drawables::cube::Cube::new(gfx, true)
        }
    }
    
    pub fn resize_callback(&mut self)
    {
        
    }

    pub fn run(&mut self)
    {
        let time = (std::time::Instant::now() - self.start_time).as_secs_f32();

        let mut uniform_buffer =
            self.cube.uniform.subbuffer.write().unwrap();

        // change the model matrix
        uniform_buffer.model = 
            cgmath::Matrix4::from_angle_y(Rad(time)).into();
    }
}
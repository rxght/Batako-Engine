use std::sync::Arc;

use rand::distributions::uniform;
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, shader::ShaderStages};
use crate::graphics::{drawable::{DrawableEntry, GenericDrawable}, Graphics, bindable::{self, UniformBuffer}, shaders::{vert_3dColored, frag_3dColored}};
use cgmath::{prelude::*, Matrix4, Perspective, PerspectiveFov, Point3, Vector3, Rad, Deg};

pub use vert_3dColored::Ubo;

pub struct Cube
{
    pub entry: DrawableEntry,
    pub uniform: Arc<UniformBuffer<Ubo>>
}

impl Cube
{
    pub fn new(gfx: &mut Graphics, create_registered: bool) -> Cube
    {
        let window_extent = gfx.get_window().inner_size();
        let aspect = window_extent.width as f32 / window_extent.height as f32;
        let uniform = UniformBuffer::new(gfx, 0, Ubo {
            model: cgmath::Matrix4::identity().into(),
            view: cgmath::Matrix4::look_at_rh(
                Point3{x: 0.0, y: 1.0, z: 2.0},
                Point3{x: 0.0, y: 0.0, z: 0.0},
                Vector3{x: 0.0, y: -1.0, z: 0.0}
            ).into(),
            proj: cgmath::perspective(Deg(90.0), aspect, 0.5, 10.0).into()
        }, ShaderStages::VERTEX);

        let mut entry = GenericDrawable::new(gfx, 1, || {
            vec![
                uniform.clone()
            ]
        }, || {
            #[derive(BufferContents, Vertex)]
            #[repr(C)]
            struct Vertex {
                #[format(R32G32B32_SFLOAT)]
                pub pos: [f32; 3],
                #[format(R32G32B32_SFLOAT)]
                pub color: [f32; 3],
            }
            let vertices: Vec<Vertex> = vec![
                Vertex{pos: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0]},
                Vertex{pos: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 1.0]},
                Vertex{pos: [ 0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0]},
                Vertex{pos: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0]},

                Vertex{pos: [-0.5, -0.5,  0.5], color: [0.0, 0.0, 1.0]},
                Vertex{pos: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0]},
                Vertex{pos: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0]},
                Vertex{pos: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 0.0]},
            ];

            let indices: Vec<u32> = vec![
                // front
                1, 0, 2,    1, 2, 3,
                
                // back
                6, 4, 5,    5, 7, 6,

                // left
                4, 0, 1,    4, 5, 1,

                // right
                3, 2, 6,    3, 7, 6,

                // top
                5, 1, 3,    5, 7, 3,

                // bottom
                4, 0, 2,    4, 6, 2,
            ];
            
            vec![
                bindable::VertexShader::from_module(vert_3dColored::load(gfx.get_device()).unwrap()),
                bindable::FragmentShader::from_module(frag_3dColored::load(gfx.get_device()).unwrap()),
                bindable::IndexBuffer::new(&gfx, indices),
                bindable::VertexBuffer::new(&gfx, vertices),
            ]
        });

        if create_registered {
            gfx.register_drawable(&mut entry);
        }

        Self {
            entry: entry,
            uniform: uniform,
        }
    }
}
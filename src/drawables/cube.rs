use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};
use crate::graphics::{drawable::{DrawableEntry, GenericDrawable}, Graphics, bindable, shaders::{vert_3dColored, frag_3dColored}};
use cgmath::{prelude::*, Matrix4, Perspective, PerspectiveFov};

pub fn new(gfx: &Graphics) -> DrawableEntry
{
    GenericDrawable::new(gfx, 1, || {
        vec![] // non-shared bindables (ex. UniformBuffer)
    }, || {
        #[derive(BufferContents, Vertex)]
        #[repr(C)]
        struct Vertex {
            #[format(R32G32B32_SFLOAT)]
            pub pos: [f32; 3],
            #[format(R32G32B32_SFLOAT)]
            pub col: [f32; 3],
        }
        let vertices: Vec<Vertex> = vec![
            Vertex{pos: [-0.5, -0.5, -0.5], col: [1.0, 1.0, 0.0]},
            Vertex{pos: [-0.5,  0.5, -0.5], col: [0.0, 1.0, 1.0]},
            Vertex{pos: [ 0.5, -0.5, -0.5], col: [1.0, 0.0, 1.0]},
            Vertex{pos: [ 0.5,  0.5, -0.5], col: [1.0, 1.0, 1.0]},

            Vertex{pos: [-0.5, -0.5,  0.5], col: [0.0, 0.0, 1.0]},
            Vertex{pos: [-0.5,  0.5,  0.5], col: [1.0, 0.0, 0.0]},
            Vertex{pos: [ 0.5, -0.5,  0.5], col: [0.0, 1.0, 0.0]},
            Vertex{pos: [ 0.5,  0.5,  0.5], col: [0.0, 0.0, 0.0]},
        ];

        let indices: Vec<u32> = vec![
            // front
            0, 1, 2,    1, 3, 2,
            
            // back
            4, 6, 5,    5, 6, 7,

            // left
            0, 4, 1,    4, 5, 1,

            // right
            2, 3, 6,    3, 7, 6,

            // top
            1, 5, 3,    5, 7, 3,

            // bottom
            0, 4, 2,    4, 6, 2,
        ];
        
        vec![
            bindable::VertexShader::from_module(vert_3dColored::load(gfx.get_device()).unwrap()),
            bindable::FragmentShader::from_module(frag_3dColored::load(gfx.get_device()).unwrap()),
            bindable::IndexBuffer::new(&gfx, indices),
            bindable::VertexBuffer::new(&gfx, vertices),
        ]
    })
}
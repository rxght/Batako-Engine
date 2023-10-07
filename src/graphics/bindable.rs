use std::sync::Arc;

use vulkano::{shader::ShaderModule, command_buffer::{AutoCommandBufferBuilder, allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer}, pipeline::PipelineLayout};

use super::{pipeline::PipelineBuilder, Graphics};

mod buffer;
mod shader;
mod uniform;
mod texture;
mod push_constant;

pub use buffer::*;
pub use shader::*;
pub use uniform::*;
pub use texture::*;
pub use push_constant::*;

pub trait Bindable
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, index_count: &mut u32);
    fn bind(&self, _gfx: &Graphics,
        _builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>,
        _pipeline_layout: Arc<PipelineLayout>
    ) {}
}
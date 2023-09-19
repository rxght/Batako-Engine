
use std::sync::Arc;
use vulkano::{pipeline::{GraphicsPipeline, graphics::{GraphicsPipelineBuilder, vertex_input::{VertexInputState, VertexBufferDescription}, input_assembly::InputAssemblyState, viewport::{ViewportState, Viewport}, color_blend::ColorBlendState, rasterization::RasterizationState, depth_stencil::DepthStencilState, discard_rectangle::DiscardRectangleState, multisample::MultisampleState, tessellation::TessellationState, render_pass::PipelineRenderingCreateInfo}}, device::Device, render_pass::RenderPass, shader::{ShaderModule, SpecializationConstants}};

use super::Graphics;

pub struct PipelineBuilder
{
    pub rendering_create_info: PipelineRenderingCreateInfo,
    pub vertex_buffer_description: Option<VertexBufferDescription>,
    pub input_assembly_state: InputAssemblyState,
    pub vertex_shader: Option<Arc<ShaderModule>>,
    pub fragment_shader: Option<Arc<ShaderModule>>,
    pub viewport_state: ViewportState,
    pub color_blend_state: ColorBlendState,
    pub rasterization_state: RasterizationState,
    pub depth_stencil_state: DepthStencilState,
    pub discard_rectangle_state: DiscardRectangleState,
    pub multisample_state: MultisampleState,
    pub tessellation_state: TessellationState,
}

impl PipelineBuilder
{
    pub fn new(gfx: &Graphics) -> Self
    {
        Self {
            rendering_create_info: PipelineRenderingCreateInfo {
                color_attachment_formats: vec![Some(gfx.get_swapchain_format())],
                ..Default::default()
            },
            vertex_buffer_description: None,
            input_assembly_state: InputAssemblyState::new(),
            vertex_shader: None,
            fragment_shader: None,
            viewport_state: ViewportState::viewport_dynamic_scissor_irrelevant(),
            color_blend_state: ColorBlendState::default(),
            rasterization_state: RasterizationState::default(),
            depth_stencil_state: DepthStencilState::default(),
            discard_rectangle_state: DiscardRectangleState::default(),
            multisample_state: MultisampleState::default(),
            tessellation_state: TessellationState::default()
        }
    }

    pub fn build(self, device: Arc<Device>) -> Arc<GraphicsPipeline>
    {
        let vertex_shader_entry = self.vertex_shader.as_ref()
            .expect("No vertex shader supplied.")
            .entry_point("main").unwrap();

        let fragment_shader_entry = self.fragment_shader.as_ref()
            .expect("No fragment shader supplied.")
            .entry_point("main").unwrap();

        GraphicsPipeline::start()
            .render_pass(self.rendering_create_info)
            .vertex_input_state(self.vertex_buffer_description.unwrap())
            .input_assembly_state(self.input_assembly_state)
            .vertex_shader(vertex_shader_entry, ())
            .fragment_shader(fragment_shader_entry, ())
            .viewport_state(self.viewport_state)
            .color_blend_state(self.color_blend_state)
            .rasterization_state(self.rasterization_state)
            .depth_stencil_state(self.depth_stencil_state)
            .discard_rectangle_state(self.discard_rectangle_state)
            .multisample_state(self.multisample_state)
            .tessellation_state(self.tessellation_state)
            .build(device).expect("Failed to create pipeline!")
    }
}

use crate::Context;
use ash::version::DeviceV1_0;
use ash::{self, vk};
use std::ptr;

// Currently it's both command encoder and render pass encoder
// TODO 
// separate it somehow 
// create from device (currently it's created from Context mega struct)
// inspiration: 
// https://github.com/gfx-rs/wgpu-rs/blob/96b70210a87307158da37d0ee8ba69e47318aef8/examples/cube/main.rs#L381
// https://gpuweb.github.io/gpuweb/#render-pass-encoder

pub struct Buffer {
    buffer: vk::Buffer,
    buffer_memory: vk::DeviceMemory,
}

// #[derive(Default)]
struct Encoder {
    command_buffers: Vec<vk::CommandBuffer>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_length: u32,
    // vertex_buffer: vk::Buffer,
    // vertex_buffer_memory: vk::DeviceMemory,
    // index_buffer: vk::Buffer,
    // index_buffer_memory: vk::DeviceMemory,
}

impl Encoder {
    pub fn new(
        ctx: &Context, 
        vertex_buffer: Buffer, 
        index_buffer: Buffer,
        index_length: u32,
    ) -> Self {
        let command_buffers = Encoder::create_command_buffers(
            &ctx.device,
            ctx.command_pool,
            ctx.graphics_pipeline,
            &ctx.swapchain_framebuffers,
            ctx.render_pass,
            ctx.swapchain_extent,
            vertex_buffer.buffer,
            index_buffer.buffer,
            ctx.pipeline_layout,
            &ctx.descriptor_sets,
        );
        Encoder {
            command_buffers,
            vertex_buffer,
            index_buffer,
            index_length
        }
    }

    // fn set_vertex_buffer(&mut self, buffer: Buffer) {
    //     self.vertex_buffer = Some(buffer);
    // }

    // fn set_index_buffer(&mut self, buffer: Buffer, length: usize) {
    //     self.index_buffer = Some(buffer);
    //     self.index_length = Some(length);
    // }

    fn build_command_buffers(&mut self) {

    }

    fn create_command_buffers(
        device: &ash::Device,
        command_pool: vk::CommandPool,
        graphics_pipeline: vk::Pipeline,
        framebuffers: &Vec<vk::Framebuffer>,
        render_pass: vk::RenderPass,
        surface_extent: vk::Extent2D,
        vertex_buffer: vk::Buffer,
        index_buffer: vk::Buffer,
        pipeline_layout: vk::PipelineLayout,
        descriptor_sets: &Vec<vk::DescriptorSet>,
    ) -> Vec<vk::CommandBuffer> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_buffer_count: framebuffers.len() as u32,
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
        };

        let command_buffers = unsafe {
            device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Failed to allocate Command Buffers!")
        };

        for (i, &command_buffer) in command_buffers.iter().enumerate() {
            let command_buffer_begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: ptr::null(),
                p_inheritance_info: ptr::null(),
                flags: vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
            };

            unsafe {
                device
                    .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                    .expect("Failed to begin recording Command Buffer at beginning!");
            }

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];

            let render_pass_begin_info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass,
                framebuffer: framebuffers[i],
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: surface_extent,
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
            };

            unsafe {
                device.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                );
                device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphics_pipeline,
                );

                let vertex_buffers = [vertex_buffer];
                let offsets = [0_u64];
                let descriptor_sets_to_bind = [descriptor_sets[i]];

                device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
                device.cmd_bind_index_buffer(
                    command_buffer,
                    index_buffer,
                    0,
                    vk::IndexType::UINT32,
                );
                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline_layout,
                    0,
                    &descriptor_sets_to_bind,
                    &[],
                );

                device.cmd_draw_indexed(command_buffer, RECT_INDICES_DATA.len() as u32, 1, 0, 0, 0);

                device.cmd_end_render_pass(command_buffer);

                device
                    .end_command_buffer(command_buffer)
                    .expect("Failed to record Command Buffer at Ending!");
            }
        }

        command_buffers
    }
}


impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            // for i in 0..MAX_FRAMES_IN_FLIGHT {
            //     self.device
            //         .destroy_semaphore(self.image_available_semaphores[i], None);
            //     self.device
            //         .destroy_semaphore(self.render_finished_semaphores[i], None);
            //     self.device.destroy_fence(self.in_flight_fences[i], None);
            // }

            // self.cleanup_swapchain();

            // self.device
            //     .destroy_descriptor_pool(self.descriptor_pool, None);

            // for i in 0..self.uniform_buffers.len() {
            //     self.device.destroy_buffer(self.uniform_buffers[i], None);
            //     self.device
            //         .free_memory(self.uniform_buffers_memory[i], None);
            // }

            // ------------------ DEVICE --------------------

            // self.device.destroy_buffer(self.index_buffer, None);
            // self.device.free_memory(self.index_buffer_memory, None);

            // self.device.destroy_buffer(self.vertex_buffer, None);
            // self.device.free_memory(self.vertex_buffer_memory, None);

            // self.device.destroy_sampler(self.texture_sampler, None);
            // self.device
            //     .destroy_image_view(self.texture_image_view, None);

            // self.device.destroy_image(self.texture_image, None);
            // self.device.free_memory(self.texture_image_memory, None);

            // self.device
            //     .destroy_descriptor_set_layout(self.ubo_layout, None);

            // self.device.destroy_command_pool(self.command_pool, None);

            // self.device.destroy_device(None);
            // self.surface_loader.destroy_surface(self.surface, None);

            // if VALIDATION.is_enable {
            //     self.debug_utils_loader
            //         .destroy_debug_utils_messenger(self.debug_merssager, None);
            // }
            // self.instance.destroy_instance(None);
        }
    }
}

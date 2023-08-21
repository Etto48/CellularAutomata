use crate::automata::Automata;
use pollster::FutureExt;

pub struct NeuralCellularAutomataGPU
{
    _filter: [[f32;3];3],
    _activation_function: String,
    //instance: wgpu::Instance,
    //adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    input_texture: wgpu::Texture,
    output_texture: wgpu::Texture,
    //shader: wgpu::ShaderModule,
    pipeline: wgpu::ComputePipeline,
    texture_bind_group: wgpu::BindGroup,
}

impl NeuralCellularAutomataGPU
{
    pub fn new(filter: [[f32;3];3], activation_function: &str, size: (usize,usize)) -> Self { 
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }).block_on().unwrap();
        let (device, queue) = adapter.request_device(&Default::default(), None).block_on().unwrap();
        let input_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Input Texture"),
            size: wgpu::Extent3d {
                width: size.0 as u32,
                height: size.1 as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let output_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Output Texture"),
            size: wgpu::Extent3d {
                width: size.0 as u32,
                height: size.1 as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
            view_formats: &[]
        });

        //wgsl shader to apply filter to input texture and save to output texture
        //filter string must represent float with '.' even if they are integers
        let filter_string = format!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
            filter[0][0], filter[0][1], filter[0][2],
            filter[1][0], filter[1][1], filter[1][2],
            filter[2][0], filter[2][1], filter[2][2],
        );
        let reg = handlebars::Handlebars::new();
        let shader_file_text = include_str!("shaders/nca_gpu.wgsl");
        let shader_text = reg.render_template(
            shader_file_text, 
            &serde_json::json!({
                "filter": filter_string,
                "activation": activation_function,
            })
        ).unwrap();

        //println!("Compiling shader:\n{}", shader_text);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute shader"),
            source: wgpu::ShaderSource::Wgsl(shader_text.into()),
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture bind group"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &input_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &output_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
            ],
        });

        //Self { filter ,activation_function: activation_function.to_string(), instance, adapter, device, queue, input_texture, output_texture, shader, pipeline, texture_bind_group}
        Self { _filter: filter, _activation_function: activation_function.to_string(), device, queue, input_texture, output_texture, pipeline, texture_bind_group}
    }
}

fn compute_work_group_count(
    (width, height): (usize, usize),
    (workgroup_width, workgroup_height): (u32, u32),
) -> (u32, u32) {
    let x = (width as u32 + workgroup_width - 1) / workgroup_width;
    let y = (height as u32 + workgroup_height - 1) / workgroup_height;

    (x, y)
}

/// Compute the next multiple of 256 for texture retrieval padding.
fn padded_bytes_per_row(width: usize) -> u32 {
    let bytes_per_row = width * 4;
    let padding = (256 - bytes_per_row % 256) % 256;
    (bytes_per_row + padding) as u32
}

impl Automata for NeuralCellularAutomataGPU
{
    fn update(&mut self, ui: &mut crate::ui::UI) {
        let size = ui.get_size();
        let mut rgba8u_buffer = Vec::new();
        rgba8u_buffer.reserve(size.0*size.1*4);
        for pixel in ui.buffer.iter()
        {
            rgba8u_buffer.push((pixel*255.0) as u8);
            rgba8u_buffer.push(0);
            rgba8u_buffer.push(0);
            rgba8u_buffer.push(0);
        }
        
        self.queue.write_texture(
            self.input_texture.as_image_copy(),
            &rgba8u_buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.0 as u32),
                rows_per_image: None, // Doesn't need to be specified as we are writing a single image.
            },
            wgpu::Extent3d { width: size.0 as u32, height: size.1 as u32, depth_or_array_layers: 1 },
        );

        let (dispatch_width, dispatch_height) =
        compute_work_group_count((size.0, size.1), (16, 16));
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute pass"),
        });

        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        compute_pass.dispatch_workgroups(dispatch_width, dispatch_height, 1);
        }   

        let padded_bytes_per_row = padded_bytes_per_row(size.0);
        let unpadded_bytes_per_row = size.0 * 4;

        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Output buffer"),
            size: (padded_bytes_per_row as usize*size.1*4) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
  
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: size.0 as u32,
                height: size.1 as u32,
                depth_or_array_layers: 1,
            },
        );
        
        self.queue.submit(Some(encoder.finish()));
        
        let buffer_slice = output_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
        self.device.poll(wgpu::Maintain::Wait);
        let padded_data = buffer_slice.get_mapped_range();

        let output_ref = ui.buffer.chunks_exact_mut(size.0);
        for (padded, pixel) in padded_data.chunks_exact(padded_bytes_per_row as usize).zip(output_ref)
        {
            let src = padded[..unpadded_bytes_per_row].as_ptr() as *const f32;
            let dst = pixel[..].as_mut_ptr();
            unsafe {
                std::ptr::copy_nonoverlapping(src, dst, size.0);
            }
        }
    }
}
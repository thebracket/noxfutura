use wgpu::util::DeviceExt;
use crate::RENDER_CONTEXT;

pub struct Palette {
    pub palette_buf: wgpu::Buffer,
    color_finder: Vec<(f32, f32, f32)>,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Palette {
    pub fn new() -> Self {
        let model = dot_vox::load("resources/vox/cordex.vox").unwrap();

        let mut color_finder = Vec::with_capacity(256);
        let mut palette = Vec::with_capacity(256 * 4);

        // Initialize the palette with the vox model default palette
        for color_bytes in model.palette.iter() {
            let rr: u8 = ((color_bytes & 0x00ff0000) >> 16) as u8;
            let rg: u8 = ((color_bytes & 0x0000ff00) >> 8) as u8;
            let rb: u8 = (color_bytes & 0x000000ff) as u8;
            let r = rr as f32 / 255.0;
            let g = rg as f32 / 255.0;
            let b = rb as f32 / 255.0;

            palette.push(r);
            palette.push(g);
            palette.push(b);
            palette.push(0.0); // To align it to 64-bits

            color_finder.push((r, g, b));
        }

        let ctl = RENDER_CONTEXT.read();
        let ctx = ctl.as_ref().unwrap();
        let palette_buf = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&palette),
                usage: wgpu::BufferUsage::STORAGE,
            });

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::StorageBuffer {
                            dynamic: false,
                            min_binding_size: wgpu::BufferSize::new(64),
                            readonly: true,
                        },
                        count: None,
                    }],
                });

        Self {
            palette_buf,
            color_finder,
            bind_group_layout,
        }
    }

    pub fn find_palette(&self, r: f32, g: f32, b: f32) -> usize {
        let mut tmp: Vec<(usize, f32)> = self
            .color_finder
            .iter()
            .enumerate()
            .map(|(idx, c)| {
                let rd = f32::abs(c.0 - r);
                let gd = f32::abs(c.1 - g);
                let bd = f32::abs(c.2 - b);
                (idx, (rd * rd) + (gd * gd) + (bd * bd))
            })
            .collect();
        tmp.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        tmp[0].0
    }
}

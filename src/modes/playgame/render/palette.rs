use bengine::*;
use gpu::util::DeviceExt;

pub struct Palette {
    pub palette_buf: gpu::Buffer,
    color_finder: Vec<(f32, f32, f32)>,
    pub bind_group_layout: gpu::BindGroupLayout,
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

            use colored::*;
            println!("{}", "COLOR".truecolor(rr, rg, rb));
        }

        let ctl = RENDER_CONTEXT.read();
        let ctx = ctl.as_ref().unwrap();
        let palette_buf = ctx
            .device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&palette),
                usage: gpu::BufferUsage::STORAGE,
            });

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[gpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: gpu::ShaderStage::VERTEX,
                        ty: gpu::BindingType::StorageBuffer {
                            dynamic: false,
                            min_binding_size: gpu::BufferSize::new(64),
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
        use colored::*;
        let rr = (r * 255.0) as u8;
        let rg = (g * 255.0) as u8;
        let rb = (b * 255.0) as u8;
        println!("{}", "REQUESTED".truecolor(rr, rg, rb));

        let mut tmp : Vec<(usize, f32)> = self.color_finder
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            let rd = f32::abs(c.0 - r);
            let gd = f32::abs(c.1 - g);
            let bd = f32::abs(c.2 - b);
            (idx, (rd * rd) + (gd * gd) + (bd * bd))
        }).collect();
        tmp.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let raw_color = self.color_finder[tmp[0].0];
        let rr = (raw_color.0 * 255.0) as u8;
        let rg = (raw_color.1 * 255.0) as u8;
        let rb = (raw_color.2 * 255.0) as u8;
        println!("{}", "FOUND".truecolor(rr, rg, rb));

        tmp[0].0
    }
}

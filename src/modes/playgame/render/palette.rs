use bengine::*;
use dot_vox::DEFAULT_PALETTE;
use gpu::util::DeviceExt;

pub struct Palette {
    palette : Vec<f32>,
    palette_buf : gpu::Buffer
}

impl Palette {
    pub fn new() -> Self {
        let mut palette = Vec::with_capacity(256*3);

        // Initialize the palette with the vox model default palette
        for color_bytes in DEFAULT_PALETTE.iter() {
            let rr: u8 = ((color_bytes & 0x00ff0000) >> 16) as u8;
            let rg: u8 = ((color_bytes & 0x0000ff00) >> 8) as u8;
            let rb: u8 = (color_bytes & 0x000000ff) as u8;
            let r = rr as f32 / 255.0;
            let g = rg as f32 / 255.0;
            let b = rb as f32 / 255.0;

            palette.push(r);
            palette.push(g);
            palette.push(b);
        }

        let ctl = RENDER_CONTEXT.read();
        let ctx = ctl.as_ref().unwrap();
        let palette_buf = ctx.device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&palette),
                usage: gpu::BufferUsage::STORAGE
                | gpu::BufferUsage::COPY_DST
                | gpu::BufferUsage::COPY_SRC,
            });

        Self {
            palette,
            palette_buf
        }
    }
}

use bengine::*;
use dot_vox::DEFAULT_PALETTE;
use gpu::util::DeviceExt;

pub struct Palette {
    palette_buf: gpu::Buffer,
    color_finder: Vec<(f32, f32, f32)>,
}

impl Palette {
    pub fn new() -> Self {
        let mut color_finder = Vec::with_capacity(256 * 3);
        let mut palette = Vec::with_capacity(256 * 3);

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

            color_finder.push((r, g, b));
        }

        let ctl = RENDER_CONTEXT.read();
        let ctx = ctl.as_ref().unwrap();
        let palette_buf = ctx
            .device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&palette),
                usage: gpu::BufferUsage::STORAGE
                    | gpu::BufferUsage::COPY_DST
                    | gpu::BufferUsage::COPY_SRC,
            });

        Self {
            palette_buf,
            color_finder,
        }
    }

    pub fn find_palette(&self, r: f32, g: f32, b: f32) -> usize {
        self.color_finder
            .iter()
            .enumerate()
            .map(|(idx, c)| {
                let rd = f32::abs(c.0 - r);
                let gd = f32::abs(c.1 - g);
                let bd = f32::abs(c.2 - b);
                (idx, rd * rd + gd * gd + bd * bd)
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

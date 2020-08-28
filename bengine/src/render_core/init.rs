use crate::imgui_wgpu::Renderer;
use imgui::*;

use imgui_winit_support::WinitPlatform;
use wgpu::{Device, Queue, SwapChainDescriptor};
use winit::window::Window;

/// Initializes ImGui for the platform
pub(crate) fn initialize_imgui(
    window: &Window,
    device: &Device,
    queue: &mut Queue,
    sc_desc: &SwapChainDescriptor,
) -> (Context, Renderer, f64, WinitPlatform) {
    let hidpi_factor = 1.0;
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../../resources/fontawesome-webfont.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                glyph_ranges: FontGlyphRanges::from_slice(&[0xf000, 0xf2e0, 0]),
                ..FontConfig::default()
            }),
        },
    ]);

    let renderer = Renderer::new(&mut imgui, device, queue, sc_desc.format);
    (imgui, renderer, hidpi_factor, platform)
}

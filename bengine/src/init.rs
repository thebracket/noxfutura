use imgui::*;
use crate::imgui_wgpu::Renderer;

use futures::executor::block_on;
use wgpu::{
    Adapter, BackendBit, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference,
    PresentMode, Queue, RequestAdapterOptions, Surface, SwapChain, SwapChainDescriptor,
    TextureFormat, TextureUsage,
};
use winit::{dpi::PhysicalSize, window::Window};
use imgui_winit_support::WinitPlatform;

const SWAPCHAIN_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;

/// Async function to request an adapter from WGPU
async fn get_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropiate adapter")
}

/// Retrieves a device object and queue object from a WGPU instance.
async fn get_device_and_queue(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                shader_validation: true,
            },
            None,
        )
        .await
        .expect("Failed to create device")
}

fn create_swap_chain(
    size: &PhysicalSize<u32>,
    device: &Device,
    surface: &Surface,
) -> (SwapChainDescriptor, SwapChain) {
    let sc_desc = SwapChainDescriptor {
        usage: TextureUsage::OUTPUT_ATTACHMENT,
        format: SWAPCHAIN_FORMAT,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Mailbox,
    };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);
    (sc_desc, swap_chain)
}

/// Container for the data intialized by a WGPU initialization
pub(crate) struct WgpuInit {
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) surface: Surface,
    pub(crate) swapchain_format: TextureFormat,
    pub(crate) swapchain_desc: SwapChainDescriptor,
    pub(crate) swap_chain: SwapChain,
}

/// Starts WGPU, associated with a window.
pub(crate) fn initialize_wgpu(window: &Window) -> WgpuInit {
    let size = window.inner_size();
    let instance = Instance::new(BackendBit::VULKAN);
    let surface = unsafe { instance.create_surface(window) };
    let adapter = block_on(get_adapter(&instance, &surface));
    let (device, queue) = block_on(get_device_and_queue(&adapter));
    let (sc_desc, swap_chain) = create_swap_chain(&size, &device, &surface);

    WgpuInit {
        adapter,
        device,
        queue,
        size,
        surface,
        swapchain_format: SWAPCHAIN_FORMAT,
        swapchain_desc: sc_desc,
        swap_chain,
    }
}

/// Initializes ImGui for the platform
pub(crate) fn initialize_imgui(
    window: &Window,
    device: &Device,
    queue: &mut Queue,
    sc_desc: &SwapChainDescriptor,
) -> (Context, Renderer, f64, WinitPlatform) {
    let mut hidpi_factor = 1.0;
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
            data: include_bytes!("../../resources/fontawesome-webfont.ttf"),
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

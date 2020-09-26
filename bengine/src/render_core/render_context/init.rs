use super::SWAPCHAIN_FORMAT;
use wgpu::{
    Adapter, Device, DeviceDescriptor, Instance, Limits, PowerPreference, PresentMode, Queue,
    RequestAdapterOptions, Surface, SwapChain, SwapChainDescriptor, TextureUsage,
};
use winit::dpi::PhysicalSize;

/// Async function to request an adapter from WGPU
pub(crate) async fn get_adapter(instance: &Instance, surface: &Surface) -> Adapter {
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
pub(crate) async fn get_device_and_queue(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(
            &DeviceDescriptor {
                features: wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY 
                | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
                | wgpu::Features::UNSIZED_BINDING_ARRAY,
                limits: Limits::default(),
                shader_validation: true,
            },
            None,
        )
        .await
        .expect("Failed to create device")
}

pub(crate) fn create_swap_chain(
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

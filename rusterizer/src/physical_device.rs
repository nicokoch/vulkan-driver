use std::ptr;
use std::u32;
use ffi_types as vk;
use dispatch::{PhysicalDevice, Device, VkLoaderDataUnion};
use version::Version;

impl PhysicalDevice {
    fn vendor_id(&self) -> u32 {
        0
    }

    fn device_id(&self) -> u32 {
        0
    }
}

pub fn get_physical_device_properties(phys_device: &PhysicalDevice, properties: &mut vk::PhysicalDeviceProperties) -> vk::Result {
    debug!("Calling get_physical_device_properties");
    properties.apiVersion = Version::new(1, 0, 5).repr();
    properties.driverVersion = 1;
    properties.vendorID = phys_device.vendor_id();
    properties.deviceID = phys_device.device_id();
    properties.deviceType = vk::PHYSICAL_DEVICE_TYPE_CPU;
    //properties.sparseProperties = ptr::null();

    let limits = vk::PhysicalDeviceLimits {
        // TODO: Values currently copied from intel driver
        maxImageDimension1D: (1 << 14),
        maxImageDimension2D: (1 << 14),
        maxImageDimension3D: (1 << 14),
        maxImageDimensionCube: (1 << 14),
        maxImageArrayLayers: (1 << 11),
        maxTexelBufferElements: 128 * 1024 * 1024,
        maxUniformBufferRange: u32::MAX,
        maxStorageBufferRange: u32::MAX,
        maxPushConstantsSize: 0, //MAX_PUSH_CONSTANTS_SIZE
        maxMemoryAllocationCount: u32::MAX,
        maxSamplerAllocationCount: 64 * 1024,
        bufferImageGranularity: 64,
        sparseAddressSpaceSize: 0,
        maxBoundDescriptorSets: 0, // MAX_SETS
        maxPerStageDescriptorSamplers: 64,
        maxPerStageDescriptorUniformBuffers: 64,
        maxPerStageDescriptorStorageBuffers: 64,
        maxPerStageDescriptorSampledImages: 64,
        maxPerStageDescriptorStorageImages: 64,
        maxPerStageDescriptorInputAttachments: 64,
        maxPerStageResources: 128,
        maxDescriptorSetSamplers: 256,
        maxDescriptorSetUniformBuffers: 256,
        maxDescriptorSetUniformBuffersDynamic: 256,
        maxDescriptorSetStorageBuffers: 256,
        maxDescriptorSetStorageBuffersDynamic: 256,
        maxDescriptorSetSampledImages: 256,
        maxDescriptorSetStorageImages: 256,
        maxDescriptorSetInputAttachments: 256,
        maxVertexInputAttributes: 32,
        maxVertexInputBindings: 32,
        maxVertexInputAttributeOffset: 2047,
        maxVertexInputBindingStride: 2048,
        maxVertexOutputComponents: 128,
        maxTessellationGenerationLevel: 0,
        maxTessellationPatchSize: 0,
        maxTessellationControlPerVertexInputComponents: 0,
        ..Default::default()
    };
    properties.limits = limits;
    vk::SUCCESS
}

pub fn get_physical_device_queue_family_properties(phys_device: &PhysicalDevice, property_count: &mut u32, properties: Option<&mut [vk::QueueFamilyProperties]>) -> vk::Result {
    debug!("Calling get_physical_device_queue_family_properties with count: {}", *property_count);
    if properties.is_none() {
        *property_count = 1;
        vk::SUCCESS
    } else {
        let properties = properties.unwrap();
        debug_assert!(properties.len() >= 1);
        properties[0] = vk::QueueFamilyProperties {
            queueFlags: vk::QUEUE_GRAPHICS_BIT | vk::QUEUE_COMPUTE_BIT | vk::QUEUE_TRANSFER_BIT,
            queueCount: 1,
            timestampValidBits: 64,
            minImageTransferGranularity: vk::Extent3D {
                width: 1,
                height: 1,
                depth: 1,
            } 
        };
        vk::SUCCESS
    }
}

pub fn get_physical_device_features(phys_device: &PhysicalDevice, features: &mut vk::PhysicalDeviceFeatures) -> vk::Result {
    //TODO
    debug!("Calling get_physical_device_features");
    *features = vk::PhysicalDeviceFeatures {
        robustBufferAccess: vk::TRUE,
        ..Default::default()
    };
    vk::SUCCESS
}

pub fn get_physical_device_memory_properties(phys_device: &PhysicalDevice, memory_properties: &mut vk::PhysicalDeviceMemoryProperties) {
    debug!("Calling get_physical_device_memory_properties");
    memory_properties.memoryTypeCount = 1;
    memory_properties.memoryTypes[0] = vk::MemoryType {
        propertyFlags:  vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT | 
                        vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                        vk::MEMORY_PROPERTY_HOST_COHERENT_BIT|
                        vk::MEMORY_PROPERTY_HOST_CACHED_BIT,
        heapIndex: 0,
    };

    memory_properties.memoryHeapCount = 1;
    memory_properties.memoryHeaps[0] = vk::MemoryHeap {
        size: 1024 * 1024 * 1024,
        flags: vk::MEMORY_HEAP_DEVICE_LOCAL_BIT,
    };

}

pub fn get_physical_device_surface_support_khr(phys_device: &PhysicalDevice, queue_family_index: u32, surface: vk::SurfaceKHR, supported: &mut vk::Bool32) -> vk::Result {
    debug!("Calling get_physical_device_surface_support_khr");
    // TODO cast surface to appropriate type
    // TODO Implement any logic at all
    *supported = vk::TRUE;
    vk::SUCCESS
}

pub fn get_physical_device_surface_formats_khr(phys_device: &PhysicalDevice, surface: vk::SurfaceKHR, surface_format_count: &mut u32, surface_formats: Option<&mut[vk::SurfaceFormatKHR]>) -> vk::Result {
    debug!("Calling get_physical_device_surface_formats_khr with param: count: {}", *surface_format_count);
    if surface_formats.is_none() {
        *surface_format_count = 1;
        vk::SUCCESS
    } else {
        let surface_formats = surface_formats.unwrap();
        debug_assert!(surface_formats.len() >= 1);
        surface_formats[0] = vk::SurfaceFormatKHR {
           format: vk::FORMAT_UNDEFINED,
           colorSpace: vk::COLORSPACE_SRGB_NONLINEAR_KHR, 
        };
        vk::SUCCESS
    }
}

pub fn get_physical_device_surface_capabilities_khr(phys_device: &PhysicalDevice, surface: vk::SurfaceKHR, surface_capabilities: &mut vk::SurfaceCapabilitiesKHR) -> vk::Result {
    // TODO These are caps for wayland. We have to actually dispatch the surface type here.
    use std::u32;
    debug!("Calling get_physical_device_surface_capabilities_khr");
    surface_capabilities.minImageCount = 1;
    surface_capabilities.maxImageCount = 4;
    surface_capabilities.currentExtent = vk::Extent2D {
       width: 1,
       height: 1,
    };
    surface_capabilities.minImageExtent = vk::Extent2D {
        width: 1,
        height: 1,
    };
    surface_capabilities.maxImageExtent = vk::Extent2D {
        width: u32::MAX,
        height: u32::MAX,
    };
    surface_capabilities.supportedTransforms = vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
    surface_capabilities.currentTransform = vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
    surface_capabilities.maxImageArrayLayers = 1;
    surface_capabilities.supportedCompositeAlpha = 
        vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR |
        vk::COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR;
    surface_capabilities.supportedUsageFlags =
        vk::IMAGE_USAGE_TRANSFER_SRC_BIT |
        vk::IMAGE_USAGE_SAMPLED_BIT |
        vk::IMAGE_USAGE_TRANSFER_DST_BIT |
        vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    vk::SUCCESS
}

pub fn get_physical_device_surface_present_modes_khr(phys_device: &PhysicalDevice, surface: vk::SurfaceKHR, present_mode_count: &mut u32, present_modes: Option<&mut [vk::PresentModeKHR]>) -> vk::Result {
    debug!("Calling get_physical_device_surface_present_modes_khr with count: {}", *present_mode_count);
    //TODO Dispatch to certain type of surface (current values are wayland)
    const SUPPORTED_PRESENT_MODES: &'static [vk::PresentModeKHR] = &[
        vk::PRESENT_MODE_FIFO_KHR,
        vk::PRESENT_MODE_MAILBOX_KHR,
    ];

    if present_modes.is_none() {
        *present_mode_count = SUPPORTED_PRESENT_MODES.len() as u32;
        vk::SUCCESS
    } else {
       debug_assert!(*present_mode_count >= SUPPORTED_PRESENT_MODES.len() as u32);
       let present_modes = present_modes.unwrap();
       for i in 0..SUPPORTED_PRESENT_MODES.len() {
            present_modes[i] = SUPPORTED_PRESENT_MODES[i];
       }
       *present_mode_count = SUPPORTED_PRESENT_MODES.len() as u32;
       vk::SUCCESS
    }
}

pub fn create_device(phys_device: &mut PhysicalDevice, create_info: &vk::DeviceCreateInfo, alloc: *const vk::AllocationCallbacks) -> Result<Box<Device>, vk::Result> {
    debug!("Calling create_device");
    Device::from_create_info(create_info, alloc).map(|device| Box::new(device))
}

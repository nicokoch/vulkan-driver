//! This file defines the interface for the icd loader.
//! the only unmangled function is vk_icdGetInstanceProcAddr, while all other function pointers
//! will be queried by the loader using this function.
//!
//! The vulkan API function in this module are all structured the same way:
//! 1. Convert raw pointers to safe rust objects (disregard input validation as this will
//!    be done by the vulkan validation layer during development)
//! 2. Call rustic function, which is located in a specialized module.
//! 3. Optionally convert the return types back to raw c pointers.
use std::ptr;
use std::ffi::CStr;
use std::slice;

use libc;
use ffi_types as vk;
use env_logger;
use LOG;

use entrypoint::{lookup_entrypoint_instance, lookup_entrypoint_device};
use extension::enumerate_extension_properties;
use dispatch::{Instance, create_instance, destroy_instance, Device, PhysicalDevice,
               enumerate_physical_devices, Queue, CommandPool, CommandBuffer};
use physical_device::{get_physical_device_properties, get_physical_device_queue_family_properties,
                      get_physical_device_features, get_physical_device_surface_support_khr,
                      create_device, get_physical_device_surface_formats_khr,
                      get_physical_device_memory_properties,
                      get_physical_device_surface_capabilities_khr,
                      get_physical_device_surface_present_modes_khr};

use device::{destroy_device, get_device_queue, create_command_pool, allocate_command_buffers};

//TODO Globally change all .as_ref().unwrap() to &* for performance.

#[no_mangle]
pub extern "system" fn vk_icdGetInstanceProcAddr(
    instance: *const Instance,
    p_name: *const libc::c_char,
) -> *const vk::PFN_vkVoidFunction {
    // This WILL be called by the loader at least once, so init logging.
    LOG.call_once(|| env_logger::init().unwrap());
    vkGetInstanceProcAddr(instance, p_name)
}

pub extern "system" fn vkGetInstanceProcAddr(
    instance: *const Instance,
    p_name: *const libc::c_char,
) -> *const vk::PFN_vkVoidFunction {
    let instance = unsafe { instance.as_ref() };
    let name = unsafe { CStr::from_ptr(p_name) };
    lookup_entrypoint_instance(instance, name)
}

pub extern "system" fn vkCreateInstance(
    create_info: *const vk::InstanceCreateInfo,
    p_allocator: *const vk::AllocationCallbacks,
    p_instance: *mut *mut Instance,
) -> vk::Result {
    let create_info = unsafe { create_info.as_ref().unwrap() };
    // TODO Do not pass raw allocator.
    match create_instance(create_info, p_allocator) {
        Err(err) => err,
        Ok(instance) => {
            unsafe { *p_instance = Box::into_raw(instance) };
            vk::SUCCESS
        }
    }
}

pub extern "system" fn vkDestroyInstance(
    instance: *mut Instance,
    p_allocator: *const vk::AllocationCallbacks,
) {
    // According to the docs, instance can be null.
    if !instance.is_null() {
        let instance = unsafe { Box::from_raw(instance) };
        destroy_instance(instance, p_allocator);
    } else {
        warn!("vkDestroyInstance called with instance == null");
    }
}

pub extern "system" fn vkEnumerateInstanceExtensionProperties(
    layer_name: *const libc::c_char,
    property_count: *mut u32,
    properties: *mut vk::ExtensionProperties,
) -> vk::Result {
    let layer_name = unsafe {
        if layer_name.is_null() {
            None
        } else {
            Some(CStr::from_ptr(layer_name))
        }
    };
    let property_count: &mut u32 = unsafe { property_count.as_mut().unwrap() };
    let mut properties = unsafe {
        if properties.is_null() {
            None
        } else {
            Some(slice::from_raw_parts_mut(
                properties,
                *property_count as usize,
            ))
        }
    };
    enumerate_extension_properties(layer_name, property_count, properties, false)
}

pub extern "system" fn vkEnumerateInstanceLayerProperties(
    property_count: *mut u32,
    properties: *mut vk::LayerProperties,
) {
    // TODO
    unimplemented!()
}

pub extern "system" fn vkGetDeviceProcAddr(
    device: *const Device,
    p_name: *const libc::c_char,
) -> *const vk::PFN_vkVoidFunction {
    // Device may not be null. (Validation layer will catch this)
    let device = unsafe { device.as_ref().unwrap() };
    let name = unsafe { CStr::from_ptr(p_name) };
    lookup_entrypoint_device(device, name)
}

pub extern "system" fn vkEnumeratePhysicalDevices(
    instance: *mut Instance,
    p_physical_devices_count: *mut u32,
    p_physical_devices: *mut *const PhysicalDevice,
) -> vk::Result {
    let instance = unsafe { instance.as_mut().unwrap() };
    let phys_device_count = unsafe { p_physical_devices_count.as_mut().unwrap() };
    let update_count = p_physical_devices.is_null();
    match enumerate_physical_devices(instance, phys_device_count, update_count) {
        None => vk::SUCCESS,
        Some(phys_device) => {
            debug_assert!(*phys_device_count >= 1);
            unsafe { *p_physical_devices = phys_device as *const _ };
            vk::SUCCESS
        }
    }
}

pub extern "system" fn vkGetPhysicalDeviceFeatures(
    phys_device: *const PhysicalDevice,
    p_features: *mut vk::PhysicalDeviceFeatures,
) -> vk::Result {
    let phys_device = unsafe { phys_device.as_ref().unwrap() };
    let features = unsafe { p_features.as_mut().unwrap() };
    get_physical_device_features(phys_device, features)
}

pub extern "system" fn vkGetPhysicalDeviceFormatProperties(
    phys_device: *const PhysicalDevice,
    format: vk::Format,
    p_format_properties: *const vk::FormatProperties,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkGetPhysicalDeviceImageFormatProperties(
    phys_device: *const PhysicalDevice,
    format: vk::Format,
    image_type: vk::ImageType,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    flags: vk::ImageCreateFlags,
    p_format_properties: vk::ImageFormatProperties,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkCreateDevice(
    phys_device: *mut PhysicalDevice,
    p_create_info: *const vk::DeviceCreateInfo,
    allocator: *const vk::AllocationCallbacks,
    p_device: *mut *const Device,
) -> vk::Result {
    let phys_device = unsafe { phys_device.as_mut().unwrap() };
    let create_info = unsafe { p_create_info.as_ref().unwrap() };
    // TODO Do not pass raw allocator
    match create_device(phys_device, create_info, allocator) {
        Err(err) => err,
        Ok(device) => {
            unsafe { *p_device = Box::into_raw(device) };
            vk::SUCCESS
        }
    }
}

pub extern "system" fn vkGetPhysicalDeviceProperties(
    phys_device: *const PhysicalDevice,
    p_properties: *mut vk::PhysicalDeviceProperties,
) -> vk::Result {
    let properties = unsafe { p_properties.as_mut().unwrap() };
    let phys_device = unsafe { phys_device.as_ref().unwrap() };
    get_physical_device_properties(phys_device, properties)
}

pub extern "system" fn vkGetPhysicalDeviceMemoryProperties(
    phys_device: *const PhysicalDevice,
    p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties,
) {
    let phys_device = unsafe { phys_device.as_ref().unwrap() };
    let memory_properties = unsafe { p_memory_properties.as_mut().unwrap() };
    get_physical_device_memory_properties(phys_device, memory_properties)
}

pub extern "system" fn vkGetPhysicalDeviceQueueFamilyProperties(
    phys_device: *const PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut vk::QueueFamilyProperties,
) -> vk::Result {
    let phys_device = unsafe { phys_device.as_ref().unwrap() };
    let property_count = unsafe { p_property_count.as_mut().unwrap() };
    let mut properties = unsafe {
        if p_properties.is_null() {
            None
        } else {
            Some(slice::from_raw_parts_mut(
                p_properties,
                *property_count as usize,
            ))
        }
    };
    get_physical_device_queue_family_properties(phys_device, property_count, properties)
}

pub extern "system" fn vkEnumerateDeviceExtensionProperties(
    phys_device: *const PhysicalDevice,
    p_layer_name: *const libc::c_char,
    p_property_count: *mut u32,
    p_properties: *mut vk::ExtensionProperties,
) -> vk::Result {
    let layer_name = unsafe {
        if p_layer_name.is_null() {
            None
        } else {
            Some(CStr::from_ptr(p_layer_name))
        }
    };
    let property_count: &mut u32 = unsafe { p_property_count.as_mut().unwrap() };
    let mut properties = unsafe {
        if p_properties.is_null() {
            None
        } else {
            Some(slice::from_raw_parts_mut(
                p_properties,
                *property_count as usize,
            ))
        }
    };
    enumerate_extension_properties(layer_name, property_count, properties, true)
}

pub extern "system" fn vkGetPhysicalDeviceSparseImageFormatProperties(
    phys_device: *const PhysicalDevice,
    format: vk::Format,
    image_type: vk::ImageType,
    samples: vk::SampleCountFlagBits,
    usage: vk::ImageUsageFlags,
    tiling: vk::ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut vk::SparseImageFormatProperties,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    phys_device: *const PhysicalDevice,
    queue_family_index: u32,
    surface: vk::SurfaceKHR,
    p_supported: *mut vk::Bool32,
) -> vk::Result {
    let phys_device = unsafe { phys_device.as_ref().unwrap() };
    let supported = unsafe { p_supported.as_mut().unwrap() };
    get_physical_device_surface_support_khr(phys_device, queue_family_index, surface, supported)
}

pub extern "system" fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    phys_device: *const PhysicalDevice,
    surface: vk::SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut vk::SurfaceFormatKHR,
) -> vk::Result {
    let phys_device = unsafe { phys_device.as_ref().unwrap() };
    let surface_format_count = unsafe { p_surface_format_count.as_mut().unwrap() };
    let surface_formats = unsafe {
        if p_surface_formats.is_null() {
            None
        } else {
            Some(slice::from_raw_parts_mut(
                p_surface_formats,
                *surface_format_count as usize,
            ))
        }
    };
    get_physical_device_surface_formats_khr(
        phys_device,
        surface,
        surface_format_count,
        surface_formats,
    )
}

pub extern "system" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    phys_device: *const PhysicalDevice,
    surface: vk::SurfaceKHR,
    p_surface_capabilities: *mut vk::SurfaceCapabilitiesKHR,
) -> vk::Result {
    let phys_device = unsafe { phys_device.as_ref().unwrap() };
    let surface_capabilities = unsafe { p_surface_capabilities.as_mut().unwrap() };
    get_physical_device_surface_capabilities_khr(phys_device, surface, surface_capabilities)
}

pub extern "system" fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    phys_device: *const PhysicalDevice,
    surface: vk::SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut vk::PresentModeKHR,
) -> vk::Result {
    let phys_device = unsafe { &*phys_device };
    let present_mode_count = unsafe { &mut *p_present_mode_count };
    let present_modes = unsafe {
        if p_present_modes.is_null() {
            None
        } else {
            Some(slice::from_raw_parts_mut(
                p_present_modes,
                *present_mode_count as usize,
            ))
        }
    };
    get_physical_device_surface_present_modes_khr(
        phys_device,
        surface,
        present_mode_count,
        present_modes,
    )
}

// Device functions

pub extern "system" fn vkDestroyDevice(
    device: *mut Device,
    allocator: *const vk::AllocationCallbacks,
) {
    if !device.is_null() {
        let device = unsafe { Box::from_raw(device) };
        destroy_device(device, allocator);
    } else {
        warn!("vkDestroyDevice called with null device");
    }
}

pub extern "system" fn vkGetDeviceQueue(
    device: *mut Device,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut *const Queue,
) {
    // What is that *mut *const construct?
    // The queue exists since device creation and will not be created in this function call.
    // This means that we only return a raw pointer to the requested queue. this raw pointer
    // (=handle) will be stored on p_queue. Not 100% sure this is the correct way to do this.
    let device = unsafe { device.as_ref().unwrap() };
    let queue = get_device_queue(device, queue_family_index, queue_index);
    unsafe { *p_queue = queue as *const _ };
}

pub extern "system" fn vkQueueSubmit(
    queue: *mut Queue,
    submit_count: u32,
    p_submits: *const vk::SubmitInfo,
    fence: vk::Fence,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkQueueWaitIdle(queue: *mut Queue) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkDeviceWaitIdle(device: *mut Device) -> vk::Result {
    //TODO
    vk::SUCCESS
}

pub extern "system" fn vkAllocateMemory(
    device: *mut Device,
    p_allocate_info: vk::MemoryAllocateInfo,
    p_allocator: *const vk::AllocationCallbacks,
    memory: *mut vk::DeviceMemory,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkFreeMemory(
    device: *mut Device,
    memory: *const vk::DeviceMemory,
    p_allocator: *const vk::AllocationCallbacks,
) {
    unimplemented!()
}

pub extern "system" fn vkMapMemory(
    device: *mut Device,
    memory: *const vk::DeviceMemory,
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
    flags: vk::MemoryMapFlags,
    pp_data: *mut *mut libc::c_void,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkUnmapMemory(
    device: *mut Device,
    memory: *const vk::DeviceMemory,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkFlushMappedMemoryRanges(
    device: *mut Device,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkInvalidateMappedMemoryRanges(
    device: *mut Device,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange,
) -> vk::Result {
    unimplemented!()
}

pub extern "system" fn vkCreateCommandPool(
    device: *mut Device,
    p_create_info: *const vk::CommandPoolCreateInfo,
    p_allocator: *const vk::AllocationCallbacks,
    p_command_pool: *mut *mut CommandPool,
) -> vk::Result {
    let device = unsafe { device.as_ref().unwrap() };
    let create_info = unsafe { p_create_info.as_ref().unwrap() };
    match create_command_pool(device, create_info, p_allocator) {
        Err(err) => err,
        Ok(command_pool) => {
            unsafe { *p_command_pool = Box::into_raw(command_pool) };
            vk::SUCCESS
        }
    }
}

//TODO p_allocate_info should actually be *const, but we have to mutate its contents, so we must
//use *mut. Normally one can use Refcell to circumvent this, but I'm not sure how this interacts
//with FFI.
pub extern "system" fn vkAllocateCommandBuffers(
    device: *mut Device,
    p_allocate_info: *mut vk::CommandBufferAllocateInfo,
    p_command_buffers: *mut *const CommandBuffer,
) -> vk::Result {
    let device = unsafe { device.as_ref().unwrap() };
    let allocate_info = unsafe { p_allocate_info.as_mut().unwrap() };
    allocate_command_buffers(device, allocate_info)
}

pub extern "system" fn vkCreateSwapchainKHR(
    device: *mut Device,
    p_create_info: *const vk::SwapchainCreateInfoKHR,
    p_allocator: *const vk::AllocationCallbacks,
    p_swapchain: *mut vk::SwapchainKHR,
) -> vk::Result {
    let device = unsafe { device.as_ref().unwrap() };
    let create_info = unsafe { p_create_info.as_ref().unwrap() };
    let swapchain = unsafe { p_swapchain.as_mut().unwrap() };
    println!("Creating swapchain");
    vk::SUCCESS
}

// pub extern "system" fn vkGetSwapchainImagesKHR(device: *mut Device, swapchain: *mut vk::SwapchainKHR, )

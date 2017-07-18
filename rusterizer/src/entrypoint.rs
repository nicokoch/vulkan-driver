use std::ptr;
use std::ffi::CStr;
use libc;

use loader_interface as api;
use ffi_types as vk;
use dispatch::{Instance, Device};

pub fn lookup_entrypoint_instance(instance: Option<&Instance>,
                                  name: &CStr)
                                  -> *const vk::PFN_vkVoidFunction {
    debug!("lookup_entrypoint_instance with params: instance: {:?}, pName: {:?}",
           instance,
           name);
    if instance.is_none() {
        // Instance is null: Only global functions
        match name.to_string_lossy().as_ref() {
            "vkGetInstanceProcAddr" => api::vkGetInstanceProcAddr as *const _,
            "vkCreateInstance" => api::vkCreateInstance as *const _,
            "vkEnumerateInstanceExtensionProperties" => {
                api::vkEnumerateInstanceExtensionProperties as *const _
            }
            "vkEnumerateInstanceLayerProperties" => {
                api::vkEnumerateInstanceLayerProperties as *const _
            }
            _ => ptr::null(), // TODO warn
        }
    } else {
        warn!("Instance not null -- not completely implemented");
        // Instance is not null: Only instance level functions
        match name.to_string_lossy().as_ref() {
            "vkGetDeviceProcAddr" => api::vkGetDeviceProcAddr as *const _,
            "vkDestroyInstance" => api::vkDestroyInstance as *const _,
            "vkEnumeratePhysicalDevices" => api::vkEnumeratePhysicalDevices as *const _,
            "vkGetPhysicalDeviceFeatures" => api::vkGetPhysicalDeviceFeatures as *const _,
            "vkGetPhysicalDeviceFormatProperties" => {
                api::vkGetPhysicalDeviceFormatProperties as *const _
            }
            "vkGetPhysicalDeviceImageFormatProperties" => {
                api::vkGetPhysicalDeviceImageFormatProperties as *const _
            }
            "vkCreateDevice" => api::vkCreateDevice as *const _,
            "vkGetPhysicalDeviceProperties" => api::vkGetPhysicalDeviceProperties as *const _,
            "vkGetPhysicalDeviceMemoryProperties" => {
                api::vkGetPhysicalDeviceMemoryProperties as *const _
            }
            "vkGetPhysicalDeviceQueueFamilyProperties" => {
                api::vkGetPhysicalDeviceQueueFamilyProperties as *const _
            }
            "vkEnumerateDeviceExtensionProperties" => {
                api::vkEnumerateDeviceExtensionProperties as *const _
            }
            "vkGetPhysicalDeviceSparseImageFormatProperties" => {
                api::vkGetPhysicalDeviceSparseImageFormatProperties as *const _
            }
            "vkGetPhysicalDeviceSurfaceSupportKHR" => {
                api::vkGetPhysicalDeviceSurfaceSupportKHR as *const _
            }
            "vkGetPhysicalDeviceSurfaceFormatsKHR" => {
                api::vkGetPhysicalDeviceSurfaceFormatsKHR as *const _
            }
            "vkGetPhysicalDeviceSurfaceCapabilitiesKHR" => {
                api::vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const _
            }
            "vkGetPhysicalDeviceSurfacePresentModesKHR" => {
                api::vkGetPhysicalDeviceSurfacePresentModesKHR as *const _
            }
            function_name => {
                warn!("Returning null pointer for function {}", function_name);
                ptr::null()
            } 
        }
    }
}

pub fn lookup_entrypoint_device(device: &Device, name: &CStr) -> *const vk::PFN_vkVoidFunction {
    debug!("Calling vkGetDeviceProcAddr with params: device: {:?}, name: {:?}",
           device,
           name);
    match name.to_string_lossy().as_ref() {
        // TODO
        "vkGetDeviceProcAddr" => api::vkGetDeviceProcAddr as *const _,
        "vkDestroyDevice" => api::vkDestroyDevice as *const _,
        "vkGetDeviceQueue" => api::vkGetDeviceQueue as *const _,
        "vkQueueSubmit" => api::vkQueueSubmit as *const _,
        "vkQueueWaitIdle" => api::vkQueueWaitIdle as *const _,
        "vkDeviceWaitIdle" => api::vkDeviceWaitIdle as *const _,
        "vkAllocateMemory" => api::vkAllocateMemory as *const _,
        "vkFreeMemory" => api::vkFreeMemory as *const _,
        "vkMapMemory" => api::vkMapMemory as *const _,
        "vkUnmapMemory" => api::vkUnmapMemory as *const _,
        "vkFlushMappedMemoryRanges" => api::vkFlushMappedMemoryRanges as *const _,
        "vkInvalidateMappedMemoryRanges" => api::vkInvalidateMappedMemoryRanges as *const _,
        "vkCreateCommandPool" => api::vkCreateCommandPool as *const _,
        "vkAllocateCommandBuffers" => api::vkAllocateCommandBuffers as *const _,
        "vkCreateSwapchainKHR" => api::vkCreateSwapchainKHR as *const _,
        "vkGetSwapchainImagesKHR" => api::vkGetSwapchainImagesKHR as *const _,
        function_name => {
            warn!("Returning null pointer for function {}", function_name);    
            ptr::null()
        },
    }
}

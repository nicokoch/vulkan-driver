use std::ffi::{CStr, CString};
use std::cmp;
use std::ptr;
use libc;
use ffi_types as vk;


pub const AVAILABLE_EXTENSIONS: &'static [ExtensionProperties] = &[
    ExtensionProperties {
        extension_name: "VK_KHR_surface",
        spec_version: 25,
    },
    #[cfg(unix)]
    ExtensionProperties {
        extension_name: "VK_KHR_xcb_surface",
        spec_version: 5,
    },
    #[cfg(windows)]
    ExtensionProperties {
        extension_name: "VK_KHR_win32_surface",
        spec_version: 5,
    },
];

pub const AVAILABLE_DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[
    ExtensionProperties {
        extension_name: "VK_KHR_swapchain",
        spec_version: 67,
    },
];

pub struct ExtensionProperties {
    extension_name: &'static str,
    spec_version: u32,
}

impl ExtensionProperties {
    fn to_vk_ffi(&self) -> vk::ExtensionProperties {
        // This conversation is a pita in rust :(
        let mut arr = [0; vk::MAX_EXTENSION_NAME_SIZE as usize];
        let value = CString::new(self.extension_name).unwrap();
        for (a, c) in arr.iter_mut().zip(value.as_bytes_with_nul().iter()) {
            *a = *c as i8;
        }

        vk::ExtensionProperties {
            extensionName: arr,
            specVersion: self.spec_version,
        }
    }

    pub fn name(&self) -> &str {
        self.extension_name
    }
}

pub fn enumerate_extension_properties(
    layer_name: Option<&CStr>,
    property_count: &mut u32,
    properties: Option<&mut [vk::ExtensionProperties]>,
    for_device: bool,
) -> vk::Result {
    debug!(
        "enumerate_extension_properties with args: layer_name: {:?}, property_count: {:?}, \
            properties_exists: {:?}, for_device: {}",
        layer_name,
        property_count,
        properties.is_some(),
        for_device
    );
    let available_extensions = if for_device {
        AVAILABLE_DEVICE_EXTENSIONS
    } else {
        AVAILABLE_EXTENSIONS
    };

    if properties.is_none() {
        // TODO deal with layer_name
        *property_count = available_extensions.len() as u32;
        vk::SUCCESS
    } else {
        let properties = properties.unwrap();
        if layer_name.is_none() {
            for i in 0..cmp::min(*property_count as usize, available_extensions.len()) {
                // We don't have clone, so we do the dirty memcpy hack.
                properties[i] = available_extensions[i].to_vk_ffi();
            }
            if *property_count < available_extensions.len() as u32 {
                *property_count = properties.len() as u32;
                vk::INCOMPLETE
            } else {
                *property_count = available_extensions.len() as u32;
                vk::SUCCESS
            }

        } else {
            // TODO: deal with layer_name
            warn!(
                "enumerate_extension_properties called with non-null layer_name, no extensions \
                   will be found."
            );
            unimplemented!()
        }
    }
}

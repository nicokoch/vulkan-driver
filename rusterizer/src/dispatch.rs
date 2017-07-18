//! This module implements all ICD dispatchable objects.
//! Dispatchable objects are required by the vulkan loader.
//! More info:
//! https://github.com/KhronosGroup/Vulkan-LoaderAndValidationLayers/blob/master/loader
//! /LoaderAndLayerInterface.md#icd-dispatchable-object-creation
use std::ffi::CStr;
use std::default::Default;
use libc;
use ffi_types as vk;
use version::Version;
use extension::AVAILABLE_EXTENSIONS;

static ICD_LOADER_MAGIC: usize = 0x01CDC0DE;

// Attempting a union
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VkLoaderDataUnion {
    // 8 byte of data
    data: usize,
}

impl VkLoaderDataUnion {
    fn as_loader_magic(&self) -> usize {
        self.data
    }

    fn as_loader_data(&self) -> *const libc::c_void {
        &self.data as *const _ as *const libc::c_void
    }

    fn set_loader_magic(&mut self, val: usize) {
        self.data = val;
    }
}

impl Default for VkLoaderDataUnion {
    fn default() -> Self {
        VkLoaderDataUnion {
            data: ICD_LOADER_MAGIC,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Instance {
    _loader_data: VkLoaderDataUnion,
    alloc: *const vk::AllocationCallbacks,
    client_version: Version,
    enabled_layers: Vec<String>, // TODO (Intel driver does not have this, why do we?)
    enabled_extensions: Vec<String>,
    // Only one physical device per instance, the CPU
    phys_device: Option<PhysicalDevice>,
}

impl Instance {
    pub fn from_create_info(create_info: &vk::InstanceCreateInfo,
                            alloc: *const vk::AllocationCallbacks)
                            -> Result<Self, vk::Result> {
        debug_assert_eq!(create_info.sType, vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO);
        println!("{:?}", create_info);
        // TODO Why does this fail?
        // debug_assert!(create_info.pNext.is_null());
        if !alloc.is_null() {
            warn!("Driver does not support custom allocators");
            return Err(vk::ERROR_INITIALIZATION_FAILED);
        }
        let app_info = unsafe { create_info.pApplicationInfo.as_ref() };
        let client_version = match app_info {
            Some(info) if info.apiVersion != 0 => Version::from_repr(info.apiVersion),
            _ => (1, 0, 0).into(),
        };

        if Version::new(1, 0, 0) > client_version || client_version > Version::new(1, 0, 0xfff) {
            return Err(vk::ERROR_INCOMPATIBLE_DRIVER);
        }

        let requested_extensions = unsafe {
            parse_cchar_array(create_info.ppEnabledExtensionNames,
                              create_info.enabledExtensionCount)
        };

        for requested_extension in &requested_extensions {
            if !AVAILABLE_EXTENSIONS.iter().any(|ext| ext.name() == requested_extension) {
                warn!("Could not find extension {}", requested_extension);
                return Err(vk::ERROR_EXTENSION_NOT_PRESENT);
            }
        }


        Ok(Instance {
            _loader_data: VkLoaderDataUnion { data: ICD_LOADER_MAGIC },
            alloc: alloc,
            client_version: client_version,
            enabled_layers: Vec::new(),
            enabled_extensions: requested_extensions,
            phys_device: None,
        })
    }

    pub fn phys_device(&self) -> Option<&PhysicalDevice> {
        self.phys_device.as_ref()
    }

    fn init_phys_device(&mut self) {
        self.phys_device = Some(PhysicalDevice::default());
    }
}

unsafe fn parse_cchar_array(extension_names: *const *const libc::c_char,
                            count: u32)
                            -> Vec<String> {
    let mut res = Vec::with_capacity(count as usize);
    for i in 0..count {
        let cstr = CStr::from_ptr(*extension_names.offset(i as isize));
        res.push(cstr.to_string_lossy().into_owned());
    }
    res
}

pub fn create_instance(create_info: &vk::InstanceCreateInfo,
                       alloc: *const vk::AllocationCallbacks)
                       -> Result<Box<Instance>, vk::Result> {
    debug!("calling create_instance");
    // Ok, so we box up the instance to store it on the heap, and immediately after "unbox" it, so
    // rust will not free the memory (Vulkan has manual memory management)
    Instance::from_create_info(create_info, alloc).map(|instance| Box::new(instance))
}

pub fn destroy_instance(instance: Box<Instance>, alloc: *const vk::AllocationCallbacks) {
    debug!("calling destroy_instance");
    assert!(alloc.is_null());
    // Using explicit drop here so it's obvious what this function does.
    drop(instance)
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PhysicalDevice {
    _loader_data: VkLoaderDataUnion,
}

pub fn enumerate_physical_devices<'a>(instance: &'a mut Instance,
                                  phys_device_count: &mut u32,
                                  update_count: bool)
                                  -> Option<&'a PhysicalDevice> {
    debug!("calling enumerate_physical_devices with params: phys_device_count: {}",
           *phys_device_count);
    // Only one device (CPU) is always available, so this method is kind of trivial
    // Note that the intel driver uses this method for allocation too, so we should definitely look
    // into doing that.
    if update_count {
        *phys_device_count = 1;
        None
    } else {
        if instance.phys_device().is_none() {
            instance.init_phys_device();
        }
        instance.phys_device()
    }
}


#[derive(Debug)]
#[repr(C)]
pub struct Device {
    _loader_data: VkLoaderDataUnion,
    queue: Queue,
}

impl Device {
    pub fn from_create_info(create_info: &vk::DeviceCreateInfo,
                            alloc: *const vk::AllocationCallbacks)
                            -> Result<Self, vk::Result> {
        debug_assert_eq!(create_info.sType, vk::STRUCTURE_TYPE_DEVICE_CREATE_INFO);
        // TODO why does this fail?
        // debug_assert!(create_info.pNext.is_null());

        if !alloc.is_null() {
            warn!("Driver does not support custom allocators");
            return Err(vk::ERROR_INITIALIZATION_FAILED);
        }
        Ok(Device {
            _loader_data: VkLoaderDataUnion::default(),
            queue: Queue::default(),
        })
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Queue {
    _loader_data: VkLoaderDataUnion,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CommandPool {
    pub buffers: Vec<Box<CommandBuffer>>,
}

impl CommandPool {
    pub fn from_create_info(create_info: &vk::CommandPoolCreateInfo, alloc: *const vk::AllocationCallbacks) -> Result<Self, vk::Result> {
        debug_assert_eq!(create_info.sType, vk::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO);
        debug_assert!(create_info.pNext.is_null());
        if !alloc.is_null() {
            warn!("Custom allocators not supported by driver");
            return Err(vk::ERROR_INITIALIZATION_FAILED);
        }
        Ok(CommandPool::default())
    }

    pub fn buffers_mut(&mut self) -> &mut Vec<Box<CommandBuffer>> {
        &mut self.buffers
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CommandBuffer {
    _loader_data: VkLoaderDataUnion,
    level: vk::CommandBufferLevel,
}

impl CommandBuffer {
    pub fn new(level: vk::CommandBufferLevel) -> Self {
        CommandBuffer {
            _loader_data: VkLoaderDataUnion::default(),
            level: level,
        }
    }
}

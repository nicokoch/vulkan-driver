use ffi_types as vk;
use dispatch::{Device, Queue, CommandPool, CommandBuffer};

pub fn destroy_device(device: Box<Device>, alloc: *const vk::AllocationCallbacks) {
    debug!("Calling destroy_device");
    debug_assert!(alloc.is_null());
    drop(device);
}

pub fn get_device_queue(device: &Device, queue_family_index: u32, queue_index: u32) -> &Queue {
    debug!("Calling get_device_queue");
    debug_assert_eq!(queue_index, 0);
    device.queue()
}

pub fn create_command_pool(
    device: &Device,
    create_info: &vk::CommandPoolCreateInfo,
    alloc: *const vk::AllocationCallbacks,
) -> Result<Box<CommandPool>, vk::Result> {
    debug!("Calling create_command_pool");
    CommandPool::from_create_info(create_info, alloc).map(|command_pool| Box::new(command_pool))
}

pub fn allocate_command_buffers(
    device: &Device,
    allocate_info: &mut vk::CommandBufferAllocateInfo,
) -> vk::Result {
    debug!("Calling allocate_command_buffers");
    debug_assert_eq!(
        allocate_info.sType,
        vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO
    );
    debug_assert!(allocate_info.pNext.is_null());
    for i in 0..allocate_info.commandBufferCount {
        let buffer = Box::new(CommandBuffer::new(allocate_info.level));
        let buffers = allocate_info.commandPool.buffers_mut();
        buffers.push(buffer);
    }
    vk::SUCCESS
}

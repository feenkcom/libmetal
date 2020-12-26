use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use metal::{CommandQueue, Device};

#[no_mangle]
pub fn metal_device_default() -> *mut ValueBox<Device> {
    ValueBox::new(Device::system_default().expect("no device found")).into_raw()
}

#[no_mangle]
pub fn metal_device_new_command_queue(
    device_ptr: *mut ValueBox<Device>,
) -> *mut ValueBox<CommandQueue> {
    device_ptr.with_not_null_return(std::ptr::null_mut(), |device| {
        ValueBox::new(device.new_command_queue()).into_raw()
    })
}

#[no_mangle]
pub fn metal_device_drop(ptr: &mut *mut ValueBox<Device>) {
    ptr.drop();
}

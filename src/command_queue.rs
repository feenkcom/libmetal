use boxer::{ValueBox, ValueBoxPointerReference};
use metal::CommandQueue;

#[no_mangle]
pub fn metal_command_queue_drop(ptr: &mut *mut ValueBox<CommandQueue>) {
    ptr.drop();
}

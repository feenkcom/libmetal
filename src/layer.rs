use boxer::{ValueBox, ValueBoxPointerReference};
use metal::MetalLayer;

#[no_mangle]
pub fn metal_layer_new() -> *mut ValueBox<MetalLayer> {
    ValueBox::new(MetalLayer::new()).into_raw()
}

#[no_mangle]
pub fn metal_layer_drop(ptr: &mut *mut ValueBox<MetalLayer>) {
    ptr.drop();
}

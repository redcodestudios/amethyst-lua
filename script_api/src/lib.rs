#[repr(C)]
pub struct MyState {
    background_color: [i32; 4],
}

#[no_mangle]
pub extern "C" fn change_bg_color(state: &mut MyState, colors: [i32; 4]) {
    state.background_color = colors;
}

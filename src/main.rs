use amethyst::{
    core::transform::TransformBundle,
    ecs::prelude::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};


// Scripting code
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

//extern crate script_api;
//use script_api::MyState;

extern{
    fn call_lua(state: *mut MyState, script: *const c_char);
}

//fn exec_script(state: *mut i32, script_path: &str) {
//    println!("RUST: loading {} script", script_path);
//    unsafe {
//        //let raw_ptr = &mut state as *mut i32;
//        call_lua(state, CString::new(script_path).expect("CString::new failed").as_ptr());
//        println!("RUST: new_ptr value {}", *state);
//   }
//}

///////////////

struct MyState {
    background_color: [i32; 4],
}

fn state_exec_script(state: *mut MyState, script_path: &str) {
    println!("RUST: loading {} script", script_path);
    unsafe {
        call_lua(state, CString::new(script_path).expect("CString::new failed").as_ptr());
    }
}


impl SimpleState for MyState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
    
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
       state_exec_script(self, "src/scripts/blink.lua");
       Trans::None
    }
}

fn main() -> amethyst::Result<()>{
    
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;
 
    let mut game = Application::new("/", MyState{background_color: [1,2,3,4]}, game_data)?;
    game.run();
    
    Ok(())
    //let mut state: i32 = 2;

    //loop {
        // exec_script(&mut state, "src/script/blink.lua");
        // output(state);
        // thread::sleep(time::Duration::from_millis(1000)); 
    //}
}

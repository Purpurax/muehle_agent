pub mod engine;

fn main() {
    
    // TODO: Create an argument parser, which can take additional Parameters

    // Windows size is 1280px on scale = 1
    let window_scale:f32 = 0.7;
    
    engine::run(window_scale);
}

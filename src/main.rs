use log::error;
use matrix_testing::{vec::Vec2, Dot, SCREEN_HEIGHT, SCREEN_WIDTH};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        WindowBuilder::new()
            .with_title("Matrix testing")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)?
    };
    let mut dot = Dot::default();
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            dot.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                error!("pixels.render: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if !input.update(&event) {
            return;
        }
        if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
            *control_flow = ControlFlow::Exit;
            return;
        }
        if input.key_pressed(VirtualKeyCode::Space) {
            dot.randomize();
        } else {
            let mut vel = Vec2::new(0, 0);
            if input.key_held(VirtualKeyCode::Up) {
                vel.y -= 500;
            }
            if input.key_held(VirtualKeyCode::Down) {
                vel.y += 500;
            }
            if input.key_held(VirtualKeyCode::Left) {
                vel.x -= 500;
            }
            if input.key_held(VirtualKeyCode::Right) {
                vel.x += 500;
            }
            if vel.x != 0 || vel.y != 0 {
                dot.vel = vel.resize();
            }
        }
        if let Some(size) = input.window_resized() {
            if let Err(err) = pixels.resize_surface(size.width, size.height) {
                error!("pixels.resize_surface: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        dot.update();
        window.request_redraw();
    });
}


// use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
// use winit::event_loop::{EventLoop, ControlFlow};

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const IS_PAINT_FPS_COUNTER: bool = true;

// pub fn init_window(
//     event_loop: &EventPump,
//     title: &str,
//     width: u32,
//     height: u32,
// ) -> winit::window::Window {
//     winit::window::WindowBuilder::new()
//         .with_title(title)
//         .with_inner_size(winit::dpi::LogicalSize::new(width, height))
//         .build(event_loop)
//         .expect("Failed to create window.")
// }

pub trait VulkanApp {
    fn draw_frame(&mut self, delta_time: f32);
    fn recreate_swapchain(&mut self);
    fn cleanup_swapchain(&self);
    fn wait_device_idle(&self);
    fn resize_framebuffer(&mut self);
    fn window_ref(&self) -> &sdl2::video::Window;
}

pub struct ProgramProc {
    pub event_pump: EventPump,
}

impl ProgramProc {

    pub fn new() -> (sdl2::Sdl, ProgramProc) {
        let sdl_context = sdl2::init().unwrap();
        // init window stuff
        let event_pump = sdl_context.event_pump().unwrap();

        (sdl_context, ProgramProc { event_pump })
    }

    pub fn main_loop<A: 'static + VulkanApp>(mut self, mut vulkan_app: A) {

        let mut tick_counter = super::fps_limiter::FPSLimiter::new();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                let delta_time = tick_counter.delta_time();
                vulkan_app.draw_frame(delta_time);

                if IS_PAINT_FPS_COUNTER {
                    print!("FPS: {}\r", tick_counter.fps());
                }

                tick_counter.tick_frame();
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        vulkan_app.wait_device_idle();
                        break 'running
                    },
                    // Event::
                    _ => {}
                }
            }
            tick_counter.tick_frame();
            // ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }    
        // self.event_loop.run(move |event, _, control_flow| {

        //     match event {
        //         | Event::WindowEvent { event, .. } => {
        //             match event {
        //                 | WindowEvent::CloseRequested => {
        //                     vulkan_app.wait_device_idle();
        //                     *control_flow = ControlFlow::Exit
        //                 },
        //                 | WindowEvent::KeyboardInput { input, .. } => {
        //                     match input {
        //                         | KeyboardInput { virtual_keycode, state, .. } => {
        //                             match (virtual_keycode, state) {
        //                                 | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
        //                                     vulkan_app.wait_device_idle();
        //                                     *control_flow = ControlFlow::Exit
        //                                 },
        //                                 | _ => {},
        //                             }
        //                         },
        //                     }
        //                 },
        //                 | WindowEvent::Resized(_new_size) => {
        //                     vulkan_app.wait_device_idle();
        //                     vulkan_app.resize_framebuffer();
        //                 },
        //                 | _ => {},
        //             }
        //         },
        //         | Event::MainEventsCleared => {
        //             vulkan_app.window_ref().request_redraw();
        //         },
        //         | Event::RedrawRequested(_window_id) => {
        //             let delta_time = tick_counter.delta_time();
        //             vulkan_app.draw_frame(delta_time);

        //             if IS_PAINT_FPS_COUNTER {
        //                 print!("FPS: {}\r", tick_counter.fps());
        //             }

        //             tick_counter.tick_frame();
        //         },
        //         | Event::LoopDestroyed => {
        //             vulkan_app.wait_device_idle();
        //         },
        //         _ => (),
        //     }

        // })
    }

}

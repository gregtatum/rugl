#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

extern crate time;
use super::glutin;
use super::gl::types::*;
use super::gl;
use super::draw_builder;
use super::clear;
use super::gl_helpers;
use std::string;

pub struct Environment {
    pub time: f64,
    pub tick: u64,
    pub viewport_width: u32,
    pub viewport_height: u32,
}

pub struct Rugl {
    start_time: f64,
    window: glutin::Window,
    events_loop: glutin::EventsLoop,
    environment: Environment
}

pub fn init() -> Rugl {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_vsync()
        .with_title("rugl")
        .with_decorations(true)
        .build(&events_loop)
        .unwrap();

    let _ = unsafe { window.make_current() };

    // Load the OpenGL function pointers
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    log_draw!(
        "OpenGL Version {}.{}",
        gl_helpers::get_major_version(),
        gl_helpers::get_minor_version()
    );

    let (viewport_width, viewport_height) = window.get_inner_size_pixels().unwrap();

    Rugl {
        start_time: time::precise_time_s(),
        window: window,
        events_loop: events_loop,
        environment: Environment {
            time: 0.0,
            tick: 0,
            viewport_width: viewport_width,
            viewport_height: viewport_height
        }
    }
}

impl Rugl {
    pub fn draw(&self) -> draw_builder::DrawBuilder {
        draw_builder::DrawBuilder::new()
    }

    pub fn frame<F>(&mut self, draw: F) where
        F: Fn(&Environment)
    {
        let start_time = self.start_time;
        #[allow(unused_variables)]
        let mut previous_time = time::precise_time_s();

        let ref mut environment = self.environment;
        let ref events_loop = self.events_loop;
        let ref window = self.window;
        let mut run_loop = true;
        while run_loop {
            #[cfg(feature = "draw_once")] {
                events_loop.interrupt();
                run_loop = false;
            }
            events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
                        events_loop.interrupt();
                        run_loop = false;
                    },
                    _ => {}
                }
            });

            let now = time::precise_time_s();
            log_draw!("update loop time:  {}ms", (now - previous_time) * 1000.0);
            log_draw!("-------------------------------------------------");
            previous_time = now;
            environment.time = now - start_time;
            environment.tick += 1;

            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                log_draw!("gl::Enable(gl::DEPTH_TEST)");
                gl::BlendFuncSeparate(gl::ONE, gl::ZERO, gl::ONE, gl::ZERO);
                log_draw!("gl::BlendFuncSeparate(gl::ONE, gl::ZERO, gl::ONE, gl::ZERO);");
            }
            draw(&environment);

            log_draw!("draw time:         {}ms", (time::precise_time_s() - now) * 1000.0);
            let _ = window.swap_buffers();

            log_draw!("swap buffers time: {}ms", (time::precise_time_s() - now) * 1000.0);
        }
    }
}

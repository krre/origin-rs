use core::settings::Settings;
use gfx::renderer::Renderer;
use std::time::SystemTime;
use winit;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const APP_NAME: &str = "Origin";

pub struct Application {
    settings: Settings,
    events_loop: winit::EventsLoop,
    window: winit::Window,
    renderer: Renderer,
}

impl Application {
    pub fn new() -> Self {
        let settings = Settings::new();
        let events_loop = winit::EventsLoop::new();

        let (x, y, width, height) = match settings.window_geometry() {
            Some(window_geometry) => window_geometry,
            None => {
                // Move window on center of screen
                let (monitor_width, monitor_height) =
                    events_loop.get_primary_monitor().get_dimensions();
                let x = (monitor_width - WINDOW_WIDTH) as i32 / 2;
                let y = (monitor_height - WINDOW_HEIGHT) as i32 / 2;
                (x, y, WINDOW_WIDTH, WINDOW_HEIGHT)
            }
        };

        // Create window
        let window = winit::WindowBuilder::new()
            .with_title(APP_NAME)
            .with_dimensions(width, height)
            .build(&events_loop)
            .unwrap();

        window.set_position(x, y);

        let renderer = Renderer::new(&window);

        Application {
            settings,
            events_loop,
            window,
            renderer,
        }
    }

    pub fn run(&mut self) {
        let mut now = SystemTime::now();
        let mut is_running = true;
        while is_running {
            self.events_loop.poll_events(|event| match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Resized(w, h),
                    ..
                } => {
                    println!("The window was resized to {}x{}", w, h);
                }
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => {
                    is_running = false;
                }
                _ => (),
            });

            let duration = now.elapsed().unwrap();
            const BILLION: u32 = 1_000_000_000;
            let dt = (duration.as_secs() as u32 * BILLION + duration.subsec_nanos()) as f32
                / BILLION as f32;
            now = SystemTime::now();

            self.update(dt);
            //            self.renderer.render();
        }
    }

    fn update(&self, _dt: f32) {
        //        println!("update: {}", dt)
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // Write window geometry to settings
        let (x, y) = self.window.get_position().unwrap();
        let (width, height) = self.window.get_inner_size().unwrap();
        self.settings.set_window_geometry(x, y, width, height);
    }
}

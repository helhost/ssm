use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::visuals::renderer::Renderer;

pub fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}

#[derive(Default)]
struct App {
    window: Option<&'static Window>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = match event_loop.create_window(
            WindowAttributes::default()
                .with_title("ssm")
                .with_inner_size(winit::dpi::LogicalSize::new(1100.0, 700.0)),
        ) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("failed to create window: {e}");
                event_loop.exit();
                return;
            }
        };

        // Make the window live for the entire program lifetime.
        let window: &'static Window = Box::leak(Box::new(window));

        let renderer = match pollster::block_on(Renderer::new(window)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("failed to create renderer: {e}");
                event_loop.exit();
                return;
            }
        };

        self.window = Some(window);
        self.renderer = Some(renderer);

        if let Some(w) = self.window {
            w.request_redraw();
        } else {
            eprintln!("internal error: window missing after creation");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                if let Some(r) = self.renderer.as_mut() {
                    r.resize(size.width, size.height);
                }

                if let Some(w) = self.window {
                    w.request_redraw();
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(r) = self.renderer.as_mut() {
                    if let Err(e) = r.render() {
                        eprintln!("render error: {e}");
                        event_loop.exit();
                        return;
                    }
                }
            }

            _ => {}
        }
    }
}

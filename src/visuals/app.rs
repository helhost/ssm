use std::time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    event::{
        ElementState,
        MouseButton,
        MouseScrollDelta,
        WindowEvent,
    },
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
    dragging: bool,
    last_cursor: Option<(f32, f32)>,

    last_click: Option<Instant>,
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
                    }
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    if state == ElementState::Pressed {
                        let now = Instant::now();
                        let is_double = self
                            .last_click
                            .map(|t| now.duration_since(t) < Duration::from_millis(300))
                            .unwrap_or(false);

                        self.last_click = Some(now);

                        if is_double {
                            if let (Some(r), Some((x, y))) =
                                (self.renderer.as_mut(), self.last_cursor)
                            {
                                if let Some(p) = r.pick_focus_point(x, y) {
                                    r.set_focus(p);
                                    if let Some(w) = self.window {
                                        w.request_redraw();
                                    }
                                }
                            }
                        }

                        self.dragging = true;
                        self.last_cursor = None;
                    } else {
                        self.dragging = false;
                        self.last_cursor = None;
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                let pos = (position.x as f32, position.y as f32);

                if self.dragging {
                    if let Some((lx, ly)) = self.last_cursor {
                        let dx = pos.0 - lx;
                        let dy = pos.1 - ly;

                        if let Some(r) = self.renderer.as_mut() {
                            r.on_camera_drag(dx, dy);
                        }

                        if let Some(w) = self.window {
                            w.request_redraw();
                        }
                    }
                }

                self.last_cursor = Some(pos);
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 * 0.01,
                };

                if let Some(r) = self.renderer.as_mut() {
                    r.on_camera_scroll(scroll);
                }

                if let Some(w) = self.window {
                    w.request_redraw();
                }
            }

            _ => {}
        }
    }
}

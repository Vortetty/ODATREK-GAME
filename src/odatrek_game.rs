use std::{sync::Arc, thread, time::{Duration, SystemTime}};

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::graphics_state::GraphicsState;

#[derive(Default)]
pub struct OdatrekGame {
    state: Option<GraphicsState>
}

impl ApplicationHandler for OdatrekGame {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop.create_window(
            Window::default_attributes()
                //.with_fullscreen(Some(Fullscreen::Borderless(None)))
                .with_active(true)
                .with_title("Test window")
                .with_inner_size(LogicalSize::new(1280, 720))
                .with_visible(true)
        );
        if window.is_ok() {
            println!("Window Created");

            let state = pollster::block_on(GraphicsState::new(Arc::new(window.unwrap())));
            self.state = Some(state);

            self.state.as_mut().unwrap().get_window().request_redraw();
        } else {
            println!("No windows sorry");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let _frame_time = state.render();

                // Emits a new redraw requested event.
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            _ => ()
        }
    }
}
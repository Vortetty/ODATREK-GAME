use odatrek_game::OdatrekGame;
use winit::event_loop::{ControlFlow, EventLoop};

mod odatrek_game;
mod graphics_state;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = OdatrekGame::default();
    let app_run = event_loop.run_app(&mut app);
    if app_run.is_ok() {
        println!("Exited");
    } else {
        println!("{:?}", app_run.err());
    }
}

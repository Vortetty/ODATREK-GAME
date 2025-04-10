use log::LevelFilter;
use odatrek_game::OdatrekGame;
use winit::event_loop::{ControlFlow, EventLoop};
use odatrek_logger::OdatrekLogger;

mod odatrek_game;
mod graphics_state;

static mut ODATREK_LOGGER: OdatrekLogger = OdatrekLogger::new();

fn main() {
    #[allow(static_mut_refs)]
    unsafe {
        ODATREK_LOGGER.init(
            "./logs".to_string(),
            Some(log::Level::Info),
            Some(log::Level::Error)
        );
        log::set_logger(&ODATREK_LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace));
    }

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = OdatrekGame::default();
    let app_run = event_loop.run_app(&mut app);
    if app_run.is_ok() {
        println!("Exited");
    } else {
        println!("{:?}", app_run.err());
    }

    #[allow(static_mut_refs)]
    unsafe {
        ODATREK_LOGGER.stow_log(None, None);
    }
}

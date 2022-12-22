mod api;

use std::fs::File;

use log::info;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use usdpl_back::{core::serdes::Primitive, Instance};

const PORT: u16 = 33220;

fn main() -> Result<(), ()> {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("/tmp/controller-tools.log").unwrap(),
        ),
    ]);
    info!("Starting backend ({} v{})", api::NAME, api::VERSION);

    Instance::new(PORT)
        .register("V_INFO", |_: Vec<Primitive>| {
            vec![format!("{} v{}", api::VERSION, api::VERSION).into()]
        })
        .register_blocking("get_battery_status", api::get_battery_status)
        .run_blocking()
}

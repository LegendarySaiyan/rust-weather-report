mod apps;
mod di;
mod infrastructure;
mod tests;
mod toolbox;

use log::{debug, info, LevelFilter};

use std::error::Error;
use tokio::time::{sleep, Duration};

use apps::weather_report::adapters::use_cases::update_usecases::ABCUpdatesUsecase;

use di::ApplicationIoC;
use infrastructure::settings::base::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new();
    env_logger::init();

    match settings {
        Err(..) => panic!("Can't initialize settings!"),
        Ok(_) => {
            let base = settings.unwrap();
            let ioc = ApplicationIoC::new(&base);

            debug!("Booting perfectly");
            loop {
                ioc.update_usecase.adopt_updates().await;
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

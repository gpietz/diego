extern crate diego;

use diego::core::application::Application;
use diego::core::diego_runtime;
use diego::log_info;
use diego::logging::console_logger::ConsoleTarget;

fn main() {
    diego_runtime::try_add_logger("Console", ConsoleTarget::default());
    log_info!("Hello World! *****1234");
    let mut application = Application::default();
    application.run();
}

use std::env;
use dr_engine::application::{Application, Definition};
use dr_engine::core::CommandLineArgs;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let application_definition =
        // start with default, sensible values
        Definition::default()
            // only specifying a title
            .with_title("DR-EDITOR".to_string())
            // overriding any defaults with data from the command line
            .override_with(CommandLineArgs::build(&env::args().collect()));

    Application::with_application_definition(application_definition)
        .run();
}

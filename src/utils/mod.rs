use names::Generator;
use std::env;

pub fn setup_logger() {
    let mode = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "ERROR"
    }
    .to_string();
    env::set_var("KARL_LOG", env::var("KARL_LOG").unwrap_or(mode));
    env_logger::init_from_env("KARL_LOG");
}

pub fn get_name() -> String {
    let mut generator = Generator::default();
    String::from(
        generator
            .next()
            .expect("New project name failed to generate!")
            .as_str(),
    )
    .replace('-', " ")
}

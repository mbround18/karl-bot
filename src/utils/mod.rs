use names::Generator;
use std::env;

pub fn setup_logger() {
    let mode = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "INFO"
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

pub fn get_var(name: &str) -> String {
    env::var(name)
        .expect(&format!("{} must be present!", name))
        .trim()
        .to_string()
        .replace("\"", "")
}

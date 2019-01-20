use clap::{App, Arg};
use config::{Config, Environment, File};
use std::path::Path;

fn main() {
    let matches = App::new("A dynamic wallpaper setter.")
        .version("0.1.0")
        .author("Siddharth Yadav <siddhart_yadav@outlook.com>")
        .about("Wallyd is a dynamic wallpaper setter based on the api provided by Unspalsh.com")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config path")
                .takes_value(true),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("wallyd.toml");
    let config_file_path = Path::new(config_file);

    let mut settings = Config::default();
    settings
        .merge(File::from(config_file_path))
        .unwrap()
        .merge(Environment::with_prefix("WALLYD"))
        .unwrap();

    let mut server = wally::Wally::new(&settings);

    server.run();
}

use wally;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config"))
        .unwrap()
        .merge(config::Environment::with_prefix("WALLYD"))
        .unwrap();

    let mut server = wally::Wally::new(&settings);

    server.run();
}

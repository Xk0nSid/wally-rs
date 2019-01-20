use std::fs::File;
use std::process::exit;
use std::process::Command;
use std::{thread, time};

/// Main structure that represents the wally daemon.
#[derive(Debug)]
pub struct Wally<'a> {
    settings: &'a config::Config,
    search_img_urls: Vec<String>,
    next_page: u8,
    quality: &'a str,
    tmp_filename: &'a str,
}

impl<'a> Wally<'a> {
    /// Create and return a new instance of Wally
    pub fn new(settings: &config::Config) -> Wally {
        // Get filename or set default filename
        let tmp_filename: &str = match settings.get("tmp_filename") {
            Ok(tf) => tf,
            Err(_) => "/tmp/rw.png",
        };

        // Default quality is full
        let quality: &str = match settings.get("quality") {
            Ok(q) => q,
            Err(_) => "full",
        };

        // Create and return Wally object
        Wally {
            settings,
            tmp_filename,
            search_img_urls: vec![],
            next_page: 1,
            quality: quality,
        }
    }

    /// Kick start the Wally daemon
    pub fn run(&mut self) {
        let mode: String = match self.settings.get("mode") {
            Ok(mode) => mode,
            Err(_) => String::from("random"),
        };
        let mut img_url: String;

        let query: String = match self.settings.get("query") {
            Ok(q) => q,
            Err(er) => {
                if mode == "search" {
                    eprintln!("Error: {:?}", er);
                    eprintln!("Invalid or not defined query. Query is required for `search` mode.");
                    exit(1);
                } else {
                    String::from("")
                }
            }
        };

        loop {
            // Get link to download image
            match mode.as_ref() {
                "search" => {
                    img_url = match self.search_img_urls.pop() {
                        Some(url) => url,
                        None => {
                            self.populate_search_results(query.as_str(), self.next_page);
                            self.update_page_number();
                            self.search_img_urls.pop().unwrap()
                        }
                    }
                }
                "random" => {
                    img_url = self.get_random_link();
                }
                _ => {
                    eprintln!("Invalid mode: {}", mode);
                    exit(1);
                }
            }

            // Download image
            self.download_image(img_url);

            // Set image as background
            let cmd = Command::new("feh")
                .env("DISPLAY", ":0")
                .arg("--bg-scale")
                .arg("/tmp/rw.png")
                .output()
                .expect("failed to set wallpaper");

            if cmd.status.success() {
                println!("Successfully set wallpaper");
            } else {
                eprintln!(
                    "Failed to execute feh: {}",
                    String::from_utf8(cmd.stderr).expect("invalid utf-8 bytes")
                );
            }

            // Sleep for configured time
            thread::sleep(time::Duration::from_secs(
                self.settings.get("delay").unwrap(),
            ));
        }
    }

    /// Update next page number in struct and also
    /// store it in application cache. This is done
    /// so as to be able to maintain page state even
    /// after process restart
    fn update_page_number(&mut self) {
        self.next_page += 1;
    }

    /// Fetches a random image url from unsplash API
    fn get_random_link(&self) -> String {
        let base_url: String = self.settings.get("api_base_url").unwrap();
        let client_id: String = self.settings.get("client_id").unwrap();
        let url = format!("{}/photos/random?client_id={}", base_url, client_id);
        let body = reqwest::get(&url).unwrap().text().unwrap();

        let response = json::parse(&body).unwrap();
        format!("{}", response["urls"][self.quality])
    }

    /// Populates vector `self.search_img_urls` with images from search result
    fn populate_search_results(&mut self, query: &str, page: u8) {
        let base_url: String = self.settings.get("api_base_url").unwrap();
        let client_id: String = self.settings.get("client_id").unwrap();
        let url = format!(
            "{}/search/photos?client_id={}&query={}&page={}",
            base_url,
            client_id,
            urlencoding::encode(query),
            page
        );
        let body = reqwest::get(&url).unwrap().text().unwrap();
        let response = json::parse(&body).unwrap();

        for res_ix in 0..10 {
            self.search_img_urls.push(format!(
                "{}",
                response["results"][res_ix]["urls"][self.quality]
            ));
        }
    }

    /// Downloads the image from provided URL to temporary folder
    fn download_image(&self, url: String) {
        let mut dest = File::create("/tmp/rw.png").unwrap();
        reqwest::get(&url)
            .unwrap()
            .copy_to(&mut dest)
            .expect("failed to download wallpaper");
    }
}

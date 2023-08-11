use chrono::Duration;
use lazy_static::lazy_static;
use reqwest::{self, Client, ClientBuilder};

lazy_static! {
    pub static ref WEB_CLIENT: Client = client();
}

pub fn client() -> Client {
    ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .connect_timeout(Duration::seconds(5).to_std().unwrap())
        .timeout(Duration::seconds(5).to_std().unwrap())
        .pool_idle_timeout(Duration::seconds(60).to_std().unwrap())
        .pool_max_idle_per_host(12)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

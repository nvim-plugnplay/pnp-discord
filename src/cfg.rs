use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Cfg {
    pub token: String,
    pub prefix: String,
}

impl Cfg {
    pub fn new() -> Self {
        Config::builder()
            .add_source(File::with_name("config"))
            .build()
            .unwrap()
            .try_deserialize::<Cfg>()
            .unwrap()
    }
}

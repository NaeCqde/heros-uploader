use dotenvy::dotenv_override;
use serde::Deserialize;

#[derive(Clone, Default, Deserialize, Debug)]
pub struct Env {
    // General
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

pub fn load() -> Result<Env, envy::Error> {
    // use dotenvy :)
    dotenv_override().ok();

    envy::from_env::<Env>()
}

use std::env;

#[derive(Debug)]
pub struct Config {
    pub token: String,
}

pub fn load_config() -> Config {
    let token_env = "OPEN_AI_API_TOKEN";

    let token = env::var(token_env)
        .expect(format!("The {:?} environment is not set.", token_env).as_str())
        .to_string();

    Config { token }
}

/// See `.env.sample` in the repository root for details.
#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database.
    #[arg(long, env)]
    pub database_url: String,

    /// The HMAC signing and verification key used for login tokens (JWTs).
    ///
    /// There is no required structure or format to this key as it's just fed into a hash function.
    /// In practice, it should be a long, random string that would be infeasible to brute-force.
    #[arg(long, env)]
    pub hmac_key: String,

    /// The host address the server will bind to.
    ///
    /// Use `0.0.0.0` to listen on all interfaces, or `127.0.0.1` to restrict to localhost only.
    #[arg(long, env, default_value = "127.0.0.1")]
    pub host: String,

    /// The port the server will listen on.
    #[arg(long, env, default_value = "13000")]
    pub port: u16,

    /// Directory for persistent application data storage outside the database.
    #[arg(long, env)]
    pub data_dir: Option<std::path::PathBuf>,

    /// Website password
    #[arg(long, env)]
    pub website_password: Option<String>,
}

impl Config {
    pub fn resolved_database_url(&self) -> &str {
        &self.database_url
    }
}

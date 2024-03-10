use std::env;

use dotenvy::dotenv;

pub struct Settings {
    pub postgres_dsn: String,
    pub redis_addr: String,
    pub redis_password: String,
    pub minio_addr: String,
    pub minio_public_addr: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_bucket_name: String,
    pub public_key: String,
    pub private_key: String,
    pub access_secret: String,
    pub refresh_secret: String,
}

fn env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} not found in environment variables.", key))
}

pub fn init_settings() -> Settings {
    dotenv().ok();

    Settings {
        postgres_dsn: env_var("POSTGRES_DSN"),
        redis_addr: env_var("REDIS_ADDR"),
        redis_password: env_var("REDIS_PASSWORD"),
        minio_addr: env_var("MINIO_ADDR"),
        minio_public_addr: env_var("MINIO_PUBLIC_ADDR"),
        minio_access_key: env_var("MINIO_ACCESS_KEY"),
        minio_secret_key: env_var("MINIO_SECRET_KEY"),
        minio_bucket_name: env_var("MINIO_BUCKET_NAME"),
        public_key: env_var("PUBLIC_KEY"),
        private_key: env_var("PRIVATE_KEY"),
        access_secret: env_var("ACCESS_SECRET"),
        refresh_secret: env_var("REFRESH_SECRET"),
    }
}
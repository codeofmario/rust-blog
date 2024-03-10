use redis::aio::Connection;

use crate::config::settings::Settings;

pub async fn init_redis(settings: Settings) -> Connection {
    let addr = settings.redis_addr.clone();
    let client = redis::Client::open(addr).expect("Redis failed to create client.");
    client.get_async_connection().await.expect("Redis failed to create connection.")
}
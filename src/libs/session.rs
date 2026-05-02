use redis::{Client, AsyncCommands};

pub fn connect(redis_url: &str) -> Client {
    Client::open(redis_url).expect("Failed to connect to Redis")
}

pub async fn set(client: &Client, key: &str, value: &str, ttl_secs: u64) -> redis::RedisResult<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.set_ex(key, value, ttl_secs).await
}

pub async fn get(client: &Client, key: &str) -> redis::RedisResult<Option<String>> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.get(key).await
}

pub async fn del(client: &Client, key: &str) -> redis::RedisResult<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.del(key).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_redis_connection_and_dummy_user() {
        dotenv().ok();
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        let client = connect(&redis_url);

        // ping
        let mut conn = client.get_multiplexed_async_connection().await.expect("Failed to connect");
        let pong: String = redis::cmd("PING").query_async(&mut conn).await.expect("Ping failed");
        assert_eq!(pong, "PONG");
        println!("Redis ping: {}", pong);

        // store dummy user (TTL 60s)
        let dummy = r#"{"id":"00000000-0000-0000-0000-000000000001","email":"dummy@oauthrs.dev","username":"dummy"}"#;
        set(&client, "user:dummy", dummy, 60).await.expect("Set failed");
        println!("Stored dummy user");

        // read it back
        let result = get(&client, "user:dummy").await.expect("Get failed");
        assert_eq!(result.as_deref(), Some(dummy));
        println!("Retrieved: {}", result.unwrap());
    }
}

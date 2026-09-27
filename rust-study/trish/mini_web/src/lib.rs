use mini_redis::client;

pub async fn start_server() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Basic set/get
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    // Set with expiration demo
    client.set("temp", "expires_soon".into()).await?;
    println!("Set temporary key 'temp'");

    // Multiple key operations
    client.set("key1", "value1".into()).await?;
    client.set("key2", "value2".into()).await?;
    client.set("key3", "value3".into()).await?;

    // Get multiple keys
    for key in &["key1", "key2", "key3", "nonexistent"] {
        let val = client.get(key).await?;
        match val {
            Some(v) => println!("{} = {:?}", key, v),
            None => println!("{} = (not found)", key),
        }
    }

    // Get the hello key again to confirm it exists
    if client.get("hello").await?.is_some() {
        println!("'hello' key exists");
    }

    // Since mini-redis doesn't support DEL natively,
    // we simulate by setting the key to empty and letting it expire
    client.set("temp", "".into()).await?;
    println!("Cleared 'temp'");

    // Publish to a channel
    for i in 0..3 {
        let msg = format!("message_{}", i);
        client.publish("notifications", msg.into()).await?;
    }
    println!("Published 3 messages to 'notifications' channel");
    Ok(())
}

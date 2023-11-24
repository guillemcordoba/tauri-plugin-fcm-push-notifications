use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = fcm::Client::new();

    let mut map = HashMap::new();
    map.insert("message", "Howdy!");

    let mut builder = fcm::MessageBuilder::new();
    builder.data(&map);

    let response = client
        .send(builder.finalize())
        .await
        .expect("Failed to send message");
}

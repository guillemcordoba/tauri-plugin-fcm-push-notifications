use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = fcm::Client::new();

    let mut map = HashMap::new();
    map.insert("message", "Howdyd22!");

    let mut builder = fcm::MessageBuilder::new(
                "AAAAyaYhFVc:APA91bGqI386NuwwKQ14mg1vJrxevvpQ5Yv2BoSF9KmL4o8vMzxALLuHmuGL-TkJgDJd546tYHedx9l0yMJgxIFJ42kZayCXc3f9tN_D0O8cK7eLvwvlCN1FmikaqsZaZ-Ws4irTfWDV",
                "f3kv1OJATH2VJFNJx309ho:APA91bG513J9c7iy-gwgsGzsD_-Arr04ifI39hY2gBac-VrF3b30JvrWDL1yXEySd8vaaNzonCBj-Ed2eT1yfMhAbNBxYzb1kN4eDIswh5yKWT_SDCDOketuezv2NLw3n0_gbqnwUzTM",
            );
    builder.data(&map);

    let response = client
        .send(builder.finalize())
        .await
        .expect("Failed to send message");
}

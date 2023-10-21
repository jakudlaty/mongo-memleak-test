pub use mongodb::{Client};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};

type OrderId = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderEntity {
    #[serde(rename = "_id")]
    pub id: OrderId,
}

#[tokio::main]
async fn main() {
    let options = ClientOptions::parse("mongodb://localhost:27017/memleak?authSource=admin&ssl=false&maxpoolsize=20").await.unwrap();
    let client = Client::with_options(options).unwrap();
    let database = client.default_database().unwrap();
    let orders = database.collection::<OrderEntity>("Orders");

    for _ in 0..10000000 {
        let _result = orders.find_one(doc! {"_id" : "12345"}, None).await.unwrap();
    }
}

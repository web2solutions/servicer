use warp::{http, Filter};
use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};


type Items = Vec<Item>;


#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    url: String,
    endpoints: Vec<String>,
    authorized_roles: Vec<String>,
}

#[derive(Clone)]
struct Store {
  service_list: Arc<RwLock<Items>>
}

impl Store {
    fn new() -> Self {
        Store {
            service_list: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

async fn add_service_list_item(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.service_list.write().push(Item {
            name: String::from(item.name),
            url: String::from(item.url),
            endpoints: item.endpoints,
            authorized_roles: item.authorized_roles,
        });


        Ok(warp::reply::with_status(
            "Added items to the service list",
            http::StatusCode::CREATED,
        ))
}


fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("services"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_service_list_item);
    
    warp::serve(add_items)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

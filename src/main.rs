// use aws_config::meta::region::RegionProviderChain;
// use aws_sdk_dynamodb::model::AttributeValue;

// async fn list_items(client: &Client) -> Result<(), Error> {
//     let resp = client.get_item()
//         .table_name(String::from("BillBackend-MainTable74195DAB-CEZ7U9Y4LZK0"))
//         .key("pk", AttributeValue::S(String::from("I#user123#1635601020000")))
//         .key("sk", AttributeValue::S(String::from("N#0000")))
//         .send().await?;
//
//     if let Some(item) = resp.item {
//         println!("   {:?}", item);
//     }
//
//     Ok(())
// }
//
// #[derive(Debug, Serialize)]
// struct SuccessResponse {
//     // pub body: String,
// }
//
// #[derive(Debug, Serialize)]
// struct FailureResponse {
//     // pub body: String,
// }
// type Response = Result<SuccessResponse, FailureResponse>;
//
// #[tokio::main]
// async fn main() -> Result<(), lambda_runtime::Error> {
//     let region_provider = RegionProviderChain::first_try(Region::new(options.region)).or_default_provider();
//     let shared_config = aws_config::from_env().region(region_provider).load().await;
//     let client = Client::new(&shared_config);
//     let arc_client = Arc::new(client);
//
//     lambda_runtime::run(handler_fn(move |_: Request, _ctx: Context| {
//         handler(arc_client.clone())
//     })).await?;
//
//     Ok(())
// }

mod adapters;

use std::sync::Arc;
use lambda_http::{IntoResponse, Request, RequestExt, service_fn};
use crate::adapters::{Config};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let config_result = Config::new()?;
    let arc_config_result = Arc::new(config_result);

    lambda_http::run(service_fn(move |r: Request| {
        fun(r, arc_config_result.clone())
    })).await?;

    Ok(())
}

async fn fun(request: Request, config_result: Arc<Config>) -> Result<impl IntoResponse, std::convert::Infallible> {
    Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .first("name")
            .unwrap_or_else(|| "stranger")
    ))
}

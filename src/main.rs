// TODO:
//  Lambda 1: call to rest endpoint with location triangle -> save in dynamodb
//  Lambda 2: on cron, check if anything in dynamo - if so, do call to url and see if charger is available -> if so, send email (and delete request in dynamo)
//  Components: lambda, cron, dynamo
//  optionally pass in a name or id of a charging point
//  CDK with Typescript, Rust, Github actions

// use std::env;
// use std::sync::Arc;
// use aws_config::meta::region::RegionProviderChain;
// use aws_sdk_dynamodb::{Client, Error, Region};
// use aws_sdk_dynamodb::model::AttributeValue;
// use lambda_runtime::{handler_fn, Context};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone)]
// struct Opt {
//     region: String,
//     table: String,
// }
//
// // ideally options is passed containing table name
// // but when using clone like for client, I get a partial moved error?
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
// #[derive(Deserialize)]
// struct Request {
//     // pub body: String,
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
//
// impl std::fmt::Display for FailureResponse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "failed")
//     }
// }
//
// impl std::error::Error for FailureResponse {}
//
// type Response = Result<SuccessResponse, FailureResponse>;
//
// #[tokio::main]
// async fn main() -> Result<(), lambda_runtime::Error> {
//     let options = Opt {
//         region: String::from("eu-west-1"),
//         table: String::from("BillBackend-MainTable74195DAB-CEZ7U9Y4LZK0"),
//     };
//
//     let region_provider = RegionProviderChain::first_try(Region::new(options.region)).or_default_provider();
//
//     let shared_config = aws_config::from_env().region(region_provider).load().await;
//
//     let client = Client::new(&shared_config);
//
//     let arc_client = Arc::new(client);
//
//     lambda_runtime::run(handler_fn(move |_: Request, _ctx: Context| {
//         handler(arc_client.clone())
//     })).await?;
//
//     Ok(())
// }

// use std::env;
use lambda_http::{IntoResponse, Request, RequestExt, service_fn};
use lambda_runtime::{Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let region = env::var("REGION")?;
    // let table = env::var("TABLE")?;
    // let func = service_fn(func);
    // lambda_runtime::run(func).await?;

    lambda_http::run(service_fn(hello)).await?;


    Ok(())
}

async fn hello(
    request: Request
) -> Result<impl IntoResponse, std::convert::Infallible> {
    let _context = request.lambda_context();

    Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .first("name")
            .unwrap_or_else(|| "stranger")
    ))
}

async fn func(request: Request) -> Result<Value, Error> {
    println!("{:?}", request);
    // let (event, _context) = event.into_parts();
    // let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello Sam!") }))
}

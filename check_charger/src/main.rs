mod adapters;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(flow)).await?;

    Ok(())
}


async fn flow(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // let (event, _context) = event.into_parts();
    // let first_name = event["firstName"].as_str().unwrap_or("world");

    // test
    let client = reqwest::blocking::Client::new();
    let data = "departure%5Blat%5D=50.844837&departure%5Blng%5D=4.39695&NELat=50.84691587&NELng=4.4037956&SWLat=50.84283428&SWLng=4.39634442";
    let res = client.post("https://nl.chargemap.com/json/charging/pools/get_from_areas")
        .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
        .body(data)
        .send()?;
    println!("{:?}", res);

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
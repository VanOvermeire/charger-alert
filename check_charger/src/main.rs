mod adapters;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(flow)).await
}

async fn flow(_: LambdaEvent<Value>) -> Result<Value, Error> {
    // TODO
    //  - for every row in our database
    //  - retrieve the NE and SW lat and lon
    //  - if we find any matches with free connectors
    //  - then send an SES message (configure env vars for this)
    //  - and delete the entry
    // (if not we will try again later)

    Ok(json!(
        { "message": "done" }
    ))
}

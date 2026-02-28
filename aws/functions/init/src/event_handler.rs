use lambda_runtime::{tracing, Error, LambdaEvent};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use layers::prelude::*;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
pub(crate)async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);
    init_rootly();
    let rootly = get_rootly();
    tracing::info!("Rootly initialized: {:?}", rootly.get_mutable_logs_file_name());
    Ok(())
}


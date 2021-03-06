#![cfg(feature = "sagemaker")]

extern crate rusoto_core;
extern crate rusoto_sagemaker;

use rusoto_core::Region;
use rusoto_sagemaker::{ListModelsInput, SageMaker, SageMakerClient};

#[test]
fn main() {
    let sm = SageMakerClient::new(Region::UsEast1);
    let req = ListModelsInput::default();
    let result = sm.list_models(req).sync().unwrap();
    println!("Got models: {:?}", result);
}

/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_qldb::model::PermissionsMode;
use aws_sdk_qldb::{Client, Error, Region, PKG_VERSION};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The default AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the ledger.
    #[structopt(short, long)]
    ledger: String,

    /// Whether to display additional runtime information.
    #[structopt(short, long)]
    verbose: bool,
}

/// Creates an Amazon QLDB ledger.
/// # Arguments
///
/// * `-l LEDGER` - The name of the ledger.
/// * `[-r REGION]` - The Region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        region,
        ledger,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    if verbose {
        println!("QLDB version: {}", PKG_VERSION);
        println!("Region:       {:?}", shared_config.region().unwrap());
        println!();
    }

    let result = client
        .create_ledger()
        .name(ledger)
        .permissions_mode(PermissionsMode::AllowAll)
        .send()
        .await?;

    println!("ARN: {}", result.arn.unwrap());

    Ok(())
}

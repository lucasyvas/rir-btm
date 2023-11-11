#![forbid(unsafe_code)]
#![warn(clippy::all)]

use anyhow::Result;
use config::Config;
use dotenvy::dotenv;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use strum::{Display, EnumString};
use tokio::try_join;

#[derive(Display, EnumString)]
enum RequestPath {
    #[strum(serialize = "headers")]
    Headers,
    #[strum(serialize = "ip")]
    Ip,
    #[strum(serialize = "user-agent")]
    UserAgent,
}

trait ResourceRequest {
    type Resource: DeserializeOwned;

    fn path(&self) -> RequestPath;
}

#[derive(Debug, Deserialize, Serialize)]
struct HeadersResource {
    headers: HashMap<String, String>,
}

struct HeadersRequest;

impl ResourceRequest for HeadersRequest {
    type Resource = HeadersResource;

    fn path(&self) -> RequestPath {
        RequestPath::Headers
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct IpResource {
    origin: IpAddr,
}

struct IpRequest;

impl ResourceRequest for IpRequest {
    type Resource = IpResource;

    fn path(&self) -> RequestPath {
        RequestPath::Ip
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct UserAgentResource {
    #[serde(rename = "user-agent")]
    user_agent: String,
}

struct UserAgentRequest;

impl ResourceRequest for UserAgentRequest {
    type Resource = UserAgentResource;

    fn path(&self) -> RequestPath {
        RequestPath::UserAgent
    }
}

async fn request_resource<R>(client: &Client, request: R) -> Result<R::Resource>
where
    R: ResourceRequest,
{
    let resource = client
        .get(format!("https://httpbin.org/{}", request.path()))
        .send()
        .await?
        .json::<R::Resource>()
        .await?;

    Ok(resource)
}

#[derive(Deserialize)]
struct Options {
    timeout_seconds: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let options = Config::builder()
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize::<Options>()?;

    let mut client_builder = reqwest::Client::builder().user_agent("reqwest");

    if let Some(seconds) = options.timeout_seconds {
        client_builder = client_builder.timeout(Duration::new(seconds, 0));
    }

    let client = client_builder.build()?;

    let resources = try_join!(
        request_resource(&client, HeadersRequest),
        request_resource(&client, IpRequest),
        request_resource(&client, UserAgentRequest),
    )?;

    println!("{:#?}", resources);

    Ok(())
}

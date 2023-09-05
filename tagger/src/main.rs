#![forbid(unsafe_code)]
#![warn(clippy::all)]

use anyhow::Result;
use cmd_lib::{run_cmd, run_fun};
use config::Config;
use dotenvy::dotenv;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
struct Env {
    image: String,
    tag: String,
}

#[derive(Debug)]
struct Image {
    _name: String,
    _tag: String,
    _id: String,
}

impl TryFrom<&[&str]> for Image {
    type Error = &'static str;

    fn try_from(items: &[&str]) -> std::result::Result<Self, Self::Error> {
        if items.len() < 3 {
            Err("Image requires at least 3 items to convert from")
        } else {
            Ok(Self {
                _name: items[0].to_string(),
                _tag: items[1].to_string(),
                _id: items[2].to_string(),
            })
        }
    }
}

fn main() -> Result<()> {
    dotenv().ok();

    let Env { image, tag } = Config::builder()
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()?;

    (run_cmd! {docker pull ${image}})?;
    (run_cmd! {docker tag ${image} ${tag}})?;
    (run_cmd! {docker rmi ${image}})?;

    println!("{:#?}", list_images()?);

    Ok(())
}

fn list_images() -> Result<Vec<Image>> {
    let images = (run_fun! {docker images | tail -n +2 | awk "{ print $1, $2, $3 }"})?
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| Image::try_from(line.split(" ").collect::<Vec<&str>>().as_slice()).ok())
        .filter_map(|image| image.into())
        .collect::<Vec<Image>>();

    Ok(images)
}

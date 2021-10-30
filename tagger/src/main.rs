#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

use anyhow::Result;
use cmd_lib::{run_cmd, run_fun};
use dotenv::dotenv;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
struct Env {
    image: String,
    tag: String,
}

#[derive(Debug)]
struct Image {
    name: String,
    tag: String,
    id: String,
}

impl TryFrom<Vec<&str>> for Image {
    type Error = &'static str;

    fn try_from(items: Vec<&str>) -> std::result::Result<Self, Self::Error> {
        if items.len() < 3 {
            Err("Image requires at least 3 items to convert from")
        } else {
            Ok(Self {
                name: items[0].to_string(),
                tag: items[1].to_string(),
                id: items[2].to_string(),
            })
        }
    }
}

fn main() -> Result<()> {
    dotenv().ok();

    let Env { image, tag } = envy::from_env()?;

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
        .map(|line| Image::try_from(line.split(" ").collect::<Vec<&str>>()).ok())
        .filter_map(|image| image.into())
        .collect::<Vec<Image>>();

    Ok(images)
}

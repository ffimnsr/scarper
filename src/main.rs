use log::{error, info};
use prettytable::{cell, color, format::consts, row, Attr, Cell, Row, Table};
use serde::Deserialize;
use std;
use std::fs::File;
use std::io::prelude::*;

use scarper::plugins::PluginManager;

#[derive(Deserialize, Default, Debug)]
struct Config {
    packages: Vec<Package>,
}

#[derive(Deserialize, Default, Debug)]
struct Package {
    name: Option<String>,
    location: Option<String>,
    version: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GithubRelease {
    tag_name: Option<String>,
}

fn parse(path: &str) -> Config {
    let mut config = String::new();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            return Config::default();
        }
    };

    file.read_to_string(&mut config)
        .unwrap_or_else(|err| panic!("Error while reading config: [{:#?}]", err));

    match toml::from_str(&config) {
        Ok(t) => t,
        Err(err) => panic!("Error while deserializing config: [{:#?}]", err),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let config = parse("ScarperWatch.toml");

    let client = reqwest::Client::builder()
        .user_agent("scarper/0.1")
        .build()?;

    let mut table = Table::new();
    table.set_titles(row!["Package Name", "Status"]);
    table.set_format(*consts::FORMAT_NO_LINESEP_WITH_TITLE);

    let mut pm = PluginManager::new();

    match pm.load_plugin("plugins\\plug_gimp.dll") {
        Ok(()) => info!("Plugin loaded"),
        Err(err) => info!("Plugin failed to load: {:?}", err),
    }

    for package in config.packages {
        let location = package.location.unwrap_or("unknown".to_string());
        let name = package.name.unwrap();
        let version = package.version;

        let mut loc = location.split(":");
        let location_type = loc.next();
        let location_uri = loc.next();

        match location_type {
            Some("github") => {
                let uri = format!(
                    "https://api.github.com/repos/{}/releases/latest",
                    location_uri.unwrap()
                );

                let json: GithubRelease = client.get(&uri).send().await?.json().await?;

                if json.tag_name == version {
                    table.add_row(Row::new(vec![
                        Cell::new(name.as_str()).with_style(Attr::Bold),
                        Cell::new("up-to date").with_style(Attr::ForegroundColor(color::GREEN)),
                    ]));
                } else {
                    table.add_row(Row::new(vec![
                        Cell::new(name.as_str()).with_style(Attr::Bold),
                        Cell::new(json.tag_name.unwrap().as_str())
                            .with_style(Attr::ForegroundColor(color::RED)),
                    ]));
                }
            }
            Some("package") => {
                unimplemented!();
            }
            Some("http") | Some("https") => {
                unimplemented!();
            }
            Some(_) | None => {
                error!("Invalid location please verify again the input location on the toml config");
            }
        }
    }

    table.printstd();
    pm.unload();

    Ok(())
}

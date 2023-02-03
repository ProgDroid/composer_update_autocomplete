use std::{borrow::ToOwned, env, fs, path::Path};

use anyhow::{anyhow, Result};
use rayon::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use walkdir::{DirEntry, WalkDir};

const ARGS_ENV_VAR: &str = "COMP_LINE";
const CURSOR_POSITION_ENV_VAR: &str = "COMP_POINT";
const COMPOSER_FILE_NAME: &str = "composer.json";

#[derive(Deserialize, Debug)]
struct ComposerModules {
    require: Value,
    #[serde(rename = "require-dev")]
    require_dev: Option<Value>,
}

fn get_current_arg() -> Option<String> {
    let cursor_position: usize = env::var(CURSOR_POSITION_ENV_VAR).ok()?.parse().ok()?;
    let args: String = env::var(ARGS_ENV_VAR)
        .ok()?
        .chars()
        .take(cursor_position)
        .collect();

    args.split_whitespace().map(ToOwned::to_owned).last()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

fn get_composer_location() -> Result<String> {
    let current_dir: String = env::current_dir()?.display().to_string();
    let dir_entry = WalkDir::new(current_dir)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .par_bridge()
        .filter_map(Result::ok)
        .find_any(|e| {
            Path::new(
                format!(
                    "{}/{}",
                    e.file_name().to_str().unwrap_or(""),
                    COMPOSER_FILE_NAME
                )
                .as_str(),
            )
            .is_file()
        });

    dir_entry.map_or_else(
        || Err(anyhow!("Could not find composer file")),
        |entry| {
            Ok(format!(
                "{}/{}",
                entry.file_name().to_str().unwrap_or(""),
                COMPOSER_FILE_NAME
            ))
        },
    )
}

fn get_composer_modules() -> Result<ComposerModules> {
    let current_dir_composer_file_content = fs::read_to_string(COMPOSER_FILE_NAME);
    if let Ok(c) = current_dir_composer_file_content {
        let modules: ComposerModules = serde_json::from_str(&c)?;
        return Ok(modules);
    }

    match get_composer_location() {
        Ok(path) => {
            let content: String = fs::read_to_string(path)?;
            let modules: ComposerModules = serde_json::from_str(&content)?;
            Ok(modules)
        }
        Err(e) => Err(e),
    }
}

#[allow(clippy::print_stdout)]
fn main() -> Result<()> {
    let current_arg = match get_current_arg() {
        Some(arg) => arg,
        None => {
            return Err(anyhow!("Could not get arguments"));
        }
    };

    let composer_modules = get_composer_modules()?;

    let require_keys: Vec<String> =
        composer_modules
            .require
            .as_object()
            .map_or_else(Vec::new, |obj| {
                obj.keys()
                    .filter(|key| current_arg == "update" || key.starts_with(&current_arg))
                    .cloned()
                    .collect()
            });

    let require_dev_keys: Vec<String> =
        composer_modules
            .require_dev
            .map_or_else(Vec::new, |require_dev| {
                require_dev.as_object().map_or_else(Vec::new, |obj| {
                    obj.keys()
                        .filter(|key| current_arg == "update" || key.starts_with(&current_arg))
                        .cloned()
                        .collect()
                })
            });

    for key in require_keys {
        println!("{key}");
    }

    for key in require_dev_keys {
        println!("{key}");
    }

    Ok(())
}

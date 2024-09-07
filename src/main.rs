#![allow(unused)]
use camino::Utf8PathBuf;
use os_checker_types::JsonOutput;
use serde::Serialize;
use std::{
    fs,
    io::{BufReader, BufWriter},
};

/// 架构下拉框之类每个页面的基础信息
mod basic;

/// 主页表格
mod home;

/// 文件树
mod file_tree;

mod utils;
pub use utils::Result;

#[cfg(feature = "batch")]
fn main() -> Result<()> {
    Ok(())
}

#[cfg(feature = "single")]
fn main() -> Result<()> {
    let file = fs::File::open("ui.json")?;
    let json: JsonOutput = serde_json::from_reader(BufReader::new(file))?;

    // Clear old data
    fs::remove_dir_all(BASE_DIR)?;
    println!("清理 {BASE_DIR}");

    // Write basic JSON
    write_to_file("", "basic", &basic::all(&json))?;
    for (repo, b) in basic::by_repo(&json) {
        write_to_file(&format!("repos/{}/{}", repo.user, repo.repo), "basic", &b)?;
    }

    // Write home JSON
    write_to_file(HOME_DIR, ALL_TARGETS, &home::all_targets(&json))?;
    for (target, nodes) in home::split_by_target(&json) {
        write_to_file(HOME_DIR, target, &nodes)?;
    }

    // Write file tree JSON
    let file_tree_all = file_tree::all_targets(&json);
    write_to_file(FILETREE_DIR, ALL_TARGETS, &file_tree_all)?;
    for filetree in file_tree_all.split_by_repo() {
        write_to_file(filetree.dir().as_str(), ALL_TARGETS, &filetree)?;
    }
    for (target, filetree) in file_tree::split_by_target(&json) {
        write_to_file(FILETREE_DIR, target, &filetree)?;

        // repo & targets
        for ftree in filetree.split_by_repo() {
            write_to_file(ftree.dir().as_str(), target, &ftree)?;
        }
    }

    Ok(())
}

fn print(t: &impl Serialize) {
    println!("{}", serde_json::to_string_pretty(t).unwrap());
}

const BASE_DIR: &str = "new_ui";
const HOME_DIR: &str = "home/split"; // FIXME: 去除 split
const FILETREE_DIR: &str = "filetree/split"; // FIXME: 去除 split
const ALL_TARGETS: &str = "All-Targets";

fn write_to_file<T: Serialize>(dir: &str, target: &str, t: &T) -> Result<()> {
    let mut path = Utf8PathBuf::from_iter([BASE_DIR, dir]);
    fs::create_dir_all(&path)?;

    path.push(format!("{target}.json"));
    let file = fs::File::create(&path)?;
    serde_json::to_writer(BufWriter::new(file), t)?;

    println!("{path} 写入成功");

    Ok(())
}

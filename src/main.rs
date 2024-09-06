use camino::Utf8PathBuf;
use os_checker_types::JsonOutput;
use serde::Serialize;
use std::io::{BufReader, BufWriter};

/// 架构下拉框之类每个页面的基础信息
mod basic;

/// 主页表格
mod home;

/// 文件树
mod file_tree;

mod utils;
pub use utils::Result;

fn main() -> Result<()> {
    let file = std::fs::File::open("ui.json")?;
    let json: JsonOutput = serde_json::from_reader(BufReader::new(file))?;

    // Write basic JSON
    write_to_file("", "basic", &basic::all(&json))?;
    for (repo, b) in basic::by_repo(&json) {
        write_to_file(&format!("repos/{}/{}", repo.user, repo.repo), "basic", &b)?;
    }

    // Write home JSON
    write_to_file("home", ALL_TARGETS, &home::all_targets(&json))?;
    for (target, nodes) in home::split_by_target(&json) {
        write_to_file("home", target, &nodes)?;
    }

    // Write file tree JSON
    let file_tree_all = file_tree::all_targets(&json);
    write_to_file("filetree", ALL_TARGETS, &file_tree_all)?;
    for (target, filetree) in file_tree::split_by_target(&json) {
        write_to_file("filetree", target, &filetree)?;
    }
    for filetree in file_tree_all.split_by_repo() {
        write_to_file(filetree.dir().as_str(), ALL_TARGETS, &filetree)?;
    }
    // TODO: repo & targets

    Ok(())
}

fn print(t: &impl Serialize) {
    println!("{}", serde_json::to_string_pretty(t).unwrap());
}

const ALL_TARGETS: &str = "All-Targets";

fn write_to_file<T: Serialize>(dir: &str, target: &str, t: &T) -> Result<()> {
    const BASE_DIR: &str = "new_ui";

    let mut path = Utf8PathBuf::from_iter([BASE_DIR, dir]);
    std::fs::create_dir_all(&path)?;

    path.push(format!("{target}.json"));
    let file = std::fs::File::create(&path)?;
    serde_json::to_writer(BufWriter::new(file), t)?;

    println!("{path} 写入成功");

    Ok(())
}

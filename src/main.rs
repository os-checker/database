#![allow(unused)]
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
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
    let paths = json_paths("batch")?;

    clear_base_dir()?;

    for path in &paths {
        dbg!(path);
        let json = &read_json(path)?;
        write_filetree(json)?;

        let batch = dbg!(path.file_stem().unwrap());
        write_batch_basic_home(json, batch)?;
    }

    // 把 batch config 合并
    {
        let src_dir = &Utf8PathBuf::from_iter([BASE_DIR, "batch", "basic"]);
        let target_dir = &Utf8PathBuf::from_iter([BASE_DIR]);
        if !target_dir.exists() {
            fs::create_dir_all(target_dir)?;
        }
        basic::write_batch(src_dir, target_dir)?;
    }

    // 把 batch home 合并
    {
        let home_dir = &Utf8PathBuf::from_iter([BASE_DIR, "batch", HOME_DIR]);
        let target_dir = &Utf8PathBuf::from_iter([BASE_DIR, HOME_DIR]);
        if !target_dir.exists() {
            fs::create_dir_all(target_dir)?;
        }
        for src_dir in subdir_paths(home_dir.as_str())? {
            home::write_batch(&src_dir, target_dir)?;
        }
    }

    #[cfg(feature = "clear_batch")]
    {
        let batch_dir = Utf8PathBuf::from_iter([BASE_DIR, "batch"]);
        println!("正在清除 {batch_dir}");
        fs::remove_dir_all(&batch_dir)?;
        println!("已清除 {batch_dir}");
    }

    Ok(())
}

/// 查找某个目录下面的 json 文件（不递归）
fn json_paths(dir: &str) -> Result<Vec<Utf8PathBuf>> {
    Ok(Utf8Path::new(dir)
        .read_dir_utf8()?
        .filter_map(|entry| {
            if let Ok(e) = entry {
                if e.file_type().ok()?.is_file() && e.path().extension() == Some("json") {
                    return Some(e.into_path());
                }
            }
            None
        })
        .sorted()
        .collect_vec())
}

/// 查找某个目录下面的目录（不递归）
fn subdir_paths(dir: &str) -> Result<Vec<Utf8PathBuf>> {
    Ok(Utf8Path::new(dir)
        .read_dir_utf8()?
        .filter_map(|entry| {
            if let Ok(e) = entry {
                if e.file_type().ok()?.is_dir() {
                    return Some(e.into_path());
                }
            }
            None
        })
        .sorted()
        .collect_vec())
}

fn write_batch_basic_home(json: &JsonOutput, batch: &str) -> Result<()> {
    // Write basic JSON
    write_to_file("batch/basic", batch, &basic::all(json))?;
    for (repo, b) in basic::by_repo(json) {
        // 仓库的 basic 数据不参与聚合
        write_to_file(&format!("repos/{}/{}", repo.user, repo.repo), "basic", &b)?;
    }

    // Write home JSON
    let mut home = Utf8PathBuf::from_iter(["batch", HOME_DIR, ALL_TARGETS]);
    write_to_file(home.as_str(), batch, &home::all_targets(json))?;
    for (target, nodes) in home::split_by_target(json) {
        home.set_file_name(target);
        write_to_file(home.as_str(), batch, &nodes)?;
    }

    Ok(())
}

fn read_json(path: &Utf8Path) -> Result<JsonOutput> {
    let file = fs::File::open(path)?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

/// Clear old data
fn clear_base_dir() -> Result<()> {
    if let Err(err) = fs::remove_dir_all(BASE_DIR) {
        eprintln!("{err:?}");
    }
    println!("清理 {BASE_DIR}");
    Ok(())
}

#[cfg(feature = "single")]
fn main() -> Result<()> {
    let json = read_json("ui.json")?;

    clear_base_dir()?;

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

    write_filetree(&json)?;

    Ok(())
}

/// 写入 filetree 和 repos 的 filetree 数据；这无需聚合
fn write_filetree(json: &JsonOutput) -> Result<()> {
    let file_tree_all = file_tree::all_targets(json);
    write_to_file(FILETREE_DIR, ALL_TARGETS, &file_tree_all)?;
    for filetree in file_tree_all.split_by_repo() {
        write_to_file(filetree.dir().as_str(), ALL_TARGETS, &filetree)?;
    }
    for (target, filetree) in file_tree::split_by_target(json) {
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

const BASE_DIR: &str = "ui";
const HOME_DIR: &str = "home/split"; // FIXME: 去除 split
const FILETREE_DIR: &str = "file-tree/split"; // FIXME: 去除 split
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

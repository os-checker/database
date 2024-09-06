use os_checker_types::JsonOutput;
use std::io::BufReader;

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

    basic::all(&json).print();
    basic::by_repo(&json).iter().for_each(|(r, b)| {
        r.print();
        b.print()
    });

    print(&home::all_targets(&json));

    let file_tree_all = file_tree::all_targets(&json);
    print(&file_tree_all);
    print(&file_tree_all.split_by_repo());

    Ok(())
}

fn print(t: &impl serde::Serialize) {
    println!("{}", serde_json::to_string_pretty(t).unwrap());
}

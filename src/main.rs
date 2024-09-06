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

    print(&home::nodes(&json));

    print(&file_tree::all(&json));

    Ok(())
}

fn print(t: &impl serde::Serialize) {
    serde_json::to_writer_pretty(std::io::stdout(), t).unwrap();
}

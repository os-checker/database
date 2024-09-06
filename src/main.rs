use os_checker_types::JsonOutput;
use std::io::BufReader;

/// 架构下拉框之类每个页面的基础信息
mod basic;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let file = std::fs::File::open("ui.json")?;
    let json: JsonOutput = serde_json::from_reader(BufReader::new(file))?;

    basic::home(&json).print();
    basic::repos(&json).iter().for_each(|(r, b)| {
        r.print();
        b.print()
    });

    Ok(())
}

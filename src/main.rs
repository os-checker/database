use os_checker_types::JsonOutput;
use std::io::BufReader;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let file = std::fs::File::open("ui.json")?;
    let json: JsonOutput = serde_json::from_reader(BufReader::new(file))?;
    dbg!(json);

    Ok(())
}

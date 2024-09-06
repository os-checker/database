#[cfg(test)]
pub fn ui_json() -> os_checker_types::JsonOutput {
    let file = std::fs::File::open("ui.json").unwrap();
    serde_json::from_reader(std::io::BufReader::new(file)).unwrap()
}

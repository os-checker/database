use os_checker_types::JsonOutput;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserRepo<'a> {
    user: &'a str,
    repo: &'a str,
}

impl<'a> UserRepo<'a> {
    pub fn print(self) {
        let Self { user, repo } = self;
        println!("{user}/{repo}");
    }
}

pub fn user_repo(json: &JsonOutput, pkg_idx: usize) -> UserRepo {
    let repo = &json.env.packages[pkg_idx].repo;
    UserRepo {
        user: &repo.user,
        repo: &repo.repo,
    }
}

#[cfg(test)]
pub fn ui_json() -> JsonOutput {
    let file = std::fs::File::open("ui.json").unwrap();
    serde_json::from_reader(std::io::BufReader::new(file)).unwrap()
}

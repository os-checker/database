use crate::utils::{user_repo_pkgidx, UserRepo};
use indexmap::IndexMap;
use itertools::Itertools;
use os_checker_types::{Cmd, JsonOutput, Kind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
pub struct Basic {
    targets: Targets,
    kinds: Kinds,
}

impl Basic {
    pub fn print(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap());
    }
}

/// 所有仓库的架构统计
pub fn all(json: &JsonOutput) -> Basic {
    let kinds = Kinds::new(json);
    let map = json.cmd.iter().into_group_map_by(|cmd| &*cmd.target_triple);
    let targets = Targets::from_map(map);
    Basic { targets, kinds }
}

/// 按仓库的架构统计
pub fn by_repo(json: &JsonOutput) -> Vec<(UserRepo, Basic)> {
    let kinds = Kinds::new(json);
    let map = json
        .cmd
        .iter()
        .into_group_map_by(|cmd| user_repo_pkgidx(json, cmd.package_idx));
    let mut v = Vec::<(UserRepo, Basic)>::with_capacity(map.len());

    for (user_repo, cmds) in map {
        let map_target = cmds
            .into_iter()
            .into_group_map_by(|cmd| &*cmd.target_triple);
        let targets = Targets::from_map(map_target);

        v.push((
            user_repo,
            Basic {
                targets,
                kinds: kinds.clone(),
            },
        ));
    }

    v
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct Targets {
    inner: Vec<Target>,
}

impl Targets {
    fn with_capacity(cap: usize) -> Self {
        let mut inner = Vec::<Target>::with_capacity(cap + 1);
        inner.push(Target::all_targets_with_0count());
        Targets { inner }
    }

    fn push(&mut self, triple: &str, cmds: &[&Cmd]) {
        let target = Target::new(triple, cmds);
        self.inner[0].count += target.count;
        self.inner.push(target);
    }

    fn from_map(map: HashMap<&str, Vec<&Cmd>>) -> Self {
        let mut targets = Targets::with_capacity(map.len());
        for (triple, cmds) in map {
            targets.push(triple, &cmds);
        }
        targets
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Target {
    pub triple: String,
    pub count: usize,
}

impl Target {
    fn all_targets_with_0count() -> Self {
        Target {
            triple: "All-Targets".to_owned(),
            count: 0,
        }
    }

    fn new(triple: &str, cmds: &[&Cmd]) -> Self {
        Target {
            triple: triple.to_owned(),
            count: cmds.iter().map(|c| c.count).sum(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Kinds {
    // #[serde(flatten)]
    // pub raw: RawKinds,
    pub columns: Vec<Column>,
}

impl Kinds {
    fn new(json: &JsonOutput) -> Self {
        Kinds {
            columns: columns(&json.env.kinds.order),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Column {
    pub field: Kind,
    pub header: String,
}

fn columns(order: &[Kind]) -> Vec<Column> {
    let rename = indexmap::indexmap! {
        Kind::Unformatted => "未格式化"
    };
    let mut checkers = IndexMap::<_, _, ahash::RandomState>::from_iter(
        order.iter().map(|kind| (*kind, kind.as_str())),
    );
    checkers.extend(rename);
    checkers
        .into_iter()
        .map(|(field, header)| Column {
            field,
            header: header.to_owned(),
        })
        .collect()
}

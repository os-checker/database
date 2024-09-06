use indexmap::IndexMap;
use itertools::Itertools;
use os_checker_types::{JsonOutput, Kind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Basic {
    pub targets: Targets,
    pub kinds: Kinds,
}

impl Basic {
    pub fn print(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap());
    }
}

pub fn home(json: &JsonOutput) -> Basic {
    let map = json.cmd.iter().into_group_map_by(|cmd| &*cmd.target_triple);
    let mut targets = Targets::with_capacity(map.len());

    for (triple, cmds) in map {
        let target = Target {
            triple: triple.to_owned(),
            count: cmds.iter().map(|c| c.count).sum(),
        };
        targets.push(target);
    }

    let kinds = Kinds {
        columns: columns(&json.env.kinds.order),
    };
    Basic { targets, kinds }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Targets {
    inner: Vec<Target>,
}

impl Targets {
    pub fn with_capacity(cap: usize) -> Self {
        let mut inner = Vec::<Target>::with_capacity(cap + 1);
        inner.push(Target::all_targets_with_0count());
        Targets { inner }
    }

    pub fn push(&mut self, target: Target) {
        self.inner[0].count += target.count;
        self.inner.push(target);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kinds {
    // #[serde(flatten)]
    // pub raw: RawKinds,
    pub columns: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
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

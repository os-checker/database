use crate::utils::{group_by, new_map_with_cap, repo_pkgidx, UserRepo};
use camino::Utf8Path;
use indexmap::IndexMap;
use os_checker_types::{Cmd, JsonOutput, Kind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

/// 读取 src_dir 的所有 JSON，合并成一个新的 JSON，并写到 target_dir。
/// 新 JSON 的文件名为 basic.json。
pub fn write_batch(src_dir: &Utf8Path, target_dir: &Utf8Path) -> crate::Result<()> {
    let vec_bytes = crate::json_paths(src_dir.as_str())?
        .into_iter()
        .map(std::fs::read)
        .collect::<Result<Vec<_>, _>>()?;

    let mut batch = Vec::<Basic>::with_capacity(24);
    for bytes in &vec_bytes {
        batch.push(serde_json::from_slice::<Basic>(bytes)?);
    }

    if batch.is_empty() {
        println!("无 batch basic 可合并");
        return Ok(());
    }

    let kinds = batch[0].kinds.clone();
    let targets = Targets::merge_batch(batch.into_iter().map(|basic| basic.targets).collect());
    let merged = Basic { targets, kinds };

    let path = target_dir.join("basic.json");
    let file = std::fs::File::create(&path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, &merged)?;

    println!("成功把 batch config 合并: src_dir={src_dir} merged={path}");
    Ok(())
}

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
    let map = group_by(&json.cmd, |cmd| &*cmd.target_triple);
    let targets = Targets::from_map(map);
    Basic { targets, kinds }
}

/// 按仓库的架构统计
pub fn by_repo(json: &JsonOutput) -> Vec<(UserRepo, Basic)> {
    let kinds = Kinds::new(json);
    let map = group_by(&json.cmd, |cmd| repo_pkgidx(json, cmd.package_idx));
    let mut v = Vec::<(UserRepo, Basic)>::with_capacity(map.len());

    for (user_repo, cmds) in map {
        let map_target = group_by(cmds, |cmd| &*cmd.target_triple);
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

    fn merge_batch(v: Vec<Self>) -> Self {
        let mut map = new_map_with_cap::<String, usize>(24);
        for targets in v {
            for Target { triple, count } in targets.inner {
                map.entry(triple)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
        // 降序排列
        map.sort_unstable_by(|k1, &v1, k2, &v2| (v2, &**k2).cmp(&(v1, &**k1)));

        let inner = map
            .into_iter()
            .map(|(triple, count)| Target { triple, count })
            .collect();
        Self { inner }
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

// [
//   {
//     "key": 0,
//     "data": {
//       "user": "kern-crates",
//       "repo": "ByteOS",
//       "total_count": 18,
//       "Cargo": 2,
//       "Clippy(Error)": 1,
//       "Clippy(Warn)": 15
//     },
//     "children": [
//       {
//         "key": 1,
//         "data": {
//           "user": "kern-crates",
//           "repo": "ByteOS",
//           "package": "kernel",
//           "total_count": 18,
//           "Cargo": 2,
//           "Clippy(Error)": 1,
//           "Clippy(Warn)": 15
//         }
//       }
//     ]
//   }
// ]

use crate::utils::{
    group_by, new_map_with_cap, pkg_cmdidx, repo_cmdidx, target_cmdidx, IndexMap, UserRepo,
    UserRepoPkg,
};
use os_checker_types::{Data as RawData, JsonOutput, Kind};
use serde::Serialize;

#[cfg(test)]
mod tests;

pub fn split_by_target(json: &JsonOutput) -> Vec<(&str, Vec<NodeRepo>)> {
    let group_by_target = group_by(&json.data, |d| target_cmdidx(json, d.cmd_idx));
    let mut v = Vec::with_capacity(group_by_target.len());

    for (target, data) in group_by_target {
        v.push((target, inner(json, &data)));
    }
    v
}

fn inner<'a>(json: &'a JsonOutput, data: &[&RawData]) -> Vec<NodeRepo<'a>> {
    let mut key = 0;

    // 按照 repo 分组
    let group_by_repo = group_by(data, |d| repo_cmdidx(json, d.cmd_idx));
    let mut nodes = Vec::with_capacity(group_by_repo.len());

    for (repo, data_repo) in group_by_repo {
        let repo_key = key;

        // 按照 pkg 分组
        let group_by_pkg = group_by(data_repo, |d| pkg_cmdidx(json, d.cmd_idx));
        let mut children = Vec::with_capacity(group_by_pkg.len());

        for (pkg, data) in group_by_pkg {
            // 按照 kind 分组
            let group_by_kind = group_by(data, |d| d.kind);
            let mut map = new_map_with_cap(group_by_kind.len());
            map.extend(group_by_kind.into_iter().map(|(kind, v)| (kind, v.len())));
            // 按照 count 降序排序
            map.sort_unstable_by(|_, a, _, b| b.cmp(a));
            key += 1;
            let count = Count::new(map);
            let total_count = count.total_count();
            let node_pkg = NodePkg {
                key,
                data: NodePkgData {
                    pkg,
                    total_count,
                    count,
                },
            };
            children.push(node_pkg);
        }

        // children 按照计数降序排列
        children.sort_unstable_by(|a, b| {
            (b.data.total_count, b.data.pkg.pkg).cmp(&(a.data.total_count, a.data.pkg.pkg))
        });

        let mut count = Count::empty();
        count.update(children.iter().map(|c| &c.data.count));
        let total_count = count.total_count();
        let node = NodeRepo {
            key: repo_key,
            data: NodeRepoData {
                repo,
                total_count,
                count,
            },
            children,
        };
        nodes.push(node);
        key += 1;
    }

    // 仓库按照计数降序排列
    nodes.sort_unstable_by(|a, b| {
        (b.data.total_count, b.data.repo).cmp(&(a.data.total_count, a.data.repo))
    });
    nodes
}

pub fn all_targets(json: &JsonOutput) -> Vec<NodeRepo> {
    let data: Vec<_> = json.data.iter().collect();
    inner(json, &data)
}

#[derive(Debug, Serialize)]
pub struct NodeRepo<'a> {
    key: usize,
    data: NodeRepoData<'a>,
    children: Vec<NodePkg<'a>>,
}

#[derive(Debug, Serialize)]
struct NodeRepoData<'a> {
    #[serde(flatten)]
    repo: UserRepo<'a>,
    total_count: usize,
    #[serde(flatten)]
    count: Count,
}

#[derive(Debug, Serialize)]
struct NodePkg<'a> {
    key: usize,
    data: NodePkgData<'a>,
}

#[derive(Debug, Serialize)]
struct NodePkgData<'a> {
    #[serde(flatten)]
    pkg: UserRepoPkg<'a>,
    total_count: usize,
    #[serde(flatten)]
    count: Count,
}

type CountInner = IndexMap<Kind, usize>;

#[derive(Debug, Serialize)]
#[serde(transparent)]
struct Count {
    map: CountInner,
}

impl Count {
    fn empty() -> Self {
        // FIXME: 在 os_checker_types 中定义这个数量
        Count {
            map: new_map_with_cap(10),
        }
    }

    fn new(map: CountInner) -> Self {
        Self { map }
    }

    fn merge(&mut self, other: &Self) {
        for (&kind, &count) in &other.map {
            self.map
                .entry(kind)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }

    fn update<'a, Iter>(&mut self, iter: Iter)
    where
        Iter: Iterator<Item = &'a Self>,
    {
        iter.for_each(|c| self.merge(c));
    }

    fn total_count(&self) -> usize {
        self.map.values().sum()
    }
}

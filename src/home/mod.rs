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

use crate::utils::{grou_by, new_map_with_cap, pkg_cmdidx, IndexMap, UserRepo, UserRepoPkg};
use itertools::Itertools;
use os_checker_types::Kind;
use serde::Serialize;

#[test]
fn serde_map() {
    let json = crate::utils::ui_json();

    // 按照 pkg 分组
    let group_by_pkg = grou_by(&json.data, |d| pkg_cmdidx(&json, d.cmd_idx));

    let mut key = 0;

    for (pkg, data) in group_by_pkg {
        // 按照 kind 分组
        let group_by_kind = grou_by(data, |d| d.kind);
        let mut map = new_map_with_cap(group_by_kind.len());
        map.extend(group_by_kind.into_iter().map(|(kind, v)| (kind, v.len())));
        // 按照计数排序（虽然不必要）
        // map.sort_by_cached_key(|_, &count| count);
        let node = NodeRepo {
            key,
            data: NodeRepoData {
                pkg: pkg.into_repo(),
                count: map,
            },
            children: vec![],
        };
        println!("{}", serde_json::to_string_pretty(&node).unwrap());
        key += 1;
    }
}

#[derive(Debug, Serialize)]
struct NodeRepo<'a> {
    key: usize,
    data: NodeRepoData<'a>,
    children: Vec<NodePkg<'a>>,
}

#[derive(Debug, Serialize)]
struct NodeRepoData<'a> {
    #[serde(flatten)]
    pkg: UserRepo<'a>,
    #[serde(flatten)]
    count: IndexMap<Kind, usize>,
}

#[derive(Debug, Serialize)]
struct NodePkg<'a> {
    key: usize,
    data: NodeRepoData<'a>,
}

#[derive(Debug, Serialize)]
struct NodePkgData<'a> {
    #[serde(flatten)]
    pkg: UserRepoPkg<'a>,
    #[serde(flatten)]
    count: IndexMap<Kind, usize>,
}

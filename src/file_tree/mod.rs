use crate::utils::{group_by, new_map_with_cap, pkg_cmdidx, IndexMap, UserRepoPkg};
use camino::Utf8Path;
use os_checker_types::{JsonOutput, Kind};
use serde::Serialize;

pub fn all(json: &JsonOutput) -> Vec<FileTree> {
    let kinds_order = &json.env.kinds.order;

    let group_by_pkg = group_by(&json.data, |d| pkg_cmdidx(json, d.cmd_idx));
    let mut v = Vec::with_capacity(group_by_pkg.len());

    for (pkg, data) in group_by_pkg {
        let count_pkg = data.len();

        let group_by_file = group_by(&data, |d| &*d.file);
        let mut reports = Vec::with_capacity(group_by_file.len());

        for (file, data_file) in group_by_file {
            let group_by_kind = group_by(data_file, |d| d.kind);
            let mut kinds = new_map_with_cap(group_by_kind.len());
            for (kind, data_kind) in group_by_kind {
                kinds.insert(kind, data_kind.iter().map(|d| &*d.raw).collect());
            }

            let count = kinds.values().map(|v: &Vec<_>| v.len()).sum();
            reports.push(RawReport { file, count, kinds });
        }

        v.push(FileTree {
            data: Data {
                pkg,
                count: count_pkg,
                raw_reports: reports,
            },
            kinds_order,
        });
    }

    v
}

#[derive(Debug, Serialize)]
pub struct FileTree<'a> {
    data: Data<'a>,
    kinds_order: &'a [Kind],
}

#[derive(Debug, Serialize)]
struct Data<'a> {
    #[serde(flatten)]
    pkg: UserRepoPkg<'a>,
    count: usize,
    raw_reports: Vec<RawReport<'a>>,
}

#[derive(Debug, Serialize)]
struct RawReport<'a> {
    file: &'a Utf8Path,
    count: usize,
    kinds: IndexMap<Kind, Vec<&'a str>>,
}

use std::sync::Arc;

use itertools::Itertools;

use crate::Symbol;

#[derive(Debug)]
pub struct PkgNode {
    pub path: Arc<[Symbol]>,
    pub children: Arc<[PkgNode]>,
}

fn from_pkgs(base_path: &[Symbol], pkgs: &[&[Symbol]]) -> Arc<[PkgNode]> {
    if pkgs.is_empty() {
        return Arc::new([]);
    }

    if pkgs.iter().filter(|p| p.first().is_some()).count() == 0 {
        return Arc::from([PkgNode {
            path: Arc::from(base_path),
            children: Arc::new([]),
        }]);
    }

    let groups = pkgs
        .iter()
        .filter(|p| p.first().is_some())
        .into_group_map_by(|p| p.first().unwrap());

    Arc::from_iter(groups.into_iter().map(|(k, v)| {
        let path = base_path.iter().chain(Some(k)).cloned().collect::<Vec<_>>();

        let pkgs = v
            .into_iter()
            .filter(|p| p.len() > 1)
            .map(|p| &p[1..])
            .collect::<Vec<_>>();

        let children = from_pkgs(&path, &pkgs);
        PkgNode {
            path: Arc::from(path),
            children,
        }
    }))
}

impl PkgNode {
    pub fn from_pkgs(pkgs: &[&[Symbol]]) -> Arc<[PkgNode]> {
        Arc::from([PkgNode {
            path: Arc::new([]),
            children: from_pkgs(&[], pkgs),
        }])
    }

    pub fn ident(&self) -> Option<Symbol> {
        self.path.last().cloned()
    }
}

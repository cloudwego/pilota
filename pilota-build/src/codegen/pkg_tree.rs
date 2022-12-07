use std::sync::Arc;

use faststr::FastStr;
use itertools::Itertools;

#[derive(Debug)]
pub struct PkgNode {
    pub path: Arc<[FastStr]>,
    pub children: Arc<[PkgNode]>,
}

fn from_pkgs(base_path: &[FastStr], pkgs: &[Arc<[FastStr]>]) -> Arc<[PkgNode]> {
    let groups = pkgs.iter().into_group_map_by(|p| p.first().unwrap());

    Arc::from_iter(groups.into_iter().map(|(k, v)| {
        let path = base_path
            .iter()
            .chain(Some(k).into_iter())
            .cloned()
            .collect::<Vec<_>>();

        let pkgs = v
            .into_iter()
            .filter(|p| p.len() > 1)
            .map(|p| Arc::from(&p[1..]))
            .collect::<Vec<_>>();

        let children = from_pkgs(&path, &pkgs);
        PkgNode {
            path: Arc::from(path),
            children,
        }
    }))
}

impl PkgNode {
    pub fn from_pkgs(pkgs: &[Arc<[FastStr]>]) -> Arc<[PkgNode]> {
        from_pkgs(&[], pkgs)
    }

    pub fn ident(&self) -> FastStr {
        self.path.last().unwrap().clone()
    }
}

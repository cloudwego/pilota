use std::path::PathBuf;

use normpath::PathExt;
use rustc_hash::FxHashSet;

use super::rir::NodeKind;
use crate::{
    DefId,
    db::{RirDatabase, RootDatabase},
    symbol::FileId,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RootSelection {
    All,
    Service(Vec<DefId>),
    Explicit(Vec<DefId>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionKind {
    All,
    Service,
    Explicit,
}

impl RootSelection {
    pub(crate) fn kind(&self) -> SelectionKind {
        match self {
            RootSelection::All => SelectionKind::All,
            RootSelection::Service(_) => SelectionKind::Service,
            RootSelection::Explicit(_) => SelectionKind::Explicit,
        }
    }
}

pub(crate) struct RootSelector<'a> {
    db: &'a RootDatabase,
    touches: Vec<(PathBuf, Vec<String>)>,
    touch_files: Vec<PathBuf>,
    input_files: Vec<FileId>,
    ignore_unused: bool,
}

impl<'a> RootSelector<'a> {
    pub(crate) fn new(
        db: &'a RootDatabase,
        touches: Vec<(PathBuf, Vec<String>)>,
        touch_files: Vec<PathBuf>,
        input_files: Vec<FileId>,
        ignore_unused: bool,
    ) -> Self {
        Self {
            db,
            touches,
            touch_files,
            input_files,
            ignore_unused,
        }
    }

    pub(crate) fn select(self) -> RootSelection {
        if !self.ignore_unused {
            RootSelection::All
        } else if !self.touch_files.is_empty() {
            self.touch_files_select()
        } else {
            let mut defs = self.service_select();
            if !self.touches.is_empty() {
                let mut touches_defs = self.touches_select();
                defs.append(&mut touches_defs);
            }
            RootSelection::Service(defs)
        }
    }

    fn touches_select(self) -> Vec<DefId> {
        self.touches
            .into_iter()
            .flat_map(|s| {
                let path = s.0.normalize().unwrap().into_path_buf();
                let file_id = *self.db.file_ids_map().get(&path).unwrap();
                s.1.into_iter()
                    .filter_map(|item_name| {
                        let def_id = self
                            .db
                            .files()
                            .get(&file_id)
                            .unwrap()
                            .items
                            .iter()
                            .find(|def_id| {
                                *self.db.item(**def_id).unwrap().symbol_name() == item_name
                            })
                            .cloned();
                        if let Some(def_id) = def_id {
                            Some(def_id)
                        } else {
                            println!(
                                "cargo:warning=touches_select `{item_name}` of `{}` not exists",
                                path.display(),
                            );
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn touch_files_select(self) -> RootSelection {
        let normalized_touch_files = self
            .touch_files
            .into_iter()
            .filter_map(|path| match path.normalize() {
                Ok(normalized) => Some(normalized.into_path_buf()),
                Err(err) => {
                    println!(
                        "cargo:warning=touch_files_select: failed to normalize path `{}`: {err}. \
                         The entry is ignored.",
                        path.display()
                    );
                    None
                }
            })
            .collect::<FxHashSet<_>>();

        let mut output = Vec::with_capacity(self.input_files.len());

        // `db.file_paths()` is expected to already contain normalized
        // absolute paths (see thrift/protobuf parsers which normalize
        // the input before inserting). We therefore match directly
        // without re-normalizing here.
        let mut matched_touch_files: FxHashSet<PathBuf> = FxHashSet::default();
        for file_id in &self.input_files {
            let Some(file_path) = self.db.file_paths().get(file_id) else {
                continue;
            };
            if !normalized_touch_files.contains(file_path.as_path()) {
                continue;
            }
            matched_touch_files.insert(file_path.as_path().to_path_buf());

            let file = self.db.file(*file_id).unwrap();
            file.items.iter().for_each(|def_id| {
                if let Some(node) = self.db.node(*def_id) {
                    if let NodeKind::Item(item) = &node.kind {
                        if !matches!(&**item, super::rir::Item::Mod(_)) {
                            output.push(*def_id);
                        }
                    }
                }
            });
        }

        for path in normalized_touch_files.difference(&matched_touch_files) {
            println!(
                "cargo:warning=touch_files_select path `{}` does not match any input file",
                path.display()
            );
        }

        RootSelection::Explicit(output)
    }

    fn service_select(&self) -> Vec<DefId> {
        let mut output = Vec::with_capacity(self.input_files.len());
        for file_id in &self.input_files {
            let file = self.db.file(*file_id).unwrap();
            file.items.iter().for_each(|def_id| {
                // Check if the node is an Item before calling item()
                if let Some(node) = self.db.node(*def_id) {
                    if let NodeKind::Item(item) = &node.kind {
                        if matches!(&**item, super::rir::Item::Service(_)) {
                            output.push(*def_id);
                        }
                    }
                }
            });
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use faststr::FastStr;
    use pilota::Bytes;
    use rustc_hash::FxHashMap;
    use tempfile::tempdir;

    use super::*;
    use crate::{
        middle::{
            ext::{FileExts, ItemExts, ModExts},
            rir::{self, Message, Mod, Service},
        },
        symbol::{DefId, FileId, Ident, Symbol},
        tags::TagId,
    };

    fn empty_package() -> rir::ItemPath {
        rir::ItemPath::from(Arc::<[Symbol]>::from(
            Vec::<Symbol>::new().into_boxed_slice(),
        ))
    }

    fn make_file(file_id: FileId, items: Vec<DefId>) -> Arc<rir::File> {
        Arc::new(rir::File {
            package: empty_package(),
            items,
            file_id,
            uses: Vec::new(),
            descriptor: Bytes::new(),
            extensions: FileExts::Thrift,
            comments: FastStr::new(""),
        })
    }

    fn make_message(name: &str) -> Arc<rir::Item> {
        Arc::new(rir::Item::Message(Message {
            name: Ident::from(name.to_string()),
            fields: Vec::new(),
            is_wrapper: false,
            item_exts: ItemExts::Thrift,
            leading_comments: FastStr::new(""),
            trailing_comments: FastStr::new(""),
        }))
    }

    fn make_service(name: &str) -> Arc<rir::Item> {
        Arc::new(rir::Item::Service(Service {
            name: Ident::from(name.to_string()),
            methods: Vec::new(),
            extend: Vec::new(),
            leading_comments: FastStr::new(""),
            trailing_comments: FastStr::new(""),
            item_exts: ItemExts::Thrift,
        }))
    }

    fn make_mod(name: &str, items: Vec<DefId>) -> Arc<rir::Item> {
        Arc::new(rir::Item::Mod(Mod {
            name: Ident::from(name.to_string()),
            items,
            extensions: ModExts::Thrift,
        }))
    }

    fn make_node(file_id: FileId, item: Arc<rir::Item>) -> rir::Node {
        rir::Node {
            file_id,
            kind: rir::NodeKind::Item(item),
            parent: None,
            tags: TagId::from_u32(0),
            related_nodes: Vec::new(),
        }
    }

    fn make_db(
        input_files: Vec<FileId>,
        files: Vec<(FileId, Arc<rir::File>)>,
        nodes: FxHashMap<DefId, rir::Node>,
        file_paths: Vec<(FileId, PathBuf)>,
    ) -> RootDatabase {
        let file_paths = file_paths
            .into_iter()
            .map(|(file_id, path)| {
                let path = path
                    .normalize()
                    .unwrap_or_else(|_| panic!("normalize path failed: {}", path.display()))
                    .into_path_buf();
                (file_id, Arc::new(path))
            })
            .collect::<FxHashMap<_, _>>();
        let file_ids_map = file_paths
            .iter()
            .map(|(file_id, path)| (path.clone(), *file_id))
            .collect::<FxHashMap<_, _>>();

        RootDatabase::default()
            .with_input_files(input_files)
            .with_files(files.into_iter())
            .with_nodes(nodes)
            .with_file_paths(file_paths)
            .with_file_ids_map(file_ids_map)
    }

    #[test]
    fn select_returns_all_when_unused_filter_disabled() {
        let db = RootDatabase::default();
        let selector = RootSelector::new(&db, vec![], vec![], vec![], false);
        assert_eq!(selector.select(), RootSelection::All);
    }

    #[test]
    fn select_prefers_touch_files_over_touches() {
        let file_id = FileId::from_u32(1);
        let message_id = DefId::from_u32(10);
        let service_id = DefId::from_u32(11);
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("root_selector_touches.thrift");
        std::fs::write(&path, "").unwrap();

        let mut nodes = FxHashMap::default();
        nodes.insert(message_id, make_node(file_id, make_message("Msg")));
        nodes.insert(service_id, make_node(file_id, make_service("Svc")));

        let db = make_db(
            vec![file_id],
            vec![(file_id, make_file(file_id, vec![message_id, service_id]))],
            nodes,
            vec![(file_id, path.clone())],
        );

        let selection = RootSelector::new(
            &db,
            vec![(path.clone(), vec!["Msg".to_string()])],
            vec![path],
            vec![file_id],
            true,
        )
        .select();

        assert_eq!(
            selection,
            RootSelection::Explicit(vec![message_id, service_id])
        );
    }

    #[test]
    fn select_appends_touches_to_service_roots_when_touch_files_are_absent() {
        let file_id = FileId::from_u32(5);
        let message_id = DefId::from_u32(50);
        let service_id = DefId::from_u32(51);
        let temp_dir = tempdir().unwrap();
        let path = temp_dir
            .path()
            .join("root_selector_touches_with_service.thrift");
        std::fs::write(&path, "").unwrap();

        let mut nodes = FxHashMap::default();
        nodes.insert(message_id, make_node(file_id, make_message("Msg")));
        nodes.insert(service_id, make_node(file_id, make_service("Svc")));

        let db = make_db(
            vec![file_id],
            vec![(file_id, make_file(file_id, vec![message_id, service_id]))],
            nodes,
            vec![(file_id, path.clone())],
        );

        let selection = RootSelector::new(
            &db,
            vec![(path, vec!["Msg".to_string()])],
            vec![],
            vec![file_id],
            true,
        )
        .select();

        assert_eq!(
            selection,
            RootSelection::Service(vec![service_id, message_id])
        );
    }

    #[test]
    fn select_collects_non_mod_items_from_touch_files() {
        let file_id = FileId::from_u32(2);
        let message_id = DefId::from_u32(20);
        let service_id = DefId::from_u32(21);
        let mod_id = DefId::from_u32(22);
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("root_selector_touch_files.thrift");
        std::fs::write(&path, "").unwrap();

        let mut nodes = FxHashMap::default();
        nodes.insert(message_id, make_node(file_id, make_message("Msg")));
        nodes.insert(service_id, make_node(file_id, make_service("Svc")));
        nodes.insert(mod_id, make_node(file_id, make_mod("Inner", Vec::new())));

        let db = make_db(
            vec![file_id],
            vec![(
                file_id,
                make_file(file_id, vec![message_id, mod_id, service_id]),
            )],
            nodes,
            vec![(file_id, path.clone())],
        );

        let selection = RootSelector::new(&db, vec![], vec![path], vec![file_id], true).select();

        assert_eq!(
            selection,
            RootSelection::Explicit(vec![message_id, service_id])
        );
    }

    #[test]
    fn select_returns_service_when_only_touches_exist_without_service_roots() {
        let file_id = FileId::from_u32(4);
        let message_id = DefId::from_u32(40);
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("root_selector_touches_only.thrift");
        std::fs::write(&path, "").unwrap();

        let mut nodes = FxHashMap::default();
        nodes.insert(message_id, make_node(file_id, make_message("Msg")));

        let db = make_db(
            vec![file_id],
            vec![(file_id, make_file(file_id, vec![message_id]))],
            nodes,
            vec![(file_id, path.clone())],
        );

        let selection = RootSelector::new(
            &db,
            vec![(path, vec!["Msg".to_string()])],
            vec![],
            vec![file_id],
            true,
        )
        .select();

        assert_eq!(selection, RootSelection::Service(vec![message_id]));
    }

    #[test]
    fn select_falls_back_to_services_when_no_touches_are_provided() {
        let file_id = FileId::from_u32(3);
        let message_id = DefId::from_u32(30);
        let service_id = DefId::from_u32(31);
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("root_selector_service.thrift");
        std::fs::write(&path, "").unwrap();

        let mut nodes = FxHashMap::default();
        nodes.insert(message_id, make_node(file_id, make_message("Msg")));
        nodes.insert(service_id, make_node(file_id, make_service("Svc")));

        let db = make_db(
            vec![file_id],
            vec![(file_id, make_file(file_id, vec![message_id, service_id]))],
            nodes,
            vec![(file_id, path)],
        );

        let selection = RootSelector::new(&db, vec![], vec![], vec![file_id], true).select();

        assert_eq!(selection, RootSelection::Service(vec![service_id]));
    }
}

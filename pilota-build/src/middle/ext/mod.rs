pub mod pb;

/// The extension for file
/// - Pb, the protobuf extension
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileExts {
    Pb(pb::FileExts),
    Thrift,
}

impl FileExts {
    pub fn has_extendees(&self) -> bool {
        match self {
            FileExts::Pb(pb::FileExts { extendees, .. }) => !extendees.is_empty(),
            FileExts::Thrift => false,
        }
    }

    pub fn has_used_options(&self) -> bool {
        match self {
            FileExts::Pb(pb::FileExts { used_options, .. }) => !used_options.is_empty(),
            FileExts::Thrift => false,
        }
    }

    pub fn unwrap_as_pb(&self) -> &pb::FileExts {
        match self {
            FileExts::Pb(exts) => exts,
            FileExts::Thrift => unreachable!(),
        }
    }
}

/// The extension for mod
/// - Pb, the protobuf extension
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ModExts {
    Pb(pb::ModExts),
    Thrift,
}

impl ModExts {
    pub fn has_extendees(&self) -> bool {
        match self {
            ModExts::Pb(pb::ModExts { extendees }) => !extendees.is_empty(),
            ModExts::Thrift => false,
        }
    }

    pub fn unwrap_as_pb(&self) -> &pb::ModExts {
        match self {
            ModExts::Pb(exts) => exts,
            ModExts::Thrift => unreachable!(),
        }
    }
}

/// The extension for item
/// - Pb, the protobuf extension
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ItemExts {
    Pb(pb::ItemExts),
    Thrift,
}

impl ItemExts {
    pub fn has_used_options(&self) -> bool {
        match self {
            ItemExts::Pb(pb::ItemExts { used_options }) => !used_options.is_empty(),
            ItemExts::Thrift => false,
        }
    }

    pub fn unwrap_as_pb(&self) -> &pb::ItemExts {
        match self {
            ItemExts::Pb(exts) => exts,
            ItemExts::Thrift => unreachable!(),
        }
    }
}

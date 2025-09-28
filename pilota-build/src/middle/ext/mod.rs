pub mod pb;

/// The extension for file
/// - Pb, the protobuf extension
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileExts {
    Pb(pb::FileExts),
    Thrift,
}

impl FileExts {
    pub fn is_empty(&self) -> bool {
        match self {
            FileExts::Pb(pb::FileExts { extendees }) => extendees.is_empty(),
            FileExts::Thrift => false,
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
    pub fn is_empty(&self) -> bool {
        match self {
            ModExts::Pb(pb::ModExts { extendees }) => extendees.is_empty(),
            ModExts::Thrift => false,
        }
    }
}

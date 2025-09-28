pub mod pb;

/// The extension for file
/// - Pb, the protobuf extension
#[derive(Clone, Debug)]
pub enum FileExts {
    Pb(pb::FileExts),
    Thrift,
}

/// The extension for mod
/// - Pb, the protobuf extension
#[derive(Clone, Debug)]
pub enum ModExts {
    Pb(pb::ModExts),
    Thrift,
}

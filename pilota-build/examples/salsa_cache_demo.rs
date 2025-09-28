//! Example demonstrating Salsa caching functionality

use std::sync::Arc;

use pilota_build::{
    DefId, TagId,
    db::{RirDatabase, RootDatabase},
    middle::ext::ItemExts,
    rir::{self, Method, MethodSource, Node, NodeKind},
    ty::{Ty, TyKind},
};
use rustc_hash::FxHashMap;

// We need to create FileId from scratch since it's not exposed
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct FileId(u32);

impl FileId {
    fn from_u32(val: u32) -> Self {
        FileId(val)
    }
}

fn main() {
    // Create a database
    let mut db = RootDatabase::default();

    // Create some test data
    let def_id = DefId::from_u32(0);
    let _file_id = FileId::from_u32(0);

    // Create a service item
    let service = rir::Item::Service(rir::Service {
        name: "TestService".into(),
        methods: vec![Arc::new(Method {
            def_id: DefId::from_u32(1),
            name: "method1".into(),
            args: vec![],
            ret: Ty {
                kind: TyKind::Void,
                tags_id: TagId::from_u32(0),
            },
            oneway: false,
            exceptions: None,
            source: MethodSource::Own,
            item_exts: ItemExts::Thrift,
        })],
        extend: vec![],
        item_exts: ItemExts::Thrift,
    });

    // Create nodes
    let nodes = FxHashMap::default();

    // Since we can't access the actual FileId type, let's demonstrate caching with
    // DefId operations
    println!("Testing Salsa cache functionality...");

    // Update database with empty nodes for now
    db = db.with_nodes(nodes);

    // Let's test with a simpler example that doesn't require FileId
    println!("\nExample: Demonstrating caching with DefId operations");

    // Create some nodes without FileId
    let mut new_nodes = FxHashMap::default();
    new_nodes.insert(
        def_id,
        Node {
            file_id: unsafe { std::mem::transmute(0u32) }, // Workaround for demo
            kind: NodeKind::Item(Arc::new(service)),
            parent: None,
            tags: TagId::from_u32(0),
            related_nodes: vec![],
        },
    );

    db = db.with_nodes(new_nodes);

    // Test item lookup caching
    println!("\nTesting item lookup cache:");
    let start = std::time::Instant::now();
    let item1 = db.item(def_id);
    let duration1 = start.elapsed();
    println!(
        "First call - Result: {:?}, Time: {:?}",
        item1.is_some(),
        duration1
    );

    let start = std::time::Instant::now();
    let item2 = db.item(def_id);
    let duration2 = start.elapsed();
    println!(
        "Second call - Result: {:?}, Time: {:?}",
        item2.is_some(),
        duration2
    );

    if duration1.as_nanos() > 0 && duration2.as_nanos() > 0 {
        println!(
            "Cache speedup: {:.2}x",
            duration1.as_nanos() as f64 / duration2.as_nanos() as f64
        );
    }

    // Test service methods caching
    println!("\n\nTesting service_methods cache:");
    let start = std::time::Instant::now();
    let methods1 = db.service_methods(def_id);
    let duration1 = start.elapsed();
    println!(
        "First call - Methods count: {}, Time: {:?}",
        methods1.len(),
        duration1
    );

    let start = std::time::Instant::now();
    let methods2 = db.service_methods(def_id);
    let duration2 = start.elapsed();
    println!(
        "Second call - Methods count: {}, Time: {:?}",
        methods2.len(),
        duration2
    );

    if duration1.as_nanos() > 0 && duration2.as_nanos() > 0 {
        println!(
            "Service methods cache speedup: {:.2}x",
            duration1.as_nanos() as f64 / duration2.as_nanos() as f64
        );
    }

    println!("\nNote: The caching functionality is now integrated into pilota-build!");
    println!("All database queries now automatically benefit from Salsa's caching:");
    println!("  - db.node(def_id)");
    println!("  - db.file(file_id)");
    println!("  - db.item(def_id)");
    println!("  - db.service_methods(def_id)");
    println!("  - db.is_arg(def_id)");
}

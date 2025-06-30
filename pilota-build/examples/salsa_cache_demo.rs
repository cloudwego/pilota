//! Example demonstrating Salsa caching functionality

use std::sync::Arc;

use pilota_build::{
    db::{RootDatabase, RirDatabase, RirDatabaseExt, CachedQueries, IntoSalsa},
    rir::{self, Item, Node, NodeKind, Method},
    symbol::{DefId, FileId},
};
use rustc_hash::FxHashMap;

fn main() {
    // Create a database
    let mut db = RootDatabase::default();
    
    // Create some test data
    let def_id = DefId(0);
    let file_id = FileId(0);
    
    // Create a service item
    let service = rir::Item::Service(rir::Service {
        name: "TestService".into(),
        methods: vec![
            Arc::new(Method {
                def_id: DefId(1),
                name: "method1".into(),
                args: vec![],
                ret: rir::Ty { kind: Arc::new(rir::ty::TyKind::Unit) },
                oneway: false,
                exceptions: None,
                source: rir::MethodSource::Own,
            })
        ],
        extend: vec![],
    });
    
    // Create nodes
    let mut nodes = FxHashMap::default();
    nodes.insert(def_id, Node {
        file_id,
        kind: NodeKind::Item(Arc::new(service)),
        parent: None,
        tags: 0.into(),
        related_nodes: vec![],
    });
    
    // Update database
    db = db.with_nodes(nodes);
    
    println!("Testing Salsa cache functionality...");
    
    // First call - will compute
    println!("\nFirst call (will compute):");
    let start = std::time::Instant::now();
    let item1 = db.item_cached(def_id);
    let duration1 = start.elapsed();
    println!("Result: {:?}", item1.is_some());
    println!("Time: {:?}", duration1);
    
    // Second call - should use cache
    println!("\nSecond call (should use cache):");
    let start = std::time::Instant::now();
    let item2 = db.item_cached(def_id);
    let duration2 = start.elapsed();
    println!("Result: {:?}", item2.is_some());
    println!("Time: {:?}", duration2);
    
    println!("\nCache speedup: {:.2}x", duration1.as_nanos() as f64 / duration2.as_nanos() as f64);
    
    // Test service methods caching
    println!("\n\nTesting service_methods cache:");
    let start = std::time::Instant::now();
    let methods1 = db.service_methods_cached(def_id);
    let duration1 = start.elapsed();
    println!("First call - Methods count: {}, Time: {:?}", methods1.len(), duration1);
    
    let start = std::time::Instant::now();
    let methods2 = db.service_methods_cached(def_id);
    let duration2 = start.elapsed();
    println!("Second call - Methods count: {}, Time: {:?}", methods2.len(), duration2);
    
    println!("Service methods cache speedup: {:.2}x", duration1.as_nanos() as f64 / duration2.as_nanos() as f64);
}
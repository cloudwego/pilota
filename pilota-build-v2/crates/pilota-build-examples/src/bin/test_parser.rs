//! Test parser example.

use pilota_build_common::{FileId, SourceMap};
use pilota_build_parser::parse_file;

fn main() {
    let mut source_map = SourceMap::new();
    
    // Test Thrift parsing
    let thrift_content = r#"
namespace rust example

struct User {
    1: required i32 id;
    2: required string name;
    3: optional string email;
}

service UserService {
    User getUser(1: i32 id);
    void createUser(1: User user);
}

enum Status {
    ACTIVE = 1,
    INACTIVE = 2,
    DELETED = 3
}
"#;

    let file_id = source_map.add_file("test.thrift", thrift_content.to_string());
    
    match parse_file(file_id, thrift_content, "test.thrift") {
        Ok(hir) => {
            println!("✅ Successfully parsed Thrift file!");
            println!("Found {} items", hir.items.len());
            
            for item in &hir.items {
                use pilota_build_hir::ItemKind;
                match &item.kind {
                    ItemKind::Message(msg) => {
                        println!("  - Message: {} with {} fields", msg.name, msg.fields.len());
                    }
                    ItemKind::Service(svc) => {
                        println!("  - Service: {} with {} methods", svc.name, svc.methods.len());
                    }
                    ItemKind::Enum(e) => {
                        println!("  - Enum: {} with {} variants", e.name, e.variants.len());
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Parse error: {}", e);
        }
    }
}
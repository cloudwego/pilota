use pilota_thrift_parser::parser::Parser;

fn main() {
    let input = r#"
    // This is a test struct
    struct TestStruct {
        // This is a test field
        1: required i32 test_field,
    }
    "#;
    
    let result = pilota_thrift_parser::File::parse(input);
    match result {
        Ok((remaining, file)) => {
            println!("Parsed successfully!");
            println!("Remaining: {:?}", remaining);
            println!("File items: {}", file.items.len());
            
            if let Some(item) = file.items.first() {
                if let pilota_thrift_parser::Item::Struct(s) = item {
                    println!("Struct name: {}", s.name.0);
                    println!("Struct comments: {:?}", s.comments);
                    if let Some(field) = s.fields.first() {
                        println!("Field name: {}", field.name.0);
                        println!("Field comments: {:?}", field.comments);
                    }
                }
            }
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}
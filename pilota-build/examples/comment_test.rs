use pilota_build::Builder;

fn main() {
    // 这个示例演示了如何使用pilota-build生成包含注释的Rust代码
    Builder::thrift()
        .input("test_comments.thrift")
        .compile(["."], pilota_build::Output::File("generated_with_comments.rs".into()));
    
    println!("代码生成完成！生成的Rust代码现在包含了来自Thrift IDL的注释作为文档注释。");
} 
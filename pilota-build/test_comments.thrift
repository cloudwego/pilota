// 这是一个测试服务
service TestService {
    // 这是一个测试方法
    // 它返回一个字符串
    string testMethod(
        // 这是输入参数
        1: string input
    );
}

// 这是一个测试结构体
struct TestStruct {
    // 这是第一个字段
    1: required string name;
    // 这是第二个字段  
    2: optional i32 age;
}

// 这是一个测试枚举
enum TestEnum {
    // 第一个值
    VALUE1 = 1,
    // 第二个值
    VALUE2 = 2,
} 
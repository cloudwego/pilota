namespace rs contract_common
struct Value { 1: required i32 number }
exception Failure { 1: required i32 number }
service Parent { Value Inherited(3: Value request) }

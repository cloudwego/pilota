include "common.thrift"
include "left.thrift"
include "right.thrift"
namespace rs contract_api
service Child extends common.Parent {
    left.Value Call(2: left.Value left, 9: right.Value right) throws (7: common.Failure failure)
    common.Value Shared(1: common.Value request)
}

include "codec.thrift"
namespace go extended
namespace rs extended
service Extended extends codec.CodecService {
    codec.CodecMessage Extra(1: codec.CodecMessage request) (streaming.mode="server")
}

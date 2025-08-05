include "image.thrift"
include "common.thrift"

namespace rs author

struct Author {
    1: required i64 id,
    2: required string username,
    3: required string email,
    4: required image.Image avatar(pilota.rust_wrapper_arc="true"),
    5: required common.CommonData common_data,
}

struct GetAuthorRequest {
    1: required i64 id,
}

struct GetAuthorResponse {
    1: required Author author,
}

service AuthorService {
    GetAuthorResponse GetAuthor(1: GetAuthorRequest req),
}
// namespace declaration
namespace rs volo.rpc.example

/*
 * Item struct represents an item with id, title, content, and extra metadata
 */

// This is a comment for the Item struct
struct Item {
    // id of the item
    1: required i64 id,                     // id of the item

    /*
     * title of the item
     */
    2: required string title,               // trailing comment test
    // content of the item
    3: required string content,             # trailing comment
    // extra metadata of the item
    10: optional map<string, string> extra, // trailing comment
}

// Status enum represents the status of an operation
enum Status {
    // Success status
    SUCCESS = 0,
    // Error status
    ERROR = 1,
}

// GetItemRequest struct represents the request for getting an item
struct GetItemRequest {
    1: required i64 id,
}

// GetItemResponse struct represents the response for getting an item
struct GetItemResponse {
    1: required Item item,
    2: required Status status,
}

// Test Service
// This is a comment for the TestService
service TestService {
    // method to get an item
    GetItemResponse getItem(1: GetItemRequest req),
}

// File comments test
// Another file comment line
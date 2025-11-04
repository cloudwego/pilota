enum B {
    Read = 1;
    Write = 2;
}

typedef i32 CommitId

typedef list<CommitId> CommitIdList

typedef string NameId

typedef i64 Score

typedef map<NameId, Score> NameScoreMap

const string A_S = "string";

const map<string, i32> STR_I32_MAP = {"hello": 1, "world": 2};

const map<string, i32> STR_I32_BTREE_MAP = {"alpha": 10, "beta": 20}(pilota.rust_type = "btree");

const set<i32> INT_SET_CONST = [7, 8];

const set<i32> INT_BTREE_SET_CONST = [9, 10](pilota.rust_type = "btree");

const binary DEFAULT_BINARY = "bin";

const CommitIdList DEFAULT_COMMIT_IDS = [1, 2];

const NameScoreMap NAME_SCORE_LITERAL = {"alice": 3, "bob": 4};

const NameScoreMap NAME_SCORE_EMPTY = [];

struct C {
    1: string off = "off",
    2: optional byte test_byte = 0,
}

const C DEFAULT_C = {"off": "const", "test_byte": 9};

struct A {
    1: required string faststr = "hello world",
    2: required string string = "test"(pilota.rust_type = "string"),
    3: optional bool a = false,
    4: optional B test_b = B.Read,
    5: optional B test_b2 = 2,
    6: optional i8 test_b3 = B.Read,
    7: optional map<string, string> map = {"hello": "world"},
    8: optional double test_double = 1,
    9: optional double test_double2 = 1.2,
    10: optional string alias_str = A_S,
    11: required binary empty = "",
    12: required map<double, double> test_map = {1.0: 2.0},
    13: required set<double> test_set = [1.0],
    14: bool a2 = 3,
    15: map<string, string> map2 = [],
    16: optional list<i32> commit_ids_raw = DEFAULT_COMMIT_IDS,
    17: optional CommitIdList commit_ids = [3, 4],
    18: optional i16 default_i16 = 16,
    19: optional i64 default_i64 = 64,
    20: optional list<i32> list_literal = [5, 6, 7],
    21: optional set<i32> set_empty = [],
    22: optional set<i32> set_from_const = INT_SET_CONST,
    23: optional set<i32> btree_set_literal = [4, 5](pilota.rust_type = "btree"),
    24: optional set<i32> btree_set_empty = [](pilota.rust_type = "btree"),
    25: optional set<i32> btree_set_from_const = INT_BTREE_SET_CONST(pilota.rust_type = "btree"),
    26: optional map<string, i32> map_literal_i32 = {"one": 1, "two": 2},
    27: optional map<string, i32> map_from_const = STR_I32_MAP,
    28: optional map<string, i32> btree_map_literal = {"three": 3}(pilota.rust_type = "btree"),
    29: optional map<string, i32> btree_map_empty = [](pilota.rust_type = "btree"),
    30: optional map<string, i32> btree_map_from_const = STR_I32_BTREE_MAP(pilota.rust_type = "btree"),
    31: optional C struct_literal = {"off": "nested", "test_byte": 7},
    32: optional C struct_partial = {"off": "partial"},
    33: optional C struct_from_const = DEFAULT_C,
    34: optional binary binary_from_const = DEFAULT_BINARY,
    35: optional NameScoreMap newtype_map_literal = {"carol": 5},
    36: optional NameScoreMap newtype_map_from_const = NAME_SCORE_LITERAL,
    37: optional NameScoreMap newtype_map_empty = [],
    38: optional NameScoreMap newtype_map_from_empty_const = NAME_SCORE_EMPTY,
}
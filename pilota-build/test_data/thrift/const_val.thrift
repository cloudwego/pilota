enum Index {
    A = 0,
    B = 1,
}

const map<Index, string> TEST_MAP = {
    Index.A: "hello",
    Index.B: "world",
};


const list<string> TEST_LIST = [
    "hello",
    "world",
];


const map<i32, list<string>> TEST_MAP_LIST = {
    1: ["hello"]
}
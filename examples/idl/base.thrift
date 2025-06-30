namespace rs base.loop

struct TrafficEnv {
	0: string Name = "",
	1: bool Open = false,
	2: string Env = "",
	256: required i64 Code,
}

struct Base {
	0: required string Addr = "",
	1: string LogID = "",
	2: string Caller = "",
	5: optional TrafficEnv TrafficEnv,
	9: Ex Enum,
	10: map<Ex, string> EnumMap,
	255: optional ExtraInfo Extra,
	256: MetaInfo Meta,
}

struct ExtraInfo {
	1: map<string, string> F1
	2: map<i64, string> F2,
	3: list<string> F3
	4: set<string> F4,
	5: map<double, Val> F5
	6: map<Int, Key> F6
	7: map<Int, map<Int, Key>> F7
	8: map<Int, list<Key>> F8
	9: map<Int, list<map<Int, Key>>> F9
	10: map<Val, Key> F10
}

struct MetaInfo {
	1: map<Int, Val> IntMap,
	2: map<Str, Key> StrMap,
	3: list<Key> List,
	4: set<Val> Set,
	11: map<Int, list<Str>> MapList
	12: list<map<Int, list<Str>>> ListMapList
	255: Base Base,
}

typedef Val Key 

struct Val {
	1: string id
	2: string name
}

typedef double Float

typedef i64 Int

typedef string Str

enum Ex {
	A = 1,
	B = 2,
	C = 3
}
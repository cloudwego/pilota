typedef i32 TypeB

const TypeB TypeB1 = 1
const TypeB TypeB2 = 2

typedef string TypeA

const TypeA TypeA1 = "a1"
const TypeA TypeA2 = "a2"

const map<TypeB, TypeA> TypeAMap = {
    TypeB1: TypeA1,
    TypeB2: TypeA2,
}

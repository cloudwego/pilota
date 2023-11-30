use std::sync::Arc;

use fxhash::FxHashMap;

use crate::{
    rir::{Arg, EnumVariant, Field, Item, Method, Node},
    ty::{Ty, TyKind},
    DefId,
};

type Nodes = Arc<FxHashMap<DefId, Node>>;

pub fn def_id_equal(nodes: &Nodes, def_id1: DefId, def_id2: DefId) -> bool {
    let node1 = nodes.get(&def_id1).unwrap();
    let node2 = nodes.get(&def_id2).unwrap();
    node_equal(nodes, node1, node2)
}

fn node_equal(nodes: &Nodes, n1: &Node, n2: &Node) -> bool {
    match (&n1.kind, &n2.kind) {
        (crate::rir::NodeKind::Item(item1), crate::rir::NodeKind::Item(item2)) => {
            item_equal(nodes, item1, item2)
        }
        (crate::rir::NodeKind::Variant(v1), crate::rir::NodeKind::Variant(v2)) => {
            variant_equal(nodes, v1, v2)
        }
        (crate::rir::NodeKind::Field(f1), crate::rir::NodeKind::Field(f2)) => {
            field_equal(nodes, f1, f2)
        }
        (crate::rir::NodeKind::Method(m1), crate::rir::NodeKind::Method(m2)) => {
            method_equal(nodes, m1, m2)
        }
        (crate::rir::NodeKind::Arg(a1), crate::rir::NodeKind::Arg(a2)) => arg_equal(nodes, a1, a2),
        _ => false,
    }
}

fn item_equal(nodes: &Nodes, item1: &Item, item2: &Item) -> bool {
    match (item1, item2) {
        (Item::Message(m1), Item::Message(m2)) => {
            m1.name == m2.name
                && vec_equal_by_key(
                    nodes,
                    &m1.fields,
                    &m2.fields,
                    |m| m.id,
                    |cx, f1, f2| field_equal(cx, f1, f2),
                )
        }
        (Item::Enum(e1), Item::Enum(e2)) => {
            e1.name == e2.name
                && vec_equal_by_key(
                    nodes,
                    &e1.variants,
                    &e2.variants,
                    |v| v.id,
                    |cx, v1, v2| variant_equal(cx, v1, v2),
                )
        }
        (Item::Service(s1), Item::Service(s2)) => {
            s1.name == s2.name
                && vec_equal_by_key(
                    nodes,
                    &s1.extend,
                    &s2.extend,
                    |e| e.did,
                    |cx, d1, d2| def_id_equal(cx, d1.did, d2.did),
                )
                && vec_equal_by_key(
                    nodes,
                    &s1.methods,
                    &s2.methods,
                    |m| m.name.0.clone(),
                    |cx, m1, m2| method_equal(cx, m1, m2),
                )
        }
        (Item::NewType(n1), Item::NewType(n2)) => {
            n1.name == n2.name && ty_equal(nodes, &n1.ty, &n2.ty)
        }
        (Item::Const(c1), Item::Const(c2)) => c1.name == c2.name && ty_equal(nodes, &c1.ty, &c2.ty),
        _ => false,
    }
}

fn vec_equal_by_key<T: Clone, F, O: Ord>(
    nodes: &Nodes,
    v1: &[T],
    v2: &[T],
    get_key: impl Fn(&T) -> O,
    f: F,
) -> bool
where
    F: Fn(&Nodes, &T, &T) -> bool,
{
    if v1.len() != v2.len() {
        return false;
    }

    v1.to_owned().sort_by_key(|i| get_key(i));
    v2.to_owned().sort_by_key(|i| get_key(i));

    v1.iter().zip(v2.iter()).all(|(i1, i2)| f(nodes, i1, i2))
}

fn variant_equal(nodes: &Nodes, v1: &EnumVariant, v2: &EnumVariant) -> bool {
    v1.name == v2.name
        && v1.id == v2.id
        && v1.discr == v2.discr
        && v1.fields.len() == v2.fields.len()
        && v1
            .fields
            .iter()
            .zip(&v2.fields)
            .all(|(t1, t2)| ty_equal(nodes, t1, t2))
}

fn field_equal(nodes: &Nodes, f1: &Field, f2: &Field) -> bool {
    f1.id == f2.id && f1.kind == f2.kind && ty_equal(nodes, &f1.ty, &f2.ty)
}

fn method_equal(nodes: &Nodes, m1: &Method, m2: &Method) -> bool {
    m1.name == m2.name
        && m1.args.len() == m2.args.len()
        && m1
            .args
            .iter()
            .zip(m2.args.iter())
            .all(|(a1, a2)| ty_equal(nodes, &a1.ty, &a2.ty))
}

fn arg_equal(nodes: &Nodes, a1: &Arg, a2: &Arg) -> bool {
    ty_equal(nodes, &a1.ty, &a2.ty)
}

fn ty_equal(nodes: &Nodes, ty1: &Ty, ty2: &Ty) -> bool {
    match (&ty1.kind, &ty2.kind) {
        (TyKind::String, TyKind::String) => true,
        (TyKind::FastStr, TyKind::FastStr) => true,
        (TyKind::Void, TyKind::Void) => true,
        (TyKind::U8, TyKind::U8) => true,
        (TyKind::Bool, TyKind::Bool) => true,
        (TyKind::BytesVec, TyKind::BytesVec) => true,
        (TyKind::Bytes, TyKind::Bytes) => true,
        (TyKind::I8, TyKind::I8) => true,
        (TyKind::I16, TyKind::I16) => true,
        (TyKind::I32, TyKind::I32) => true,
        (TyKind::I64, TyKind::I64) => true,
        (TyKind::UInt32, TyKind::UInt32) => true,
        (TyKind::UInt64, TyKind::UInt32) => true,
        (TyKind::F32, TyKind::F32) => true,
        (TyKind::F64, TyKind::F64) => true,
        (TyKind::Map(k1, v1), TyKind::Map(k2, v2)) => {
            ty_equal(nodes, k1, k2) && ty_equal(nodes, v1, v2)
        }
        (TyKind::Vec(t1), TyKind::Vec(t2))
        | (TyKind::Set(t1), TyKind::Set(t2))
        | (TyKind::Arc(t1), TyKind::Arc(t2)) => ty_equal(nodes, t1, t2),
        (TyKind::Path(p1), TyKind::Path(p2)) => def_id_equal(nodes, p1.did, p2.did),
        _ => false,
    }
}

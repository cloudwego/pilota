use pilota_thrift_parser::Function;

use crate::tags::thrift::ThriftStreamingMode;

pub(super) fn mode(function: &Function) -> Result<ThriftStreamingMode, String> {
    let mut annotations = function
        .annotations
        .iter()
        .filter(|annotation| annotation.key == "streaming.mode");
    let Some(annotation) = annotations.next() else {
        return Ok(ThriftStreamingMode::None);
    };
    if annotations.next().is_some() {
        return Err("duplicate `streaming.mode` annotation; exactly one value is allowed".into());
    }
    annotation.value.parse().map_err(|reason| {
        format!(
            "invalid `streaming.mode` value `{}`: {}",
            annotation.value.0, reason
        )
    })
}

pub(super) fn validate(function: &Function) -> Result<(), String> {
    let mode = mode(function)?;
    if mode == ThriftStreamingMode::None {
        return Ok(());
    }
    if function.arguments.len() != 1 {
        return Err("a method with `streaming.mode` must have exactly one argument".into());
    }
    if !mode.is_streaming() {
        return Ok(());
    }
    if !function.throws.is_empty() {
        return Err("streaming `throws` is unsupported; use an application or business terminal status instead".into());
    }
    if function.oneway {
        return Err("streaming methods cannot be `oneway`".into());
    }
    if matches!(function.result_type.0, pilota_thrift_parser::Ty::Void) {
        return Err("streaming methods must return a message struct, not `void`".into());
    }
    Ok(())
}

/// Validate the resolved message kind so typedefs and includes cannot disguise
/// a scalar or container as a streaming message.
pub(crate) fn validate_resolved(db: &crate::db::RootDatabase) -> anyhow::Result<()> {
    use crate::{
        db::RirDatabase,
        rir::{Item, NodeKind},
        ty::{Ty, TyKind},
    };
    fn is_message(
        db: &crate::db::RootDatabase,
        ty: &Ty,
        seen: &mut rustc_hash::FxHashSet<crate::DefId>,
    ) -> bool {
        match &ty.kind {
            TyKind::Arc(inner) => is_message(db, inner, seen),
            TyKind::Path(path) if seen.insert(path.did) => match db.item(path.did).as_deref() {
                Some(Item::Message(_)) => true,
                // Thrift unions are represented as enums with no integer repr.
                Some(Item::Enum(value)) => value.repr.is_none(),
                Some(Item::NewType(value)) => is_message(db, &value.ty, seen),
                _ => false,
            },
            _ => false,
        }
    }
    for node in db.nodes().values() {
        let NodeKind::Method(method) = &node.kind else {
            continue;
        };
        let Some(tags) = db.tags_map().get(&node.tags) else {
            continue;
        };
        if !tags
            .get::<ThriftStreamingMode>()
            .is_some_and(|mode| mode.is_streaming())
        {
            continue;
        }
        let mut seen = rustc_hash::FxHashSet::default();
        let valid_request = method
            .args
            .first()
            .is_some_and(|arg| is_message(db, &arg.ty, &mut seen));
        seen.clear();
        if valid_request && is_message(db, &method.ret, &mut seen) {
            continue;
        }
        let service = node.parent.and_then(|parent| db.item(parent));
        let service_name: &str = match service.as_deref() {
            Some(Item::Service(service)) => service.name.as_ref(),
            _ => "<unknown>",
        };
        let file = &db.file_paths()[&node.file_id];
        let source = std::fs::read_to_string(file.as_ref()).unwrap_or_default();
        let location =
            pilota_thrift_parser::FileParser::new(pilota_thrift_parser::FileSource::new(&source))
                .function_locations()
                .into_iter()
                .find(|location| {
                    location.service == service_name
                        && location.function.as_str() == method.name.raw_str().as_str()
                });
        let position = location.map_or_else(String::new, |location| {
            let prefix = &source[..location.span.start];
            format!(
                ":{}:{}",
                prefix.bytes().filter(|byte| *byte == b'\n').count() + 1,
                prefix
                    .rsplit('\n')
                    .next()
                    .unwrap_or_default()
                    .chars()
                    .count()
                    + 1
            )
        });
        anyhow::bail!(
            "{}{}: service `{}`, method `{}`: streaming request and response must be message structs (including typedefs/unions), not scalar or container values",
            file.display(),
            position,
            service_name,
            method.name
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use super::*;

    fn function(signature: &str) -> Function {
        Function::get_parser().parse(signature).unwrap()
    }

    #[test]
    fn annotation_modes_are_independent_of_transport() {
        assert_eq!(
            mode(&function("Resp f(1:Req req)")),
            Ok(ThriftStreamingMode::None)
        );
        for (value, expected) in [
            ("unary", ThriftStreamingMode::Unary),
            ("client", ThriftStreamingMode::Client),
            ("server", ThriftStreamingMode::Server),
            ("bidirectional", ThriftStreamingMode::Bidirectional),
        ] {
            let f = function(&format!("Resp f(1:Req req) (streaming.mode=\"{value}\")"));
            assert_eq!(mode(&f), Ok(expected));
            assert_eq!(validate(&f), Ok(()));
        }
    }

    #[test]
    fn rejects_ambiguous_and_invalid_annotations() {
        for value in ["", "bidi", "Client", "unknown"] {
            let f = function(&format!("Resp f(1:Req req) (streaming.mode=\"{value}\")"));
            assert!(
                validate(&f)
                    .unwrap_err()
                    .contains("invalid `streaming.mode`")
            );
        }
        for pair in [
            "streaming.mode=\"client\",streaming.mode=\"client\"",
            "streaming.mode=\"client\",streaming.mode=\"server\"",
        ] {
            let f = function(&format!("Resp f(1:Req req) ({pair})"));
            assert!(validate(&f).unwrap_err().contains("duplicate"));
        }
    }

    #[test]
    fn streaming_throws_and_unsupported_signatures_are_diagnostics() {
        for signature in [
            "Resp f(1:Req req) throws (1:Ex ex) (streaming.mode=\"client\")",
            "Resp f() (streaming.mode=\"server\")",
            "Resp f(1:Req req,2:Req other) (streaming.mode=\"unary\")",
            "oneway void f(1:Req req) (streaming.mode=\"client\")",
            "void f(1:Req req) (streaming.mode=\"bidirectional\")",
        ] {
            assert!(validate(&function(signature)).is_err(), "{signature}");
        }
        for signature in [
            "Resp f() throws (1:Ex ex)",
            "Resp f(1:Req req) throws (1:Ex ex) (streaming.mode=\"unary\")",
        ] {
            assert_eq!(validate(&function(signature)), Ok(()));
        }
    }
}

namespace rs typing
struct SOME { 1: optional SOME nested }
struct OPTION { 1: required SOME value }
struct NONE { 1: optional binary body }
typedef SOME RequestAlias
service Mixed {
    OPTION Plain(1: RequestAlias request)
    OPTION Watch(1: RequestAlias request) (streaming.mode="server", pilota.name="watch_events")
    NONE Chat(1: SOME request) (streaming.mode="bidirectional")
} (pilota.rust_wrapper_arc="true")

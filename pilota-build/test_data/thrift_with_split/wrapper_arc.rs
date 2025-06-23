pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};

    pub mod wrapper_arc {
        use ::pilota::{Buf as _, BufMut as _};
        include!("wrapper_arc/mod.rs");
    }
}

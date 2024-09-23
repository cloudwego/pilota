pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]

    pub mod wrapper_arc {

        include!("wrapper_arc/A.rs");

        include!("wrapper_arc/TestService.rs");

        include!("wrapper_arc/TestServiceTestResultRecv.rs");

        include!("wrapper_arc/TestServiceTestArgsRecv.rs");

        include!("wrapper_arc/TestServiceTestResultSend.rs");

        include!("wrapper_arc/TEST.rs");

        include!("wrapper_arc/TestServiceTestArgsSend.rs");
    }
}

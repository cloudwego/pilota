pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]

    pub mod wrapper_arc {
        include!("wrapper_arc/message_A.rs");
        include!("wrapper_arc/service_TestService.rs");
        include!("wrapper_arc/enum_TestServiceTestResultRecv.rs");
        include!("wrapper_arc/message_TestServiceTestArgsRecv.rs");
        include!("wrapper_arc/enum_TestServiceTestResultSend.rs");
        include!("wrapper_arc/message_TEST.rs");
        include!("wrapper_arc/message_TestServiceTestArgsSend.rs");
    }
}

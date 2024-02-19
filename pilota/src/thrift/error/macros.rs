#[doc(hidden)]
#[macro_export]
macro_rules! msg_impl {
    () => {
        /// Append a message to the existing error message.
        ///
        /// That means, the new message will be: `old_message` + `message`.
        pub fn append_msg(&mut self, message: &str) {
            let mut s = String::with_capacity(self.message.len() + message.len());
            s.push_str(self.message.as_str());
            s.push_str(message);
            self.message = s.into();
        }

        /// Prepend a message to the existing error message.
        ///
        /// That means, the new message will be: `message` + `old_message`.
        pub fn prepend_msg(&mut self, message: &str) {
            let mut s = String::with_capacity(self.message.len() + message.len());
            s.push_str(message);
            s.push_str(self.message.as_str());
            self.message = s.into();
        }
    };
}

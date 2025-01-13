#![deny(missing_docs)]

use std::fmt::{self, Debug, Display};
use std::string::String;

/// Represents the Component Model `error-context` type.
pub struct ErrorContext {
    handle: u32,
}

impl ErrorContext {
    #[doc(hidden)]
    pub fn from_handle(handle: u32) -> Self {
        Self { handle }
    }

    #[doc(hidden)]
    pub fn handle(&self) -> u32 {
        self.handle
    }
}

impl Debug for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorContext").finish()
    }
}

impl Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for ErrorContext {}

impl Drop for ErrorContext {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            unreachable!();
        }

        #[cfg(target_arch = "wasm32")]
        {
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[error-context-drop]"]
                fn error_drop(_: u32);
            }
            if self.handle != 0 {
                unsafe { error_drop(self.handle) }
            }
        }
    }
}

/// Call the `error-context.new` canonical built-in function.
pub fn error_context_new(debug_message: &str) -> ErrorContext {
    #[cfg(not(target_arch = "wasm32"))]
    {
        _ = debug_message;
        unreachable!();
    }

    #[cfg(target_arch = "wasm32")]
    {
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "[error-context-new]"]
            fn error_context_new(_: i32, _: i32) -> i32;
        }

        let len = i32::try_from(debug_message.len()).expect("invalid debug message length");
        unsafe {
            let err_ctx_handle = error_context_new(debug_message.as_ptr(), len);
            ErrorContext::from_handle(u32::try_from(err_ctx_handle))
        }
    }
}

#![deny(missing_docs)]

use std::fmt::{self, Debug, Display};

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

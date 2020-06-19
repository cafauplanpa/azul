    #![allow(dead_code, unused_imports)]
    //! Definition of azuls internal `Option<*>` wrappers
    use crate::dll::*;
    use std::ffi::c_void;


    /// `ResultRefAnyBlockError` struct
    pub use crate::dll::AzResultRefAnyBlockError as ResultRefAnyBlockError;

    impl std::fmt::Debug for ResultRefAnyBlockError { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_result_ref_any_block_error_fmt_debug)(self)) } }
    impl Clone for ResultRefAnyBlockError { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_result_ref_any_block_error_deep_copy)(self) } }
    impl Drop for ResultRefAnyBlockError { fn drop(&mut self) { (crate::dll::get_azul_dll().az_result_ref_any_block_error_delete)(self); } }

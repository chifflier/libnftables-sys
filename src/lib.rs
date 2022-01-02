extern crate json;

mod bindings;

use json::JsonValue;
use std::ffi::CStr;
use std::os::raw::c_char;

pub use bindings::*;

pub struct Nftables {
    ctx: *mut nft_ctx,
}

impl Nftables {
    pub fn new() -> Self {
        unsafe {
            let ctx = nft_ctx_new(0);
            nft_ctx_output_set_flags(ctx, NFT_CTX_DEFAULT);
            nft_ctx_buffer_output(ctx);
            nft_ctx_buffer_error(ctx);
            Self { ctx }
        }
    }

    pub fn run_cmd(&mut self, cmd: *const c_char) -> (i32, *const i8, *const i8) {
        assert!(!self.ctx.is_null());

        unsafe {
            (
                nft_run_cmd_from_buffer(self.ctx, cmd),
                nft_ctx_get_output_buffer(self.ctx),
                nft_ctx_get_error_buffer(self.ctx),
            )
        }
    }

    pub fn json_cmd(&mut self, cmd: *const c_char) -> (i32, JsonValue, *const i8) {
        assert!(!self.ctx.is_null());

        let (rc, output, error) = unsafe {
            let flags = nft_ctx_output_get_flags(self.ctx);
            nft_ctx_output_set_flags(self.ctx, flags | NFT_CTX_OUTPUT_JSON);
            let rc = nft_run_cmd_from_buffer(self.ctx, cmd);
            let output = nft_ctx_get_output_buffer(self.ctx);
            let error = nft_ctx_get_error_buffer(self.ctx);
            nft_ctx_output_set_flags(self.ctx, flags);
            (rc, output, error)
        };

        if output.is_null() {
            eprintln!("OUTPUT IS EMPTY");
            return (rc, JsonValue::from(""), error);
        }

        let s = unsafe { CStr::from_ptr(output) };
        let output = match s.to_str() {
            Ok(s) => JsonValue::from(s),
            Err(e) => panic!("Error converting output to UTF-8: {:?}", e),
        };

        (rc, output, error)
    }

    pub fn set_debug(&mut self, flags: nft_debug_level) {
        unsafe { nft_ctx_output_set_debug(self.ctx, flags) };
    }

    pub fn get_debug(&self) -> nft_debug_level {
        unsafe { nft_ctx_output_get_debug(self.ctx) }
    }
}

impl Drop for Nftables {
    fn drop(&mut self) {
        unsafe { nft_ctx_free(self.ctx) };
        self.ctx = ::std::ptr::null_mut();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    extern "C" {
        fn getuid() -> u32;
    }

    #[test]
    fn list_ruleset() {
        assert_eq!(unsafe { getuid() }, 0);

        let mut nft = Nftables::new();

        assert!(nft.ctx != ::std::ptr::null_mut());

        let cmd = CStr::from_bytes_with_nul(b"list ruleset\0").unwrap();
        let (rc, output, error) = nft.run_cmd(cmd.as_ptr());

        assert_eq!(rc, 0);
        assert_ne!(output, ::std::ptr::null());
        assert_ne!(error, ::std::ptr::null());
    }

    #[test]
    fn set_debug() {
        // assert_eq!(unsafe{ getuid() }, 0);

        let mut nft = Nftables::new();

        assert!(nft.ctx != ::std::ptr::null_mut());

        let lvl = nft.get_debug();
        assert_eq!(lvl, 0);

        nft.set_debug(NFT_DEBUG_SCANNER | NFT_DEBUG_EVALUATION | NFT_DEBUG_NETLINK);

        let lvl = nft.get_debug();
        assert_eq!(lvl, 0xd);
    }
}

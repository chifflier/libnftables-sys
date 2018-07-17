extern crate libnftables_sys;

use libnftables_sys::*;

use std::ffi::CStr;
use std::process::exit;

extern "C" {
    fn getuid() -> u32;
}

fn main() {
    if unsafe{ getuid() } != 0 {
        eprintln!("Sorry, you have to be uid==0");
        exit(1);
    }

    let mut nft = Nftables::new();

    let cmd = CStr::from_bytes_with_nul(b"list ruleset\0").unwrap();
    let (rc,output,error) = nft.json_cmd(cmd.as_ptr());

    println!("{}", output);

    if error != ::std::ptr::null() {
        let s = unsafe{ CStr::from_ptr(error) };
        eprintln!("{}", s.to_string_lossy());
    }

    exit(rc);
}


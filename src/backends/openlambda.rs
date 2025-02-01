use crate::Error;
use core::mem::MaybeUninit;

pub use crate::util::{inner_u32, inner_u64};

mod api {
    #[link(wasm_import_module = "ol_args")]
    extern "C" {
        pub fn get_random_value(buf_ptr: *mut u8, buf_len: u32);
    }
}

pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    let ptr = dest.as_mut_ptr().cast();
    let len = dest.len() as u32;

    unsafe {
        api::get_random_value(ptr, len);
    }

    Ok(())
}

use crate::Error;

mod api {
    #[link(wasm_import_module = "ol_args")]
    extern "C" {
        pub fn get_random_value(buf_ptr: *mut u8, buf_len: u32);
    }
}

pub(crate) fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    let ptr = dest.as_mut_ptr();
    let len = dest.len() as u32;

    unsafe {
        api::get_random_value(ptr, len);
    }

    Ok(())
}

use std::ffi::CString;
use std::os::raw::c_char;

use espflash::Flasher;

#[no_mangle]
pub extern "C" fn write_flash(fd: usize, offset: u64, bin_file: *const c_char) -> *mut c_char {
    let serial_port = serialport::new("", 9200).open().unwrap();
    // let mut flasher = Flasher::connect(serial_port, serial_port, None);
    CString::new("Hello world".to_owned()).unwrap().into_raw()
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;
    use std::ffi::CString;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_github_douile_espflash_ESPFlash_write_flash(
        env: JNIEnv,
        _: JClass,
    ) -> jstring {
        let world_ptr = CString::new("Hello world from Rust world").unwrap();
        let output = env
            .new_string(world_ptr.to_str().unwrap())
            .expect("Couldn't create java string!");
        output.into_inner()
    }
}

use htrk;

#[no_mangle]
pub extern "C" fn c_lib_fun() {
    htrk::library_function();
}

/* automatically generated by rust-bindgen */

pub type va_list = __builtin_va_list;
pub type size_t = ::std::os::raw::c_ulong;
pub type __int64_t = ::std::os::raw::c_long;
pub use self::_LogLevel as LogLevel;
pub const _LogLevel_LOGLEVEL_UNSET: _LogLevel = 0;
pub const _LogLevel_LOGLEVEL_ERROR: _LogLevel = 1;
pub const _LogLevel_LOGLEVEL_CRITICAL: _LogLevel = 2;
pub const _LogLevel_LOGLEVEL_WARNING: _LogLevel = 3;
pub const _LogLevel_LOGLEVEL_INFO: _LogLevel = 4;
pub const _LogLevel_LOGLEVEL_DEBUG: _LogLevel = 5;
pub const _LogLevel_LOGLEVEL_TRACE: _LogLevel = 6;
pub type _LogLevel = i32;
pub type Logger = _Logger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Logger {
    pub log: ::std::option::Option<
        unsafe extern "C" fn(
            logger: *mut Logger,
            level: LogLevel,
            fileName: *const ::std::os::raw::c_char,
            functionName: *const ::std::os::raw::c_char,
            lineNumber: ::std::os::raw::c_int,
            format: *const ::std::os::raw::c_char,
            vargs: *mut __va_list_tag,
        ),
    >,
    pub flush: ::std::option::Option<unsafe extern "C" fn(logger: *mut Logger)>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(logger: *mut Logger)>,
}
#[test]
fn bindgen_test_layout__Logger() {
    assert_eq!(
        ::std::mem::size_of::<_Logger>(),
        24usize,
        concat!("Size of: ", stringify!(_Logger))
    );
    assert_eq!(
        ::std::mem::align_of::<_Logger>(),
        8usize,
        concat!("Alignment of ", stringify!(_Logger))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Logger>())).log as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Logger),
            "::",
            stringify!(log)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Logger>())).flush as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_Logger),
            "::",
            stringify!(flush)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Logger>())).destroy as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_Logger),
            "::",
            stringify!(destroy)
        )
    );
}
extern "C" {
    pub fn logger_setDefault(logger: *mut Logger);
}
extern "C" {
    pub fn logger_getDefault() -> *mut Logger;
}
extern "C" {
    pub fn logger_log(
        logger: *mut Logger,
        level: LogLevel,
        fileName: *const ::std::os::raw::c_char,
        functionName: *const ::std::os::raw::c_char,
        lineNumber: ::std::os::raw::c_int,
        format: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn logger_get_global_start_time_micros() -> i64;
}
extern "C" {
    pub fn logger_now_micros() -> i64;
}
extern "C" {
    pub fn logger_elapsed_micros() -> i64;
}
extern "C" {
    pub fn logger_elapsed_string(dst: *mut ::std::os::raw::c_char, size: size_t) -> size_t;
}
extern "C" {
    pub fn logger_set_global_start_time_micros(arg1: i64);
}
extern "C" {
    pub fn logger_base_name(
        filename: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn logger_flush(logger: *mut Logger);
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}

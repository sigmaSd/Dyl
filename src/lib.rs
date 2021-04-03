#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::CString;
use std::mem::transmute;
use std::mem::ManuallyDrop;
use std::os::raw::c_void;

// definitions taken from rust libc crate
type c_char = i8;
type c_int = i32;
type CPtr = *const u8;
//type c_long = i64;
type ssize_t = isize;
type size_t = usize;

extern "C" {
    fn dlsym(handle: CPtr, symbol: CPtr) -> CPtr;
}

//pub unsafe extern "C" fn open(path: *const c_char, oflag: c_int, ...) -> c_int
#[no_mangle]
pub fn open(path: *const c_char, oflag: c_int) -> c_int {
    unsafe {
        //define RTLD_NEXT	((void *) -1l)
        let RTLD_NEXT: CPtr = -1i64 as CPtr; //c_long;
        let orig_open = dlsym(RTLD_NEXT, "open\0".as_ptr());
        let orig_open: extern "C" fn(*const c_char, c_int) -> c_int = transmute(orig_open);

        let path_name = ManuallyDrop::new(CString::from_raw(path as *mut _));
        dbg!(&path_name, &oflag);

        orig_open(path, oflag)
    }
}

//pub unsafe extern "C" fn open64(path: *const c_char, oflag: c_int, ...) -> c_int
#[no_mangle]
pub fn open64(path: *const c_char, oflag: c_int) -> c_int {
    unsafe {
        //define RTLD_NEXT	((void *) -1l)
        let RTLD_NEXT: CPtr = -1i64 as CPtr; //c_long;
        let orig_open64 = dlsym(RTLD_NEXT, "open64\0".as_ptr());
        let orig_open64: extern "C" fn(*const c_char, c_int) -> c_int = transmute(orig_open64);

        let path_name = ManuallyDrop::new(CString::from_raw(path as *mut _));
        dbg!(&path_name, &oflag);

        orig_open64(path, oflag)
    }
}

/*pub unsafe extern "C" fn read(
    fd: c_int,
    buf: *mut c_void,
    count: size_t
) -> ssize_t*/
#[no_mangle]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    let RTLD_NEXT: CPtr = -1i64 as CPtr; //c_long;
    let read = dlsym(RTLD_NEXT, "read\0".as_ptr());
    let read: extern "C" fn(c_int, *mut c_void, size_t) -> ssize_t = transmute(read);

    let res = read(fd, buf, count);
    dbg!(ManuallyDrop::new(CString::from_raw(buf as *mut i8)));
    res
}

/*pub unsafe extern "C" fn write(
    fd: c_int,
    buf: *const c_void,
    count: size_t
) -> ssize_t */
#[no_mangle]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    let RTLD_NEXT: CPtr = -1i64 as CPtr; //c_long;
    let write = dlsym(RTLD_NEXT, "write\0".as_ptr());
    let write: extern "C" fn(c_int, *const c_void, size_t) -> ssize_t = transmute(write);

    let res = write(fd, buf, count);
    //to print buf uncomment next lines
    //let bufread = ManuallyDrop::new(CStr::from_ptr(buf as _).to_string_lossy().to_string());
    //write(
    //    1,
    //    CString::new(bufread.as_bytes()).unwrap().into_raw() as _,
    //    1,
    //);
    res
}

/*pub unsafe extern "C" fn write64(
    fd: c_int,
    buf: *const c_void,
    count: size_t
) -> ssize_t */
#[no_mangle]
pub unsafe extern "C" fn write64(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    let RTLD_NEXT: CPtr = -1i64 as CPtr; //c_long;
    let write64 = dlsym(RTLD_NEXT, "write64\0".as_ptr());
    let write64: extern "C" fn(c_int, *const c_void, size_t) -> ssize_t = transmute(write64);

    let res = write64(fd, buf, count);
    //to print buf uncomment next lines
    //let bufread = ManuallyDrop::new(CStr::from_ptr(buf as _).to_string_lossy().to_string());
    //write64(
    //    1,
    //    CString::new(bufread.as_bytes()).unwrap().into_raw() as _,
    //    1,
    //);
    res
}

/*pub unsafe extern "C" fn recv(
    socket: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int
) -> ssize_t */

#[no_mangle]
pub unsafe extern "C" fn recv(
    socket: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int,
) -> ssize_t {
    let RTLD_NEXT: CPtr = -1i64 as CPtr; //c_long;
    let recv = dlsym(RTLD_NEXT, "recv\0".as_ptr());
    let recv: extern "C" fn(c_int, *mut c_void, size_t, c_int) -> ssize_t = transmute(recv);
    dbg!(ManuallyDrop::new(CString::from_raw(buf as _)));

    // limit bandwidth
    //std::thread::sleep_ms(20);
    recv(socket, buf, len, flags)
}

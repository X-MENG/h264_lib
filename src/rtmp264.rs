use std::ffi::CString;
use std::os::raw;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Error {
    LibError(i32),
}

extern {
    fn RTMP264_Connect(url: *const raw::c_char) -> raw::c_int;
    fn RTMP264_Send(read_buffer: unsafe extern "C" fn (buf: *const raw::c_char, buf_size: raw::c_int) -> raw::c_int) -> raw::c_int;
    fn RTMP264_Close();
}

pub fn connect(url: &str) -> std::result::Result<(), Error> {
    let url = CString::new(url).unwrap();

    let rv = unsafe { RTMP264_Connect(url.as_ptr()) };
    if rv != 0 {
        return Err(Error::LibError(rv));
    }
    
    Ok(())
}

#[no_mangle]
extern "C" fn read_buffer(buf: *const raw::c_char, buf_size: raw::c_int) -> raw::c_int {
    println!("Hello from Rust!");
    0
}

pub fn send<S: Read>(stream: &mut S) -> std::result::Result<(), Error> {
    let rv = unsafe { RTMP264_Send(read_buffer) };
    if rv != 0 {
        return Err(Error::LibError(rv));
    }
    
    Ok(())
}

pub fn close() {
    unsafe { RTMP264_Close(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test4() {
        connect("baidu.com").unwrap();

        let h264_in = include_bytes!("../data/test-25fps.h264");
        let mut buff = Cursor::new(h264_in);

        send(&mut buff).unwrap();
    }
}

use std::ffi::CString;
use std::os::raw;

#[derive(Debug)]
pub struct Error {
    code: i32,
}

extern {
    fn RTMP264_Connect(url: *const raw::c_char) -> raw::c_int;
    fn RTMP264_Send(read_buffer: fn (buf: *const raw::c_char, buf_size: raw::c_int) -> raw::c_int) -> raw::c_int;
    fn RTMP264_Close();
}

pub fn connect(url: &str) -> std::result::Result<(), Error> {
    let url = CString::new(url).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test4() {
        connect("baidu.com").unwrap();
    }
}

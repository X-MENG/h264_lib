use std::cell::RefCell;
use std::ffi::CString;
use std::io::Read;
use std::os::raw;

#[derive(Debug)]
pub enum Error {
    LibError(i32),
}

extern "C" {
    fn InitSockets() -> raw::c_int;
    fn ClearSockets();
    fn RTMP264_Connect(url: *const raw::c_char) -> raw::c_int;
    fn RTMP264_Send(
        read_buffer: unsafe extern "C" fn(
            buf: *const raw::c_char,
            buf_size: raw::c_int,
        ) -> raw::c_int,
    ) -> raw::c_int;
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

thread_local! {
    static S: RefCell<Option<*mut dyn Read>> = RefCell::new(None);
}

#[no_mangle]
extern "C" fn read_buffer(buf: *const raw::c_char, buf_size: raw::c_int) -> raw::c_int {
    S.with(|f| {
        let stream = (*f.borrow()).unwrap();

        // stream.read();

        12
    })
}

pub fn send<S: Read + 'static>(stream: &mut S) -> std::result::Result<(), Error> {
    let stream = stream as *mut dyn Read;

    S.with(|f| {
        assert!((*f.borrow()).is_none());
        (*f.borrow_mut()) = Some(stream);
    });

    let rv = unsafe { RTMP264_Send(read_buffer) };

    S.with(|f| {
        assert!((*f.borrow()).is_some());
        (*f.borrow_mut()) = None;
    });

    if rv != 0 {
        return Err(Error::LibError(rv));
    }

    Ok(())
}

pub fn close() {
    unsafe {
        RTMP264_Close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test4() {
        unsafe {
            InitSockets();
        }

        connect("baidu.com").unwrap();

        let h264_in = include_bytes!("../data/test-25fps.h264");
        let mut buff = Cursor::new(h264_in);

        send(&mut buff).unwrap();

        close();

        unsafe {
            ClearSockets();
        }
    }
}

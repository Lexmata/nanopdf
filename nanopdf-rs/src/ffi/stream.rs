//! C FFI for stream - MuPDF compatible
//! Safe Rust implementation using handle-based resource management

use super::{Handle, STREAMS, BUFFERS};
use std::ffi::c_char;

/// Internal stream state
pub struct Stream {
    pub(crate) data: Vec<u8>,
    position: usize,
    eof: bool,
}

impl Stream {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            position: 0,
            eof: true,
        }
    }

    pub fn from_memory(data: Vec<u8>) -> Self {
        let eof = data.is_empty();
        Self {
            data,
            position: 0,
            eof,
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        if self.position >= self.data.len() {
            self.eof = true;
            return 0;
        }

        let available = self.data.len() - self.position;
        let to_read = buf.len().min(available);
        buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
        self.position += to_read;

        if self.position >= self.data.len() {
            self.eof = true;
        }

        to_read
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.position >= self.data.len() {
            self.eof = true;
            return None;
        }
        let byte = self.data[self.position];
        self.position += 1;
        if self.position >= self.data.len() {
            self.eof = true;
        }
        Some(byte)
    }

    pub fn peek_byte(&self) -> Option<u8> {
        if self.position >= self.data.len() {
            return None;
        }
        Some(self.data[self.position])
    }

    pub fn seek(&mut self, offset: i64, whence: i32) {
        let new_pos = match whence {
            0 => offset as usize, // SEEK_SET
            1 => (self.position as i64 + offset) as usize, // SEEK_CUR
            2 => (self.data.len() as i64 + offset) as usize, // SEEK_END
            _ => self.position,
        };
        self.position = new_pos.min(self.data.len());
        self.eof = self.position >= self.data.len();
    }

    pub fn tell(&self) -> i64 {
        self.position as i64
    }

    pub fn is_eof(&self) -> bool {
        self.eof
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}

/// Keep (increment ref) a stream
#[unsafe(no_mangle)]
pub extern "C" fn fz_keep_stream(_ctx: Handle, stm: Handle) -> Handle {
    STREAMS.keep(stm)
}

/// Drop a stream reference
#[unsafe(no_mangle)]
pub extern "C" fn fz_drop_stream(_ctx: Handle, stm: Handle) {
    let _ = STREAMS.remove(stm);
}

/// Open a file for reading
///
/// # Safety
/// Caller must ensure `filename` is a valid null-terminated C string.
#[unsafe(no_mangle)]
pub extern "C" fn fz_open_file(_ctx: Handle, filename: *const c_char) -> Handle {
    if filename.is_null() {
        return 0;
    }

    // SAFETY: Caller guarantees filename is a valid null-terminated C string
    #[allow(unsafe_code)]
    let c_str = unsafe { std::ffi::CStr::from_ptr(filename) };
    let path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    match std::fs::read(path) {
        Ok(data) => STREAMS.insert(Stream::from_memory(data)),
        Err(_) => 0,
    }
}

/// Open a stream from memory
///
/// # Safety
/// Caller must ensure `data` points to valid memory of at least `len` bytes.
#[unsafe(no_mangle)]
pub extern "C" fn fz_open_memory(
    _ctx: Handle,
    data: *const u8,
    len: usize,
) -> Handle {
    if data.is_null() || len == 0 {
        return STREAMS.insert(Stream::new());
    }

    // SAFETY: Caller guarantees data points to valid memory of `len` bytes
    #[allow(unsafe_code)]
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    STREAMS.insert(Stream::from_memory(slice.to_vec()))
}

/// Open a stream from a buffer handle
#[unsafe(no_mangle)]
pub extern "C" fn fz_open_buffer(_ctx: Handle, buf: Handle) -> Handle {
    if let Some(buffer) = BUFFERS.get(buf) {
        if let Ok(guard) = buffer.lock() {
            return STREAMS.insert(Stream::from_memory(guard.data().to_vec()));
        }
    }
    0
}

/// Read from stream into buffer
///
/// # Safety
/// Caller must ensure `data` points to writable memory of at least `len` bytes.
#[unsafe(no_mangle)]
pub extern "C" fn fz_read(
    _ctx: Handle,
    stm: Handle,
    data: *mut u8,
    len: usize,
) -> usize {
    if data.is_null() || len == 0 {
        return 0;
    }

    if let Some(stream) = STREAMS.get(stm) {
        if let Ok(mut guard) = stream.lock() {
            // SAFETY: Caller guarantees data points to writable memory of `len` bytes
            #[allow(unsafe_code)]
            let buf = unsafe { std::slice::from_raw_parts_mut(data, len) };
            return guard.read(buf);
        }
    }
    0
}

/// Read a single byte from stream
#[unsafe(no_mangle)]
pub extern "C" fn fz_read_byte(_ctx: Handle, stm: Handle) -> i32 {
    if let Some(stream) = STREAMS.get(stm) {
        if let Ok(mut guard) = stream.lock() {
            if let Some(byte) = guard.read_byte() {
                return byte as i32;
            }
        }
    }
    -1 // EOF
}

/// Peek at next byte without consuming
#[unsafe(no_mangle)]
pub extern "C" fn fz_peek_byte(_ctx: Handle, stm: Handle) -> i32 {
    if let Some(stream) = STREAMS.get(stm) {
        if let Ok(guard) = stream.lock() {
            if let Some(byte) = guard.peek_byte() {
                return byte as i32;
            }
        }
    }
    -1
}

/// Check if stream is at EOF
#[unsafe(no_mangle)]
pub extern "C" fn fz_is_eof(_ctx: Handle, stm: Handle) -> i32 {
    if let Some(stream) = STREAMS.get(stm) {
        if let Ok(guard) = stream.lock() {
            return i32::from(guard.is_eof());
        }
    }
    1
}

/// Seek in stream
#[unsafe(no_mangle)]
pub extern "C" fn fz_seek(_ctx: Handle, stm: Handle, offset: i64, whence: i32) {
    if let Some(stream) = STREAMS.get(stm) {
        if let Ok(mut guard) = stream.lock() {
            guard.seek(offset, whence);
        }
    }
}

/// Get current position in stream
#[unsafe(no_mangle)]
pub extern "C" fn fz_tell(_ctx: Handle, stm: Handle) -> i64 {
    if let Some(stream) = STREAMS.get(stm) {
        if let Ok(guard) = stream.lock() {
            return guard.tell();
        }
    }
    0
}

// Integer reading functions
#[unsafe(no_mangle)]
pub extern "C" fn fz_read_uint16(_ctx: Handle, stm: Handle) -> u16 {
    let mut buf = [0u8; 2];
    if fz_read(_ctx, stm, buf.as_mut_ptr(), 2) == 2 {
        u16::from_be_bytes(buf)
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fz_read_uint32(_ctx: Handle, stm: Handle) -> u32 {
    let mut buf = [0u8; 4];
    if fz_read(_ctx, stm, buf.as_mut_ptr(), 4) == 4 {
        u32::from_be_bytes(buf)
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fz_read_uint16_le(_ctx: Handle, stm: Handle) -> u16 {
    let mut buf = [0u8; 2];
    if fz_read(_ctx, stm, buf.as_mut_ptr(), 2) == 2 {
        u16::from_le_bytes(buf)
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fz_read_uint32_le(_ctx: Handle, stm: Handle) -> u32 {
    let mut buf = [0u8; 4];
    if fz_read(_ctx, stm, buf.as_mut_ptr(), 4) == 4 {
        u32::from_le_bytes(buf)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_from_memory() {
        let data = vec![1, 2, 3, 4, 5];
        let handle = STREAMS.insert(Stream::from_memory(data));

        assert_eq!(fz_read_byte(0, handle), 1);
        assert_eq!(fz_read_byte(0, handle), 2);
        assert_eq!(fz_tell(0, handle), 2);

        fz_seek(0, handle, 0, 0); // SEEK_SET
        assert_eq!(fz_read_byte(0, handle), 1);

        fz_drop_stream(0, handle);
    }

    #[test]
    fn test_stream_eof() {
        let data = vec![1];
        let handle = STREAMS.insert(Stream::from_memory(data));

        assert_eq!(fz_is_eof(0, handle), 0);
        fz_read_byte(0, handle);
        assert_eq!(fz_is_eof(0, handle), 1);

        fz_drop_stream(0, handle);
    }
}

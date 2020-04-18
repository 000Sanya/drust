use std::mem;
use std::ptr::drop_in_place;
use std::mem::ManuallyDrop;
use std::ffi::{CStr, CString};
use std::slice::from_raw_parts;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RawSlice<T> {
	ptr: *const T,
	len: usize,
}

impl<T> RawSlice<T> {
	pub fn from_slice(slice: &[T]) -> Self {
		Self {
			ptr: slice.as_ptr(),
			len: slice.len()
		}
	}

	pub const fn null() -> Self {
		Self {
			ptr: std::ptr::null(),
			len: 0
		}
	}
}

#[no_mangle]
pub extern fn test_one(a: i32, b: i32) -> i32 {
	a + b
}

#[no_mangle]
pub extern fn test_create() -> *mut Test {
	Box::into_raw(Box::new(Test::new()))
}

#[no_mangle]
pub unsafe extern fn test_destroy(ptr: *mut Test) {
	let _ = Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern fn test_set_message(this: *mut Test, message: RawSlice<u8>) {
	let test = unsafe { this.as_mut() };
	let s = from_raw_parts(message.ptr, message.len);
	let s = std::str::from_utf8(s);
	if let Ok(s) = s {
		test.map(|t| t.set_message(s));
	}
}

#[no_mangle]
pub extern fn test_get_message(this: *mut Test) -> RawSlice<u8> {
	unsafe {
		this.as_ref().map(|test| {
			let s = test.get_message();
			RawSlice::from_slice(s.as_bytes())
		}).unwrap_or(RawSlice::null())
	}
}

#[no_mangle]
pub extern fn try_bool() -> bool {
	true
}

pub struct Test {
	message: String,
}

impl Test {
	pub fn new() -> Self {
		Self {
			message: "Hello from Rust".to_owned(),
		}
	}

	pub fn get_message(&self) -> &str {
		&self.message
	}

	pub fn set_message(&mut self, message: &str) {
		self.message.clear();
		self.message.push_str(message);
		println!("{}", self.message);
	}
}
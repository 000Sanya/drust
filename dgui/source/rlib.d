module rlib;

import ffi = rlib_binding;

class Test {
    private ffi.Test* inner;

    this() {
        inner = ffi.test_create();
    }

    ~this() {
        ffi.test_destroy(inner);
    }

    string get_message() {
        auto slice = ffi.test_get_message(inner);
        return cast(string)(slice.ptr ? slice.ptr[0..slice.len] : null);
    }

    void set_message(string message) {
        auto slice = ffi.RawSlice_u8(cast(ubyte*)message.ptr, message.length);
        ffi.test_set_message(inner, slice);
    }
}
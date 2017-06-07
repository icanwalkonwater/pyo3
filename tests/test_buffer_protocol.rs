#![allow(dead_code, unused_variables)]
#![feature(proc_macro, specialization)]

extern crate pyo3;

use std::ptr;
use std::os::raw::{c_int, c_void};

use pyo3::*;


#[py::class]
struct TestClass {
    vec: Vec<u8>,
    token: PyToken,
}
#[py::ptr(TestClass)]
struct TestClassPtr(PyPtr);

#[py::proto]
impl class::PyBufferProtocol for TestClass {

    fn bf_getbuffer(&self, py: Python, view: *mut ffi::Py_buffer, flags: c_int) -> PyResult<()> {
        if view == ptr::null_mut() {
            return Err(PyErr::new::<exc::BufferError, _>(py, "View is null"))
        }

        unsafe {
            (*view).obj = ptr::null_mut();
        }

        if (flags & ffi::PyBUF_WRITABLE) == ffi::PyBUF_WRITABLE {
            return Err(PyErr::new::<exc::BufferError, _>(py, "Object is not writable"))
        }

        let bytes = &self.vec;

        unsafe {
            (*view).buf = bytes.as_ptr() as *mut c_void;
            (*view).len = bytes.len() as isize;
            (*view).readonly = 1;
            (*view).itemsize = 1;

            (*view).format = ptr::null_mut();
            if (flags & ffi::PyBUF_FORMAT) == ffi::PyBUF_FORMAT {
                let msg = ::std::ffi::CStr::from_ptr("B\0".as_ptr() as *const _);
                (*view).format = msg.as_ptr() as *mut _;
            }

            (*view).ndim = 1;
            (*view).shape = ptr::null_mut();
            if (flags & ffi::PyBUF_ND) == ffi::PyBUF_ND {
                (*view).shape = (&((*view).len)) as *const _ as *mut _;
            }

            (*view).strides = ptr::null_mut();
            if (flags & ffi::PyBUF_STRIDES) == ffi::PyBUF_STRIDES {
                (*view).strides = &((*view).itemsize) as *const _ as *mut _;
            }

            (*view).suboffsets = ptr::null_mut();
            (*view).internal = ptr::null_mut();
        }

        Ok(())
    }
}


#[test]
fn test_buffer() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let t = py.init(|t| TestClass{vec: vec![b' ', b'2', b'3'], token: t}).unwrap();

    let d = PyDict::new(py);
    let _ = d.set_item(py, "ob", t);
    py.run("assert bytes(ob) == b' 23'", None, Some(&d)).unwrap();
}
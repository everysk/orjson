// SPDX-License-Identifier: (Apache-2.0 OR MIT)
// Copyright Everysk (2025-2026)

// OPT_BIG_INTEGER: serialize an int that does not fit in 64 bits (or in
// 53 bits with OPT_STRICT_INTEGER) as its decimal text instead of raising.

use crate::ffi::{Py_DECREF, PyErr_Clear, PyErr_Occurred, PyIntRef, PyStrRef};
use crate::serialize::error::SerializeError;
use crate::serialize::writer::{BytesWriter, JsonWriter};

#[cold]
#[inline(never)]
pub(crate) fn big_int_to_string(
    ob: &PyIntRef,
    error: SerializeError,
) -> Result<String, SerializeError> {
    unsafe {
        // The failed 64-bit conversion can leave an OverflowError set.
        if !PyErr_Occurred().is_null() {
            PyErr_Clear();
        }
        let pystr = pyo3_ffi::PyObject_Str(ob.as_ptr());
        if pystr.is_null() {
            PyErr_Clear();
            return Err(error);
        }
        let ret = match PyStrRef::from_ptr_unchecked(pystr).as_str() {
            Some(val) => Ok(String::from(val)),
            None => {
                PyErr_Clear();
                Err(error)
            }
        };
        Py_DECREF(pystr);
        ret
    }
}

#[cold]
#[inline(never)]
pub(crate) fn serialize_big_int(
    writer: &mut BytesWriter,
    ob: &PyIntRef,
    error: SerializeError,
) -> Result<(), SerializeError> {
    let val = big_int_to_string(ob, error)?;
    writer.reserve(val.len() + 16);
    writer.put_slice(val.as_bytes());
    Ok(())
}

//! PDF Object FFI - MuPDF API Compatible Exports
//!
//! This module provides C-compatible exports for PDF object manipulation.

use std::ffi::{c_char, CStr, CString};
use std::sync::{LazyLock, Mutex};

use super::{Handle, HandleStore};

// ============================================================================
// PDF Object Types
// ============================================================================

/// PDF Object type enumeration
#[derive(Debug, Clone)]
pub enum PdfObjType {
    Null,
    Bool(bool),
    Int(i64),
    Real(f64),
    Name(String),
    String(Vec<u8>),
    Array(Vec<PdfObj>),
    Dict(Vec<(String, PdfObj)>),
    Indirect { num: i32, generation: i32 },
    Stream { dict: Box<PdfObj>, data: Vec<u8> },
}

impl PdfObjType {
    /// Compare two object types for equality (shallow comparison)
    pub fn shallow_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PdfObjType::Null, PdfObjType::Null) => true,
            (PdfObjType::Bool(a), PdfObjType::Bool(b)) => a == b,
            (PdfObjType::Int(a), PdfObjType::Int(b)) => a == b,
            (PdfObjType::Real(a), PdfObjType::Real(b)) => (a - b).abs() < f64::EPSILON,
            (PdfObjType::Name(a), PdfObjType::Name(b)) => a == b,
            (PdfObjType::String(a), PdfObjType::String(b)) => a == b,
            (PdfObjType::Array(a), PdfObjType::Array(b)) => a.len() == b.len(),
            (PdfObjType::Dict(a), PdfObjType::Dict(b)) => a.len() == b.len(),
            (PdfObjType::Indirect { num: n1, generation: g1 }, PdfObjType::Indirect { num: n2, generation: g2 }) => {
                n1 == n2 && g1 == g2
            }
            (PdfObjType::Stream { .. }, PdfObjType::Stream { .. }) => false, // Streams never match
            _ => false,
        }
    }
}

/// Internal PDF object representation
#[derive(Debug, Clone)]
pub struct PdfObj {
    pub obj_type: PdfObjType,
    pub marked: bool,
    pub dirty: bool,
    pub parent_num: i32,
    pub refs: i32,
}

impl PdfObj {
    pub fn new_null() -> Self {
        Self {
            obj_type: PdfObjType::Null,
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_bool(b: bool) -> Self {
        Self {
            obj_type: PdfObjType::Bool(b),
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_int(i: i64) -> Self {
        Self {
            obj_type: PdfObjType::Int(i),
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_real(f: f64) -> Self {
        Self {
            obj_type: PdfObjType::Real(f),
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_name(s: &str) -> Self {
        Self {
            obj_type: PdfObjType::Name(s.to_string()),
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_string(data: &[u8]) -> Self {
        Self {
            obj_type: PdfObjType::String(data.to_vec()),
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_array(cap: usize) -> Self {
        Self {
            obj_type: PdfObjType::Array(Vec::with_capacity(cap)),
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_dict(cap: usize) -> Self {
        Self {
            obj_type: PdfObjType::Dict(Vec::with_capacity(cap)),
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }

    pub fn new_indirect(num: i32, generation: i32) -> Self {
        Self {
            obj_type: PdfObjType::Indirect { num, generation },
            marked: false,
            dirty: false,
            parent_num: 0,
            refs: 1,
        }
    }
}

/// Handle type for PDF objects
pub type PdfObjHandle = Handle;

/// Global PDF object storage
pub static PDF_OBJECTS: LazyLock<HandleStore<PdfObj>> = LazyLock::new(HandleStore::default);

// ============================================================================
// PDF Object Creation Functions
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_null(_ctx: Handle) -> PdfObjHandle {
    PDF_OBJECTS.insert(PdfObj::new_null())
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_bool(_ctx: Handle, b: i32) -> PdfObjHandle {
    PDF_OBJECTS.insert(PdfObj::new_bool(b != 0))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_int(_ctx: Handle, i: i64) -> PdfObjHandle {
    PDF_OBJECTS.insert(PdfObj::new_int(i))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_real(_ctx: Handle, f: f32) -> PdfObjHandle {
    PDF_OBJECTS.insert(PdfObj::new_real(f as f64))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_name(_ctx: Handle, str: *const c_char) -> PdfObjHandle {
    if str.is_null() {
        return PDF_OBJECTS.insert(PdfObj::new_name(""));
    }
    #[allow(unsafe_code)]
    let name = unsafe { CStr::from_ptr(str) }
        .to_str()
        .unwrap_or("");
    PDF_OBJECTS.insert(PdfObj::new_name(name))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_string(_ctx: Handle, str: *const c_char, len: usize) -> PdfObjHandle {
    if str.is_null() || len == 0 {
        return PDF_OBJECTS.insert(PdfObj::new_string(&[]));
    }
    #[allow(unsafe_code)]
    let data = unsafe { std::slice::from_raw_parts(str as *const u8, len) };
    PDF_OBJECTS.insert(PdfObj::new_string(data))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_text_string(_ctx: Handle, s: *const c_char) -> PdfObjHandle {
    if s.is_null() {
        return PDF_OBJECTS.insert(PdfObj::new_string(&[]));
    }
    #[allow(unsafe_code)]
    let text = unsafe { CStr::from_ptr(s) }
        .to_str()
        .unwrap_or("");
    PDF_OBJECTS.insert(PdfObj::new_string(text.as_bytes()))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_indirect(_ctx: Handle, _doc: Handle, num: i32, generation: i32) -> PdfObjHandle {
    PDF_OBJECTS.insert(PdfObj::new_indirect(num, generation))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_array(_ctx: Handle, _doc: Handle, initialcap: i32) -> PdfObjHandle {
    PDF_OBJECTS.insert(PdfObj::new_array(initialcap.max(0) as usize))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_dict(_ctx: Handle, _doc: Handle, initialcap: i32) -> PdfObjHandle {
    PDF_OBJECTS.insert(PdfObj::new_dict(initialcap.max(0) as usize))
}

// ============================================================================
// PDF Object Reference Counting
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_keep_obj(_ctx: Handle, obj: PdfObjHandle) -> PdfObjHandle {
    if let Some(arc) = PDF_OBJECTS.get(obj) {
        if let Ok(mut guard) = arc.lock() {
            guard.refs += 1;
        }
    }
    obj
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_drop_obj(_ctx: Handle, obj: PdfObjHandle) {
    if let Some(arc) = PDF_OBJECTS.get(obj) {
        let should_remove = {
            if let Ok(mut guard) = arc.lock() {
                guard.refs -= 1;
                guard.refs <= 0
            } else {
                false
            }
        };
        if should_remove {
            PDF_OBJECTS.remove(obj);
        }
    }
}

// ============================================================================
// Helper function to extract object properties
// ============================================================================

fn with_obj<T, F: FnOnce(&PdfObj) -> T>(obj: PdfObjHandle, default: T, f: F) -> T {
    PDF_OBJECTS
        .get(obj)
        .and_then(|arc| arc.lock().ok().map(|guard| f(&guard)))
        .unwrap_or(default)
}

fn with_obj_mut<T, F: FnOnce(&mut PdfObj) -> T>(obj: PdfObjHandle, default: T, f: F) -> T {
    PDF_OBJECTS
        .get(obj)
        .and_then(|arc| arc.lock().ok().map(|mut guard| f(&mut guard)))
        .unwrap_or(default)
}

// ============================================================================
// PDF Object Type Checking
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_null(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 1, |o| i32::from(matches!(o.obj_type, PdfObjType::Null)))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_bool(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Bool(_))))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_int(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Int(_))))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_real(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Real(_))))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_number(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| {
        i32::from(matches!(o.obj_type, PdfObjType::Int(_) | PdfObjType::Real(_)))
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_name(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Name(_))))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_string(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::String(_))))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_array(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Array(_))))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_dict(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Dict(_))))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_indirect(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Indirect { .. })))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_is_stream(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(matches!(o.obj_type, PdfObjType::Stream { .. })))
}

// ============================================================================
// PDF Object Value Extraction
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_bool(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| match &o.obj_type {
        PdfObjType::Bool(b) => i32::from(*b),
        _ => 0,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_int(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| match &o.obj_type {
        PdfObjType::Int(i) => *i as i32,
        PdfObjType::Real(f) => *f as i32,
        _ => 0,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_int64(_ctx: Handle, obj: PdfObjHandle) -> i64 {
    with_obj(obj, 0, |o| match &o.obj_type {
        PdfObjType::Int(i) => *i,
        PdfObjType::Real(f) => *f as i64,
        _ => 0,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_real(_ctx: Handle, obj: PdfObjHandle) -> f32 {
    with_obj(obj, 0.0, |o| match &o.obj_type {
        PdfObjType::Real(f) => *f as f32,
        PdfObjType::Int(i) => *i as f32,
        _ => 0.0,
    })
}

// Static storage for returned name strings
static NAME_STORAGE: LazyLock<Mutex<Vec<CString>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_name(_ctx: Handle, obj: PdfObjHandle) -> *const c_char {
    static EMPTY: &[u8] = b"\0";

    let name = with_obj(obj, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    match name {
        Some(s) => {
            if let Ok(cstring) = CString::new(s) {
                let ptr = cstring.as_ptr();
                if let Ok(mut storage) = NAME_STORAGE.lock() {
                    storage.push(cstring);
                }
                ptr
            } else {
                EMPTY.as_ptr() as *const c_char
            }
        }
        None => EMPTY.as_ptr() as *const c_char,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_num(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| match &o.obj_type {
        PdfObjType::Indirect { num, .. } => *num,
        _ => 0,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_gen(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| match &o.obj_type {
        PdfObjType::Indirect { generation, .. } => *generation,
        _ => 0,
    })
}

// ============================================================================
// PDF Object Value Extraction with Defaults
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_bool_default(_ctx: Handle, obj: PdfObjHandle, def: i32) -> i32 {
    with_obj(obj, def, |o| match &o.obj_type {
        PdfObjType::Bool(b) => i32::from(*b),
        _ => def,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_int_default(_ctx: Handle, obj: PdfObjHandle, def: i32) -> i32 {
    with_obj(obj, def, |o| match &o.obj_type {
        PdfObjType::Int(i) => *i as i32,
        PdfObjType::Real(f) => *f as i32,
        _ => def,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_real_default(_ctx: Handle, obj: PdfObjHandle, def: f32) -> f32 {
    with_obj(obj, def, |o| match &o.obj_type {
        PdfObjType::Real(f) => *f as f32,
        PdfObjType::Int(i) => *i as f32,
        _ => def,
    })
}

// ============================================================================
// PDF Array Operations
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_len(_ctx: Handle, array: PdfObjHandle) -> i32 {
    with_obj(array, 0, |o| match &o.obj_type {
        PdfObjType::Array(arr) => arr.len() as i32,
        _ => 0,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_push(_ctx: Handle, array: PdfObjHandle, obj: PdfObjHandle) {
    let obj_to_push = with_obj(obj, None, |o| Some(o.clone()));

    if let Some(obj_clone) = obj_to_push {
        with_obj_mut(array, (), |arr| {
            if let PdfObjType::Array(ref mut a) = arr.obj_type {
                a.push(obj_clone);
                arr.dirty = true;
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_push_int(_ctx: Handle, array: PdfObjHandle, x: i64) {
    with_obj_mut(array, (), |arr| {
        if let PdfObjType::Array(ref mut a) = arr.obj_type {
            a.push(PdfObj::new_int(x));
            arr.dirty = true;
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_push_real(_ctx: Handle, array: PdfObjHandle, x: f64) {
    with_obj_mut(array, (), |arr| {
        if let PdfObjType::Array(ref mut a) = arr.obj_type {
            a.push(PdfObj::new_real(x));
            arr.dirty = true;
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_push_bool(_ctx: Handle, array: PdfObjHandle, x: i32) {
    with_obj_mut(array, (), |arr| {
        if let PdfObjType::Array(ref mut a) = arr.obj_type {
            a.push(PdfObj::new_bool(x != 0));
            arr.dirty = true;
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_delete(_ctx: Handle, array: PdfObjHandle, index: i32) {
    with_obj_mut(array, (), |arr| {
        if let PdfObjType::Array(ref mut a) = arr.obj_type {
            let idx = index as usize;
            if idx < a.len() {
                a.remove(idx);
                arr.dirty = true;
            }
        }
    });
}

// ============================================================================
// PDF Dictionary Operations
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_len(_ctx: Handle, dict: PdfObjHandle) -> i32 {
    with_obj(dict, 0, |o| match &o.obj_type {
        PdfObjType::Dict(d) => d.len() as i32,
        _ => 0,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_puts(
    _ctx: Handle,
    dict: PdfObjHandle,
    key: *const c_char,
    val: PdfObjHandle,
) {
    if key.is_null() {
        return;
    }

    #[allow(unsafe_code)]
    let key_str = unsafe { CStr::from_ptr(key) }
        .to_str()
        .unwrap_or("")
        .to_string();

    let val_obj = with_obj(val, None, |o| Some(o.clone()));

    if let Some(val_clone) = val_obj {
        with_obj_mut(dict, (), |d| {
            if let PdfObjType::Dict(ref mut dict_entries) = d.obj_type {
                if let Some(entry) = dict_entries.iter_mut().find(|(k, _)| k == &key_str) {
                    entry.1 = val_clone;
                } else {
                    dict_entries.push((key_str.clone(), val_clone));
                }
                d.dirty = true;
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_dels(_ctx: Handle, dict: PdfObjHandle, key: *const c_char) {
    if key.is_null() {
        return;
    }

    #[allow(unsafe_code)]
    let key_str = unsafe { CStr::from_ptr(key) }
        .to_str()
        .unwrap_or("")
        .to_string();

    with_obj_mut(dict, (), |d| {
        if let PdfObjType::Dict(ref mut dict_entries) = d.obj_type {
            dict_entries.retain(|(k, _)| k != &key_str);
            d.dirty = true;
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_put_int(
    _ctx: Handle,
    dict: PdfObjHandle,
    key: PdfObjHandle,
    x: i64,
) {
    let key_name = with_obj(key, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    if let Some(key_str) = key_name {
        with_obj_mut(dict, (), |d| {
            if let PdfObjType::Dict(ref mut dict_entries) = d.obj_type {
                let val = PdfObj::new_int(x);
                if let Some(entry) = dict_entries.iter_mut().find(|(k, _)| k == &key_str) {
                    entry.1 = val;
                } else {
                    dict_entries.push((key_str.clone(), val));
                }
                d.dirty = true;
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_put_real(
    _ctx: Handle,
    dict: PdfObjHandle,
    key: PdfObjHandle,
    x: f64,
) {
    let key_name = with_obj(key, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    if let Some(key_str) = key_name {
        with_obj_mut(dict, (), |d| {
            if let PdfObjType::Dict(ref mut dict_entries) = d.obj_type {
                let val = PdfObj::new_real(x);
                if let Some(entry) = dict_entries.iter_mut().find(|(k, _)| k == &key_str) {
                    entry.1 = val;
                } else {
                    dict_entries.push((key_str.clone(), val));
                }
                d.dirty = true;
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_put_bool(
    _ctx: Handle,
    dict: PdfObjHandle,
    key: PdfObjHandle,
    x: i32,
) {
    let key_name = with_obj(key, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    if let Some(key_str) = key_name {
        with_obj_mut(dict, (), |d| {
            if let PdfObjType::Dict(ref mut dict_entries) = d.obj_type {
                let val = PdfObj::new_bool(x != 0);
                if let Some(entry) = dict_entries.iter_mut().find(|(k, _)| k == &key_str) {
                    entry.1 = val;
                } else {
                    dict_entries.push((key_str.clone(), val));
                }
                d.dirty = true;
            }
        });
    }
}

// ============================================================================
// PDF Object Marking
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_obj_marked(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(o.marked))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_mark_obj(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj_mut(obj, 0, |o| {
        let was_marked = o.marked;
        o.marked = true;
        i32::from(was_marked)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_unmark_obj(_ctx: Handle, obj: PdfObjHandle) {
    with_obj_mut(obj, (), |o| {
        o.marked = false;
    });
}

// ============================================================================
// PDF Object Dirty Tracking
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_obj_is_dirty(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| i32::from(o.dirty))
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dirty_obj(_ctx: Handle, obj: PdfObjHandle) {
    with_obj_mut(obj, (), |o| {
        o.dirty = true;
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_clean_obj(_ctx: Handle, obj: PdfObjHandle) {
    with_obj_mut(obj, (), |o| {
        o.dirty = false;
    });
}

// ============================================================================
// PDF Object Parent
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_set_obj_parent(_ctx: Handle, obj: PdfObjHandle, num: i32) {
    with_obj_mut(obj, (), |o| {
        o.parent_num = num;
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_obj_parent_num(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| o.parent_num)
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_obj_refs(_ctx: Handle, obj: PdfObjHandle) -> i32 {
    with_obj(obj, 0, |o| o.refs)
}

// ============================================================================
// PDF Object Comparison
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_objcmp(_ctx: Handle, a: PdfObjHandle, b: PdfObjHandle) -> i32 {
    let obj_a = with_obj(a, None, |o| Some(o.obj_type.clone()));
    let obj_b = with_obj(b, None, |o| Some(o.obj_type.clone()));

    match (obj_a, obj_b) {
        (Some(a_type), Some(b_type)) => i32::from(!a_type.shallow_eq(&b_type)),
        _ => 1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_name_eq(_ctx: Handle, a: PdfObjHandle, b: PdfObjHandle) -> i32 {
    let name_a = with_obj(a, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    let name_b = with_obj(b, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    match (name_a, name_b) {
        (Some(a_name), Some(b_name)) => i32::from(a_name == b_name),
        _ => 0,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_object_creation() {
        let null = pdf_new_null(0);
        assert_eq!(pdf_is_null(0, null), 1);

        let bool_true = pdf_new_bool(0, 1);
        assert_eq!(pdf_is_bool(0, bool_true), 1);
        assert_eq!(pdf_to_bool(0, bool_true), 1);

        let int_val = pdf_new_int(0, 42);
        assert_eq!(pdf_is_int(0, int_val), 1);
        assert_eq!(pdf_to_int(0, int_val), 42);

        let real_val = pdf_new_real(0, 3.14);
        assert_eq!(pdf_is_real(0, real_val), 1);
        assert!((pdf_to_real(0, real_val) - 3.14).abs() < 0.01);
    }

    #[test]
    fn test_pdf_array_operations() {
        let arr = pdf_new_array(0, 0, 10);
        assert_eq!(pdf_is_array(0, arr), 1);
        assert_eq!(pdf_array_len(0, arr), 0);

        pdf_array_push_int(0, arr, 100);
        assert_eq!(pdf_array_len(0, arr), 1);

        pdf_array_push_real(0, arr, 2.5);
        assert_eq!(pdf_array_len(0, arr), 2);
    }

    #[test]
    fn test_pdf_dict_operations() {
        let dict = pdf_new_dict(0, 0, 10);
        assert_eq!(pdf_is_dict(0, dict), 1);
        assert_eq!(pdf_dict_len(0, dict), 0);

        let key = pdf_new_name(0, b"Type\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, dict, key, 42);
        assert_eq!(pdf_dict_len(0, dict), 1);
    }

    #[test]
    fn test_pdf_object_marking() {
        let obj = pdf_new_int(0, 1);
        assert_eq!(pdf_obj_marked(0, obj), 0);

        pdf_mark_obj(0, obj);
        assert_eq!(pdf_obj_marked(0, obj), 1);

        pdf_unmark_obj(0, obj);
        assert_eq!(pdf_obj_marked(0, obj), 0);
    }

    #[test]
    fn test_pdf_object_comparison() {
        let int1 = pdf_new_int(0, 42);
        let int2 = pdf_new_int(0, 42);
        let int3 = pdf_new_int(0, 100);

        assert_eq!(pdf_objcmp(0, int1, int2), 0); // Equal
        assert_eq!(pdf_objcmp(0, int1, int3), 1); // Not equal

        let name1 = pdf_new_name(0, b"Test\0".as_ptr() as *const c_char);
        let name2 = pdf_new_name(0, b"Test\0".as_ptr() as *const c_char);
        let name3 = pdf_new_name(0, b"Other\0".as_ptr() as *const c_char);

        assert_eq!(pdf_name_eq(0, name1, name2), 1); // Equal
        assert_eq!(pdf_name_eq(0, name1, name3), 0); // Not equal
    }
}

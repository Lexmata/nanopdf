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
// PDF Array Get/Put Operations
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_get(_ctx: Handle, array: PdfObjHandle, index: i32) -> PdfObjHandle {
    let obj = with_obj(array, None, |o| match &o.obj_type {
        PdfObjType::Array(arr) => {
            let idx = index as usize;
            if idx < arr.len() {
                Some(arr[idx].clone())
            } else {
                None
            }
        }
        _ => None,
    });

    match obj {
        Some(o) => PDF_OBJECTS.insert(o),
        None => 0, // Return null handle
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_put(_ctx: Handle, array: PdfObjHandle, index: i32, obj: PdfObjHandle) {
    let obj_to_put = with_obj(obj, None, |o| Some(o.clone()));

    if let Some(obj_clone) = obj_to_put {
        with_obj_mut(array, (), |arr| {
            if let PdfObjType::Array(ref mut a) = arr.obj_type {
                let idx = index as usize;
                if idx < a.len() {
                    a[idx] = obj_clone;
                    arr.dirty = true;
                } else if idx == a.len() {
                    // Allow appending at the end
                    a.push(obj_clone);
                    arr.dirty = true;
                }
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_insert(_ctx: Handle, array: PdfObjHandle, index: i32, obj: PdfObjHandle) {
    let obj_to_insert = with_obj(obj, None, |o| Some(o.clone()));

    if let Some(obj_clone) = obj_to_insert {
        with_obj_mut(array, (), |arr| {
            if let PdfObjType::Array(ref mut a) = arr.obj_type {
                let idx = (index as usize).min(a.len());
                a.insert(idx, obj_clone);
                arr.dirty = true;
            }
        });
    }
}

// ============================================================================
// PDF Dictionary Get/Put Operations
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_get(_ctx: Handle, dict: PdfObjHandle, key: PdfObjHandle) -> PdfObjHandle {
    let key_name = with_obj(key, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    let key_str = match key_name {
        Some(k) => k,
        None => return 0,
    };

    let obj = with_obj(dict, None, |o| match &o.obj_type {
        PdfObjType::Dict(entries) => {
            entries.iter()
                .find(|(k, _)| k == &key_str)
                .map(|(_, v)| v.clone())
        }
        _ => None,
    });

    match obj {
        Some(o) => PDF_OBJECTS.insert(o),
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_gets(_ctx: Handle, dict: PdfObjHandle, key: *const c_char) -> PdfObjHandle {
    if key.is_null() {
        return 0;
    }

    #[allow(unsafe_code)]
    let key_str = unsafe { CStr::from_ptr(key) }
        .to_str()
        .unwrap_or("")
        .to_string();

    let obj = with_obj(dict, None, |o| match &o.obj_type {
        PdfObjType::Dict(entries) => {
            entries.iter()
                .find(|(k, _)| k == &key_str)
                .map(|(_, v)| v.clone())
        }
        _ => None,
    });

    match obj {
        Some(o) => PDF_OBJECTS.insert(o),
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_put(_ctx: Handle, dict: PdfObjHandle, key: PdfObjHandle, val: PdfObjHandle) {
    let key_name = with_obj(key, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    let key_str = match key_name {
        Some(k) => k,
        None => return,
    };

    let val_obj = with_obj(val, None, |o| Some(o.clone()));

    if let Some(val_clone) = val_obj {
        with_obj_mut(dict, (), |d| {
            if let PdfObjType::Dict(ref mut entries) = d.obj_type {
                if let Some(entry) = entries.iter_mut().find(|(k, _)| k == &key_str) {
                    entry.1 = val_clone;
                } else {
                    entries.push((key_str.clone(), val_clone));
                }
                d.dirty = true;
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_put_name(_ctx: Handle, dict: PdfObjHandle, key: PdfObjHandle, name: *const c_char) {
    if name.is_null() {
        return;
    }

    let key_name = with_obj(key, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    let key_str = match key_name {
        Some(k) => k,
        None => return,
    };

    #[allow(unsafe_code)]
    let name_str = unsafe { CStr::from_ptr(name) }
        .to_str()
        .unwrap_or("");

    with_obj_mut(dict, (), |d| {
        if let PdfObjType::Dict(ref mut entries) = d.obj_type {
            let val = PdfObj::new_name(name_str);
            if let Some(entry) = entries.iter_mut().find(|(k, _)| k == &key_str) {
                entry.1 = val;
            } else {
                entries.push((key_str.clone(), val));
            }
            d.dirty = true;
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_put_string(_ctx: Handle, dict: PdfObjHandle, key: PdfObjHandle, str: *const c_char, len: usize) {
    let key_name = with_obj(key, None, |o| match &o.obj_type {
        PdfObjType::Name(s) => Some(s.clone()),
        _ => None,
    });

    let key_str = match key_name {
        Some(k) => k,
        None => return,
    };

    let data = if str.is_null() || len == 0 {
        Vec::new()
    } else {
        #[allow(unsafe_code)]
        unsafe { std::slice::from_raw_parts(str as *const u8, len) }.to_vec()
    };

    with_obj_mut(dict, (), |d| {
        if let PdfObjType::Dict(ref mut entries) = d.obj_type {
            let val = PdfObj::new_string(&data);
            if let Some(entry) = entries.iter_mut().find(|(k, _)| k == &key_str) {
                entry.1 = val;
            } else {
                entries.push((key_str.clone(), val));
            }
            d.dirty = true;
        }
    });
}

// ============================================================================
// PDF String Extraction
// ============================================================================

static STRING_STORAGE: LazyLock<Mutex<Vec<Vec<u8>>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_string(_ctx: Handle, obj: PdfObjHandle, sizep: *mut usize) -> *const c_char {
    let data = with_obj(obj, None, |o| match &o.obj_type {
        PdfObjType::String(s) => Some(s.clone()),
        _ => None,
    });

    match data {
        Some(s) => {
            if !sizep.is_null() {
                #[allow(unsafe_code)]
                unsafe { *sizep = s.len(); }
            }
            let ptr = s.as_ptr() as *const c_char;
            if let Ok(mut storage) = STRING_STORAGE.lock() {
                storage.push(s);
            }
            ptr
        }
        None => {
            if !sizep.is_null() {
                #[allow(unsafe_code)]
                unsafe { *sizep = 0; }
            }
            std::ptr::null()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_str_buf(_ctx: Handle, obj: PdfObjHandle) -> *const c_char {
    pdf_to_string(_ctx, obj, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_str_len(_ctx: Handle, obj: PdfObjHandle) -> usize {
    with_obj(obj, 0, |o| match &o.obj_type {
        PdfObjType::String(s) => s.len(),
        _ => 0,
    })
}

// ============================================================================
// PDF Object Copy Operations
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_copy_array(_ctx: Handle, _doc: Handle, array: PdfObjHandle) -> PdfObjHandle {
    let copied = with_obj(array, None, |o| match &o.obj_type {
        PdfObjType::Array(arr) => {
            let mut new_arr = PdfObj::new_array(arr.len());
            if let PdfObjType::Array(ref mut new_vec) = new_arr.obj_type {
                for item in arr {
                    new_vec.push(item.clone());
                }
            }
            Some(new_arr)
        }
        _ => None,
    });

    match copied {
        Some(obj) => PDF_OBJECTS.insert(obj),
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_copy_dict(_ctx: Handle, _doc: Handle, dict: PdfObjHandle) -> PdfObjHandle {
    let copied = with_obj(dict, None, |o| match &o.obj_type {
        PdfObjType::Dict(entries) => {
            let mut new_dict = PdfObj::new_dict(entries.len());
            if let PdfObjType::Dict(ref mut new_entries) = new_dict.obj_type {
                for (k, v) in entries {
                    new_entries.push((k.clone(), v.clone()));
                }
            }
            Some(new_dict)
        }
        _ => None,
    });

    match copied {
        Some(obj) => PDF_OBJECTS.insert(obj),
        None => 0,
    }
}

fn deep_copy_obj_inner(obj: &PdfObj) -> PdfObj {
    let new_type = match &obj.obj_type {
        PdfObjType::Null => PdfObjType::Null,
        PdfObjType::Bool(b) => PdfObjType::Bool(*b),
        PdfObjType::Int(i) => PdfObjType::Int(*i),
        PdfObjType::Real(r) => PdfObjType::Real(*r),
        PdfObjType::Name(s) => PdfObjType::Name(s.clone()),
        PdfObjType::String(s) => PdfObjType::String(s.clone()),
        PdfObjType::Array(arr) => {
            PdfObjType::Array(arr.iter().map(deep_copy_obj_inner).collect())
        }
        PdfObjType::Dict(entries) => {
            PdfObjType::Dict(
                entries.iter()
                    .map(|(k, v)| (k.clone(), deep_copy_obj_inner(v)))
                    .collect()
            )
        }
        PdfObjType::Indirect { num, generation } => {
            PdfObjType::Indirect { num: *num, generation: *generation }
        }
        PdfObjType::Stream { dict, data } => {
            PdfObjType::Stream {
                dict: Box::new(deep_copy_obj_inner(dict)),
                data: data.clone(),
            }
        }
    };

    PdfObj {
        obj_type: new_type,
        marked: false,
        dirty: false,
        parent_num: 0,
        refs: 1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_deep_copy_obj(_ctx: Handle, _doc: Handle, obj: PdfObjHandle) -> PdfObjHandle {
    let copied = with_obj(obj, None, |o| Some(deep_copy_obj_inner(o)));

    match copied {
        Some(new_obj) => PDF_OBJECTS.insert(new_obj),
        None => 0,
    }
}

// ============================================================================
// PDF Geometry Object Creation
// ============================================================================

/// Create a PDF array representing a point [x, y]
#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_point(_ctx: Handle, _doc: Handle, x: f32, y: f32) -> PdfObjHandle {
    let mut arr = PdfObj::new_array(2);
    if let PdfObjType::Array(ref mut a) = arr.obj_type {
        a.push(PdfObj::new_real(x as f64));
        a.push(PdfObj::new_real(y as f64));
    }
    PDF_OBJECTS.insert(arr)
}

/// Create a PDF array representing a rect [x0, y0, x1, y1]
#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_rect(_ctx: Handle, _doc: Handle, x0: f32, y0: f32, x1: f32, y1: f32) -> PdfObjHandle {
    let mut arr = PdfObj::new_array(4);
    if let PdfObjType::Array(ref mut a) = arr.obj_type {
        a.push(PdfObj::new_real(x0 as f64));
        a.push(PdfObj::new_real(y0 as f64));
        a.push(PdfObj::new_real(x1 as f64));
        a.push(PdfObj::new_real(y1 as f64));
    }
    PDF_OBJECTS.insert(arr)
}

/// Create a PDF array representing a matrix [a, b, c, d, e, f]
#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_matrix(_ctx: Handle, _doc: Handle, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> PdfObjHandle {
    let mut arr = PdfObj::new_array(6);
    if let PdfObjType::Array(ref mut arr_vec) = arr.obj_type {
        arr_vec.push(PdfObj::new_real(a as f64));
        arr_vec.push(PdfObj::new_real(b as f64));
        arr_vec.push(PdfObj::new_real(c as f64));
        arr_vec.push(PdfObj::new_real(d as f64));
        arr_vec.push(PdfObj::new_real(e as f64));
        arr_vec.push(PdfObj::new_real(f as f64));
    }
    PDF_OBJECTS.insert(arr)
}

/// Create a PDF date string from components
#[unsafe(no_mangle)]
pub extern "C" fn pdf_new_date(_ctx: Handle, _doc: Handle, year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> PdfObjHandle {
    // PDF date format: D:YYYYMMDDHHmmSS
    let date_str = format!(
        "D:{:04}{:02}{:02}{:02}{:02}{:02}",
        year, month, day, hour, minute, second
    );
    PDF_OBJECTS.insert(PdfObj::new_string(date_str.as_bytes()))
}

// ============================================================================
// PDF Array/Dict Key Access
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_get_key(_ctx: Handle, dict: PdfObjHandle, index: i32) -> PdfObjHandle {
    let key = with_obj(dict, None, |o| match &o.obj_type {
        PdfObjType::Dict(entries) => {
            let idx = index as usize;
            if idx < entries.len() {
                Some(PdfObj::new_name(&entries[idx].0))
            } else {
                None
            }
        }
        _ => None,
    });

    match key {
        Some(k) => PDF_OBJECTS.insert(k),
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_dict_get_val(_ctx: Handle, dict: PdfObjHandle, index: i32) -> PdfObjHandle {
    let val = with_obj(dict, None, |o| match &o.obj_type {
        PdfObjType::Dict(entries) => {
            let idx = index as usize;
            if idx < entries.len() {
                Some(entries[idx].1.clone())
            } else {
                None
            }
        }
        _ => None,
    });

    match val {
        Some(v) => PDF_OBJECTS.insert(v),
        None => 0,
    }
}

// ============================================================================
// PDF Array Push Name/String
// ============================================================================

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_push_name(_ctx: Handle, array: PdfObjHandle, name: *const c_char) {
    if name.is_null() {
        return;
    }

    #[allow(unsafe_code)]
    let name_str = unsafe { CStr::from_ptr(name) }
        .to_str()
        .unwrap_or("");

    with_obj_mut(array, (), |arr| {
        if let PdfObjType::Array(ref mut a) = arr.obj_type {
            a.push(PdfObj::new_name(name_str));
            arr.dirty = true;
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pdf_array_push_string(_ctx: Handle, array: PdfObjHandle, str: *const c_char, len: usize) {
    let data = if str.is_null() || len == 0 {
        Vec::new()
    } else {
        #[allow(unsafe_code)]
        unsafe { std::slice::from_raw_parts(str as *const u8, len) }.to_vec()
    };

    with_obj_mut(array, (), |arr| {
        if let PdfObjType::Array(ref mut a) = arr.obj_type {
            a.push(PdfObj::new_string(&data));
            arr.dirty = true;
        }
    });
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Object Creation Tests
    // ============================================================================

    #[test]
    fn test_pdf_new_null() {
        let null = pdf_new_null(0);
        assert_eq!(pdf_is_null(0, null), 1);
        assert_eq!(pdf_is_bool(0, null), 0);
        assert_eq!(pdf_is_int(0, null), 0);
    }

    #[test]
    fn test_pdf_new_bool() {
        let bool_true = pdf_new_bool(0, 1);
        assert_eq!(pdf_is_bool(0, bool_true), 1);
        assert_eq!(pdf_to_bool(0, bool_true), 1);

        let bool_false = pdf_new_bool(0, 0);
        assert_eq!(pdf_is_bool(0, bool_false), 1);
        assert_eq!(pdf_to_bool(0, bool_false), 0);

        // Non-zero should also be true
        let bool_nonzero = pdf_new_bool(0, 42);
        assert_eq!(pdf_to_bool(0, bool_nonzero), 1);
    }

    #[test]
    fn test_pdf_new_int() {
        let int_val = pdf_new_int(0, 42);
        assert_eq!(pdf_is_int(0, int_val), 1);
        assert_eq!(pdf_to_int(0, int_val), 42);
        assert_eq!(pdf_to_int64(0, int_val), 42);

        // Negative value
        let neg_val = pdf_new_int(0, -100);
        assert_eq!(pdf_to_int(0, neg_val), -100);
        assert_eq!(pdf_to_int64(0, neg_val), -100);

        // Large value
        let large_val = pdf_new_int(0, i64::MAX);
        assert_eq!(pdf_to_int64(0, large_val), i64::MAX);
    }

    #[test]
    fn test_pdf_new_real() {
        let real_val = pdf_new_real(0, 3.14);
        assert_eq!(pdf_is_real(0, real_val), 1);
        assert!((pdf_to_real(0, real_val) - 3.14).abs() < 0.01);

        // Negative value
        let neg_real = pdf_new_real(0, -2.5);
        assert!((pdf_to_real(0, neg_real) + 2.5).abs() < 0.01);
    }

    #[test]
    fn test_pdf_is_number() {
        let int_val = pdf_new_int(0, 42);
        let real_val = pdf_new_real(0, 3.14);
        let null_val = pdf_new_null(0);

        assert_eq!(pdf_is_number(0, int_val), 1);
        assert_eq!(pdf_is_number(0, real_val), 1);
        assert_eq!(pdf_is_number(0, null_val), 0);
    }

    #[test]
    fn test_pdf_new_name() {
        let name = pdf_new_name(0, b"Type\0".as_ptr() as *const c_char);
        assert_eq!(pdf_is_name(0, name), 1);

        // Empty name
        let empty_name = pdf_new_name(0, std::ptr::null());
        assert_eq!(pdf_is_name(0, empty_name), 1);
    }

    #[test]
    fn test_pdf_new_string() {
        let data = b"Hello, PDF!";
        let str_obj = pdf_new_string(0, data.as_ptr() as *const c_char, data.len());
        assert_eq!(pdf_is_string(0, str_obj), 1);

        // Empty string
        let empty_str = pdf_new_string(0, std::ptr::null(), 0);
        assert_eq!(pdf_is_string(0, empty_str), 1);

        // Null pointer with non-zero length
        let null_str = pdf_new_string(0, std::ptr::null(), 10);
        assert_eq!(pdf_is_string(0, null_str), 1);
    }

    #[test]
    fn test_pdf_new_text_string() {
        let text_obj = pdf_new_text_string(0, b"Hello World\0".as_ptr() as *const c_char);
        assert_eq!(pdf_is_string(0, text_obj), 1);

        // Null text
        let null_text = pdf_new_text_string(0, std::ptr::null());
        assert_eq!(pdf_is_string(0, null_text), 1);
    }

    #[test]
    fn test_pdf_new_indirect() {
        let indirect = pdf_new_indirect(0, 0, 10, 2);
        assert_eq!(pdf_is_indirect(0, indirect), 1);
        assert_eq!(pdf_to_num(0, indirect), 10);
        assert_eq!(pdf_to_gen(0, indirect), 2);
    }

    // ============================================================================
    // Reference Counting Tests
    // ============================================================================

    #[test]
    fn test_pdf_keep_drop_obj() {
        let obj = pdf_new_int(0, 42);
        assert_eq!(pdf_obj_refs(0, obj), 1);

        pdf_keep_obj(0, obj);
        assert_eq!(pdf_obj_refs(0, obj), 2);

        pdf_drop_obj(0, obj);
        assert_eq!(pdf_obj_refs(0, obj), 1);

        pdf_drop_obj(0, obj);
        // Object should be removed, so refs should be 0 (default)
        assert_eq!(pdf_obj_refs(0, obj), 0);
    }

    #[test]
    fn test_pdf_keep_invalid_handle() {
        let invalid = pdf_keep_obj(0, 99999);
        assert_eq!(invalid, 99999); // Should return same handle
    }

    // ============================================================================
    // Value Extraction with Defaults Tests
    // ============================================================================

    #[test]
    fn test_pdf_to_bool_default() {
        let bool_obj = pdf_new_bool(0, 1);
        let null_obj = pdf_new_null(0);

        assert_eq!(pdf_to_bool_default(0, bool_obj, 0), 1);
        assert_eq!(pdf_to_bool_default(0, null_obj, 99), 99);
    }

    #[test]
    fn test_pdf_to_int_default() {
        let int_obj = pdf_new_int(0, 42);
        let null_obj = pdf_new_null(0);
        let real_obj = pdf_new_real(0, 3.7);

        assert_eq!(pdf_to_int_default(0, int_obj, 0), 42);
        assert_eq!(pdf_to_int_default(0, null_obj, 99), 99);
        assert_eq!(pdf_to_int_default(0, real_obj, 0), 3); // Truncated
    }

    #[test]
    fn test_pdf_to_real_default() {
        let real_obj = pdf_new_real(0, 3.14);
        let null_obj = pdf_new_null(0);
        let int_obj = pdf_new_int(0, 5);

        assert!((pdf_to_real_default(0, real_obj, 0.0) - 3.14).abs() < 0.01);
        assert!((pdf_to_real_default(0, null_obj, 99.0) - 99.0).abs() < 0.01);
        assert!((pdf_to_real_default(0, int_obj, 0.0) - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_pdf_to_name() {
        let name = pdf_new_name(0, b"TestName\0".as_ptr() as *const c_char);
        let ptr = pdf_to_name(0, name);
        assert!(!ptr.is_null());

        // Test non-name object returns empty
        let int_obj = pdf_new_int(0, 42);
        let ptr2 = pdf_to_name(0, int_obj);
        assert!(!ptr2.is_null());
    }

    // ============================================================================
    // Array Operations Tests
    // ============================================================================

    #[test]
    fn test_pdf_array_operations() {
        let arr = pdf_new_array(0, 0, 10);
        assert_eq!(pdf_is_array(0, arr), 1);
        assert_eq!(pdf_array_len(0, arr), 0);

        // Push int
        pdf_array_push_int(0, arr, 100);
        assert_eq!(pdf_array_len(0, arr), 1);

        // Push real
        pdf_array_push_real(0, arr, 2.5);
        assert_eq!(pdf_array_len(0, arr), 2);

        // Push bool
        pdf_array_push_bool(0, arr, 1);
        assert_eq!(pdf_array_len(0, arr), 3);

        // Push object
        let obj = pdf_new_int(0, 42);
        pdf_array_push(0, arr, obj);
        assert_eq!(pdf_array_len(0, arr), 4);

        // Delete
        pdf_array_delete(0, arr, 0);
        assert_eq!(pdf_array_len(0, arr), 3);

        // Delete out of bounds (should not crash)
        pdf_array_delete(0, arr, 100);
        assert_eq!(pdf_array_len(0, arr), 3);
    }

    #[test]
    fn test_pdf_array_len_non_array() {
        let dict = pdf_new_dict(0, 0, 10);
        assert_eq!(pdf_array_len(0, dict), 0);

        let null = pdf_new_null(0);
        assert_eq!(pdf_array_len(0, null), 0);
    }

    #[test]
    fn test_pdf_array_push_to_non_array() {
        let dict = pdf_new_dict(0, 0, 10);
        pdf_array_push_int(0, dict, 42); // Should not crash
        assert_eq!(pdf_dict_len(0, dict), 0); // Dict unchanged
    }

    // ============================================================================
    // Dictionary Operations Tests
    // ============================================================================

    #[test]
    fn test_pdf_dict_operations() {
        let dict = pdf_new_dict(0, 0, 10);
        assert_eq!(pdf_is_dict(0, dict), 1);
        assert_eq!(pdf_dict_len(0, dict), 0);

        // Put int with name key
        let key1 = pdf_new_name(0, b"Type\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, dict, key1, 42);
        assert_eq!(pdf_dict_len(0, dict), 1);

        // Put real
        let key2 = pdf_new_name(0, b"Width\0".as_ptr() as *const c_char);
        pdf_dict_put_real(0, dict, key2, 100.5);
        assert_eq!(pdf_dict_len(0, dict), 2);

        // Put bool
        let key3 = pdf_new_name(0, b"Enabled\0".as_ptr() as *const c_char);
        pdf_dict_put_bool(0, dict, key3, 1);
        assert_eq!(pdf_dict_len(0, dict), 3);

        // Update existing key
        pdf_dict_put_int(0, dict, key1, 99);
        assert_eq!(pdf_dict_len(0, dict), 3); // Length unchanged

        // Delete by string key
        pdf_dict_dels(0, dict, b"Width\0".as_ptr() as *const c_char);
        assert_eq!(pdf_dict_len(0, dict), 2);
    }

    #[test]
    fn test_pdf_dict_puts() {
        let dict = pdf_new_dict(0, 0, 10);
        let val = pdf_new_int(0, 42);

        pdf_dict_puts(0, dict, b"Key\0".as_ptr() as *const c_char, val);
        assert_eq!(pdf_dict_len(0, dict), 1);

        // Null key
        pdf_dict_puts(0, dict, std::ptr::null(), val);
        assert_eq!(pdf_dict_len(0, dict), 1); // Unchanged
    }

    #[test]
    fn test_pdf_dict_dels_null_key() {
        let dict = pdf_new_dict(0, 0, 10);
        pdf_dict_dels(0, dict, std::ptr::null()); // Should not crash
    }

    #[test]
    fn test_pdf_dict_put_with_non_name_key() {
        let dict = pdf_new_dict(0, 0, 10);
        let int_key = pdf_new_int(0, 42); // Not a name

        pdf_dict_put_int(0, dict, int_key, 100);
        assert_eq!(pdf_dict_len(0, dict), 0); // Should be unchanged
    }

    #[test]
    fn test_pdf_dict_len_non_dict() {
        let arr = pdf_new_array(0, 0, 10);
        assert_eq!(pdf_dict_len(0, arr), 0);
    }

    // ============================================================================
    // Object Marking Tests
    // ============================================================================

    #[test]
    fn test_pdf_object_marking() {
        let obj = pdf_new_int(0, 1);
        assert_eq!(pdf_obj_marked(0, obj), 0);

        let was_marked = pdf_mark_obj(0, obj);
        assert_eq!(was_marked, 0); // Was not marked before
        assert_eq!(pdf_obj_marked(0, obj), 1);

        let was_marked2 = pdf_mark_obj(0, obj);
        assert_eq!(was_marked2, 1); // Was marked before

        pdf_unmark_obj(0, obj);
        assert_eq!(pdf_obj_marked(0, obj), 0);
    }

    // ============================================================================
    // Object Dirty Tracking Tests
    // ============================================================================

    #[test]
    fn test_pdf_object_dirty() {
        let obj = pdf_new_int(0, 1);
        assert_eq!(pdf_obj_is_dirty(0, obj), 0);

        pdf_dirty_obj(0, obj);
        assert_eq!(pdf_obj_is_dirty(0, obj), 1);

        pdf_clean_obj(0, obj);
        assert_eq!(pdf_obj_is_dirty(0, obj), 0);
    }

    // ============================================================================
    // Parent Number Tests
    // ============================================================================

    #[test]
    fn test_pdf_obj_parent_num() {
        let obj = pdf_new_int(0, 42);
        assert_eq!(pdf_obj_parent_num(0, obj), 0);

        pdf_set_obj_parent(0, obj, 100);
        assert_eq!(pdf_obj_parent_num(0, obj), 100);
    }

    // ============================================================================
    // Object Comparison Tests
    // ============================================================================

    #[test]
    fn test_pdf_objcmp() {
        // Same type, same value
        let int1 = pdf_new_int(0, 42);
        let int2 = pdf_new_int(0, 42);
        assert_eq!(pdf_objcmp(0, int1, int2), 0);

        // Same type, different value
        let int3 = pdf_new_int(0, 100);
        assert_eq!(pdf_objcmp(0, int1, int3), 1);

        // Different types
        let real = pdf_new_real(0, 42.0);
        assert_eq!(pdf_objcmp(0, int1, real), 1);

        // Null objects
        let null1 = pdf_new_null(0);
        let null2 = pdf_new_null(0);
        assert_eq!(pdf_objcmp(0, null1, null2), 0);

        // Bool comparison
        let bool1 = pdf_new_bool(0, 1);
        let bool2 = pdf_new_bool(0, 1);
        let bool3 = pdf_new_bool(0, 0);
        assert_eq!(pdf_objcmp(0, bool1, bool2), 0);
        assert_eq!(pdf_objcmp(0, bool1, bool3), 1);

        // Invalid handles
        assert_eq!(pdf_objcmp(0, 99999, 99998), 1);
    }

    #[test]
    fn test_pdf_name_eq() {
        let name1 = pdf_new_name(0, b"Test\0".as_ptr() as *const c_char);
        let name2 = pdf_new_name(0, b"Test\0".as_ptr() as *const c_char);
        let name3 = pdf_new_name(0, b"Other\0".as_ptr() as *const c_char);

        assert_eq!(pdf_name_eq(0, name1, name2), 1);
        assert_eq!(pdf_name_eq(0, name1, name3), 0);

        // Non-name objects
        let int_obj = pdf_new_int(0, 42);
        assert_eq!(pdf_name_eq(0, name1, int_obj), 0);
        assert_eq!(pdf_name_eq(0, int_obj, int_obj), 0);
    }

    // ============================================================================
    // Type Checking Edge Cases
    // ============================================================================

    #[test]
    fn test_pdf_is_stream() {
        let dict = pdf_new_dict(0, 0, 10);
        assert_eq!(pdf_is_stream(0, dict), 0);

        // Note: We don't have pdf_new_stream, so we can't test positive case easily
    }

    #[test]
    fn test_type_checks_invalid_handle() {
        let invalid: PdfObjHandle = 99999;
        assert_eq!(pdf_is_null(0, invalid), 1); // Default is 1 for null check
        assert_eq!(pdf_is_bool(0, invalid), 0);
        assert_eq!(pdf_is_int(0, invalid), 0);
        assert_eq!(pdf_is_real(0, invalid), 0);
        assert_eq!(pdf_is_number(0, invalid), 0);
        assert_eq!(pdf_is_name(0, invalid), 0);
        assert_eq!(pdf_is_string(0, invalid), 0);
        assert_eq!(pdf_is_array(0, invalid), 0);
        assert_eq!(pdf_is_dict(0, invalid), 0);
        assert_eq!(pdf_is_indirect(0, invalid), 0);
        assert_eq!(pdf_is_stream(0, invalid), 0);
    }

    #[test]
    fn test_value_extraction_wrong_type() {
        let str_obj = pdf_new_string(0, b"test".as_ptr() as *const c_char, 4);

        assert_eq!(pdf_to_bool(0, str_obj), 0);
        assert_eq!(pdf_to_int(0, str_obj), 0);
        assert_eq!(pdf_to_int64(0, str_obj), 0);
        assert!((pdf_to_real(0, str_obj) - 0.0).abs() < 0.01);
        assert_eq!(pdf_to_num(0, str_obj), 0);
        assert_eq!(pdf_to_gen(0, str_obj), 0);
    }

    // ============================================================================
    // PdfObjType Tests
    // ============================================================================

    #[test]
    fn test_pdf_obj_type_shallow_eq() {
        // Same string values
        let s1 = PdfObjType::String(b"hello".to_vec());
        let s2 = PdfObjType::String(b"hello".to_vec());
        assert!(s1.shallow_eq(&s2));

        // Different string values
        let s3 = PdfObjType::String(b"world".to_vec());
        assert!(!s1.shallow_eq(&s3));

        // Arrays with same length
        let a1 = PdfObjType::Array(vec![PdfObj::new_int(1)]);
        let a2 = PdfObjType::Array(vec![PdfObj::new_int(2)]);
        assert!(a1.shallow_eq(&a2)); // Only checks length

        // Dicts with same length
        let d1 = PdfObjType::Dict(vec![("key".to_string(), PdfObj::new_int(1))]);
        let d2 = PdfObjType::Dict(vec![("other".to_string(), PdfObj::new_int(2))]);
        assert!(d1.shallow_eq(&d2)); // Only checks length

        // Indirect refs
        let i1 = PdfObjType::Indirect { num: 1, generation: 0 };
        let i2 = PdfObjType::Indirect { num: 1, generation: 0 };
        let i3 = PdfObjType::Indirect { num: 2, generation: 0 };
        assert!(i1.shallow_eq(&i2));
        assert!(!i1.shallow_eq(&i3));

        // Streams never match
        let st1 = PdfObjType::Stream { dict: Box::new(PdfObj::new_dict(0)), data: vec![] };
        let st2 = PdfObjType::Stream { dict: Box::new(PdfObj::new_dict(0)), data: vec![] };
        assert!(!st1.shallow_eq(&st2));

        // Different types
        let null = PdfObjType::Null;
        let int = PdfObjType::Int(42);
        assert!(!null.shallow_eq(&int));
    }

    #[test]
    fn test_pdf_obj_new_functions() {
        let null = PdfObj::new_null();
        assert!(matches!(null.obj_type, PdfObjType::Null));
        assert!(!null.marked);
        assert!(!null.dirty);
        assert_eq!(null.refs, 1);

        let arr = PdfObj::new_array(5);
        if let PdfObjType::Array(a) = &arr.obj_type {
            assert!(a.capacity() >= 5);
        } else {
            panic!("Expected array");
        }

        let dict = PdfObj::new_dict(3);
        if let PdfObjType::Dict(d) = &dict.obj_type {
            assert!(d.capacity() >= 3);
        } else {
            panic!("Expected dict");
        }

        let indirect = PdfObj::new_indirect(10, 2);
        if let PdfObjType::Indirect { num, generation } = &indirect.obj_type {
            assert_eq!(*num, 10);
            assert_eq!(*generation, 2);
        } else {
            panic!("Expected indirect");
        }
    }

    // ============================================================================
    // Array Get/Put Tests
    // ============================================================================

    #[test]
    fn test_pdf_array_get() {
        let arr = pdf_new_array(0, 0, 10);
        pdf_array_push_int(0, arr, 100);
        pdf_array_push_int(0, arr, 200);
        pdf_array_push_int(0, arr, 300);

        let elem0 = pdf_array_get(0, arr, 0);
        assert_eq!(pdf_to_int(0, elem0), 100);

        let elem1 = pdf_array_get(0, arr, 1);
        assert_eq!(pdf_to_int(0, elem1), 200);

        let elem2 = pdf_array_get(0, arr, 2);
        assert_eq!(pdf_to_int(0, elem2), 300);

        // Out of bounds
        let elem3 = pdf_array_get(0, arr, 10);
        assert_eq!(elem3, 0);
    }

    #[test]
    fn test_pdf_array_put() {
        let arr = pdf_new_array(0, 0, 10);
        pdf_array_push_int(0, arr, 100);
        pdf_array_push_int(0, arr, 200);

        let new_val = pdf_new_int(0, 999);
        pdf_array_put(0, arr, 0, new_val);

        let elem0 = pdf_array_get(0, arr, 0);
        assert_eq!(pdf_to_int(0, elem0), 999);

        // Put at end (append)
        let append_val = pdf_new_int(0, 300);
        pdf_array_put(0, arr, 2, append_val);
        assert_eq!(pdf_array_len(0, arr), 3);
    }

    #[test]
    fn test_pdf_array_insert() {
        let arr = pdf_new_array(0, 0, 10);
        pdf_array_push_int(0, arr, 100);
        pdf_array_push_int(0, arr, 300);

        let insert_val = pdf_new_int(0, 200);
        pdf_array_insert(0, arr, 1, insert_val);

        assert_eq!(pdf_array_len(0, arr), 3);
        let elem1 = pdf_array_get(0, arr, 1);
        assert_eq!(pdf_to_int(0, elem1), 200);
    }

    // ============================================================================
    // Dictionary Get/Put Tests
    // ============================================================================

    #[test]
    fn test_pdf_dict_get() {
        let dict = pdf_new_dict(0, 0, 10);
        let key = pdf_new_name(0, b"Type\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, dict, key, 42);

        let val = pdf_dict_get(0, dict, key);
        assert_eq!(pdf_to_int(0, val), 42);

        // Non-existent key
        let bad_key = pdf_new_name(0, b"NotExist\0".as_ptr() as *const c_char);
        let bad_val = pdf_dict_get(0, dict, bad_key);
        assert_eq!(bad_val, 0);
    }

    #[test]
    fn test_pdf_dict_gets() {
        let dict = pdf_new_dict(0, 0, 10);
        let key = pdf_new_name(0, b"Width\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, dict, key, 100);

        let val = pdf_dict_gets(0, dict, b"Width\0".as_ptr() as *const c_char);
        assert_eq!(pdf_to_int(0, val), 100);

        // Non-existent key
        let bad_val = pdf_dict_gets(0, dict, b"Height\0".as_ptr() as *const c_char);
        assert_eq!(bad_val, 0);

        // Null key
        let null_val = pdf_dict_gets(0, dict, std::ptr::null());
        assert_eq!(null_val, 0);
    }

    #[test]
    fn test_pdf_dict_put() {
        let dict = pdf_new_dict(0, 0, 10);
        let key = pdf_new_name(0, b"Value\0".as_ptr() as *const c_char);
        let val = pdf_new_int(0, 42);

        pdf_dict_put(0, dict, key, val);
        assert_eq!(pdf_dict_len(0, dict), 1);

        let retrieved = pdf_dict_get(0, dict, key);
        assert_eq!(pdf_to_int(0, retrieved), 42);
    }

    #[test]
    fn test_pdf_dict_put_name() {
        let dict = pdf_new_dict(0, 0, 10);
        let key = pdf_new_name(0, b"Type\0".as_ptr() as *const c_char);

        pdf_dict_put_name(0, dict, key, b"Page\0".as_ptr() as *const c_char);

        let val = pdf_dict_get(0, dict, key);
        assert_eq!(pdf_is_name(0, val), 1);
    }

    #[test]
    fn test_pdf_dict_get_key_val() {
        let dict = pdf_new_dict(0, 0, 10);
        let key = pdf_new_name(0, b"Type\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, dict, key, 42);

        let key0 = pdf_dict_get_key(0, dict, 0);
        assert_eq!(pdf_is_name(0, key0), 1);

        let val0 = pdf_dict_get_val(0, dict, 0);
        assert_eq!(pdf_to_int(0, val0), 42);

        // Out of bounds
        let key_bad = pdf_dict_get_key(0, dict, 10);
        assert_eq!(key_bad, 0);
    }

    // ============================================================================
    // String Extraction Tests
    // ============================================================================

    #[test]
    fn test_pdf_to_string() {
        let data = b"Hello PDF";
        let str_obj = pdf_new_string(0, data.as_ptr() as *const c_char, data.len());

        let mut size: usize = 0;
        let ptr = pdf_to_string(0, str_obj, &mut size as *mut usize);
        assert!(!ptr.is_null());
        assert_eq!(size, data.len());
    }

    #[test]
    fn test_pdf_to_str_len() {
        let data = b"Test String";
        let str_obj = pdf_new_string(0, data.as_ptr() as *const c_char, data.len());

        assert_eq!(pdf_to_str_len(0, str_obj), data.len());

        // Non-string returns 0
        let int_obj = pdf_new_int(0, 42);
        assert_eq!(pdf_to_str_len(0, int_obj), 0);
    }

    // ============================================================================
    // Copy Operations Tests
    // ============================================================================

    #[test]
    fn test_pdf_copy_array() {
        let arr = pdf_new_array(0, 0, 10);
        pdf_array_push_int(0, arr, 100);
        pdf_array_push_int(0, arr, 200);

        let copy = pdf_copy_array(0, 0, arr);
        assert_eq!(pdf_array_len(0, copy), 2);

        // Modify original, copy should be unchanged
        pdf_array_push_int(0, arr, 300);
        assert_eq!(pdf_array_len(0, arr), 3);
        assert_eq!(pdf_array_len(0, copy), 2);
    }

    #[test]
    fn test_pdf_copy_dict() {
        let dict = pdf_new_dict(0, 0, 10);
        let key = pdf_new_name(0, b"Key\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, dict, key, 42);

        let copy = pdf_copy_dict(0, 0, dict);
        assert_eq!(pdf_dict_len(0, copy), 1);

        // Modify original, copy should be unchanged
        let key2 = pdf_new_name(0, b"Key2\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, dict, key2, 100);
        assert_eq!(pdf_dict_len(0, dict), 2);
        assert_eq!(pdf_dict_len(0, copy), 1);
    }

    #[test]
    fn test_pdf_deep_copy_obj() {
        // Deep copy a nested structure
        let arr = pdf_new_array(0, 0, 10);
        let inner_dict = pdf_new_dict(0, 0, 5);
        let key = pdf_new_name(0, b"Inner\0".as_ptr() as *const c_char);
        pdf_dict_put_int(0, inner_dict, key, 99);
        pdf_array_push(0, arr, inner_dict);

        let copy = pdf_deep_copy_obj(0, 0, arr);
        assert_eq!(pdf_array_len(0, copy), 1);

        // The inner dict should also be copied
        let copied_inner = pdf_array_get(0, copy, 0);
        assert_eq!(pdf_is_dict(0, copied_inner), 1);
    }

    // ============================================================================
    // Geometry Object Creation Tests
    // ============================================================================

    #[test]
    fn test_pdf_new_point() {
        let point = pdf_new_point(0, 0, 10.0, 20.0);
        assert_eq!(pdf_is_array(0, point), 1);
        assert_eq!(pdf_array_len(0, point), 2);

        let x = pdf_array_get(0, point, 0);
        let y = pdf_array_get(0, point, 1);
        assert!((pdf_to_real(0, x) - 10.0).abs() < 0.01);
        assert!((pdf_to_real(0, y) - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_pdf_new_rect() {
        let rect = pdf_new_rect(0, 0, 0.0, 0.0, 100.0, 200.0);
        assert_eq!(pdf_is_array(0, rect), 1);
        assert_eq!(pdf_array_len(0, rect), 4);

        let x0 = pdf_array_get(0, rect, 0);
        let y0 = pdf_array_get(0, rect, 1);
        let x1 = pdf_array_get(0, rect, 2);
        let y1 = pdf_array_get(0, rect, 3);

        assert!((pdf_to_real(0, x0) - 0.0).abs() < 0.01);
        assert!((pdf_to_real(0, y0) - 0.0).abs() < 0.01);
        assert!((pdf_to_real(0, x1) - 100.0).abs() < 0.01);
        assert!((pdf_to_real(0, y1) - 200.0).abs() < 0.01);
    }

    #[test]
    fn test_pdf_new_matrix() {
        let matrix = pdf_new_matrix(0, 0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(pdf_is_array(0, matrix), 1);
        assert_eq!(pdf_array_len(0, matrix), 6);

        let a = pdf_array_get(0, matrix, 0);
        let d = pdf_array_get(0, matrix, 3);
        assert!((pdf_to_real(0, a) - 1.0).abs() < 0.01);
        assert!((pdf_to_real(0, d) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_pdf_new_date() {
        let date = pdf_new_date(0, 0, 2024, 12, 3, 15, 30, 45);
        assert_eq!(pdf_is_string(0, date), 1);
        assert!(pdf_to_str_len(0, date) > 0);
    }

    // ============================================================================
    // Array Push Name/String Tests
    // ============================================================================

    #[test]
    fn test_pdf_array_push_name() {
        let arr = pdf_new_array(0, 0, 10);
        pdf_array_push_name(0, arr, b"Type\0".as_ptr() as *const c_char);

        assert_eq!(pdf_array_len(0, arr), 1);
        let elem = pdf_array_get(0, arr, 0);
        assert_eq!(pdf_is_name(0, elem), 1);
    }

    #[test]
    fn test_pdf_array_push_string() {
        let arr = pdf_new_array(0, 0, 10);
        let data = b"Hello";
        pdf_array_push_string(0, arr, data.as_ptr() as *const c_char, data.len());

        assert_eq!(pdf_array_len(0, arr), 1);
        let elem = pdf_array_get(0, arr, 0);
        assert_eq!(pdf_is_string(0, elem), 1);
        assert_eq!(pdf_to_str_len(0, elem), 5);
    }
}

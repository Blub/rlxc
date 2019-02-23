use std::ffi::CString;
use std::os::raw::c_char;

/// Helper to create a C string array (`char**`) variable with the ownership
/// still in rust code. The raw version of this will contain a trailing `NULL`
/// pointer.
#[derive(Debug, Default)]
pub struct CStringVec {
    owned: Vec<CString>,
    ffi: Vec<*const c_char>,
}

impl CStringVec {
    /// Create a new empty vector.
    pub fn new() -> Self {
        Self {
            owned: Vec::new(),
            ffi: Vec::new(),
        }
    }

    /// Update the inner `ffi` vector.
    fn update(&mut self) {
        self.ffi.truncate(0);
        self.ffi.reserve(self.owned.len() + 1);
        for cstr in self.owned.iter() {
            self.ffi.push(cstr.as_ptr());
        }
        self.ffi.push(std::ptr::null());
    }

    /// Get a reference to the ffi vector. We return a reference to the `Vec`
    /// type instead of returning a `*const *const c_char` to explicitly show
    /// that this borrows `self`!
    pub fn get_raw<'a>(&'a mut self) -> &'a Vec<*const c_char> {
        self.update();
        &self.ffi
    }

    //pub fn into_inner(self) -> Vec<CString> {
    //    self.owned
    //}
}

// Implement `Deref<Vec<CString>>` so we can use this type exactly as if
// it actually were just the inner `Vec<CString>`.
impl std::ops::Deref for CStringVec {
    type Target = Vec<CString>;

    fn deref(&self) -> &Self::Target {
        &self.owned
    }
}

impl std::ops::DerefMut for CStringVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.owned
    }
}

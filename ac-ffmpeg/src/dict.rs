use std::{
    ffi::{c_char, c_void, CStr},
    marker::PhantomData,
};

extern "C" {
    fn ffw_dict_advance_iterator(dict: *const c_void, prev: *const c_void) -> *const c_void;
    fn ffw_dict_entry_get_key(entry: *const c_void) -> *const c_char;
    fn ffw_dict_entry_get_value(entry: *const c_void) -> *const c_char;
}

/// The element of a [Dict], a key-value pair.
pub struct DictEntry<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

/// Iterator over the entries of an ffmpeg dictionary.
///
/// # Warning
///
/// Do not mutate the dictionary while iterating over it.
pub struct Dict<'a> {
    /// A pointer to the dictionary.
    dict: *const c_void,
    /// A pointer to the previous dictionary entry, or null if there is none.
    prev: *const c_void,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Dict<'a> {
    /// Create a new dictionary iterator.
    ///
    /// # Panics
    ///
    /// This will panic if the pointer to the dictionary is null.
    pub fn new(dict: *const c_void) -> Dict<'a> {
        assert!(!dict.is_null());

        Self {
            dict,
            prev: std::ptr::null_mut::<c_void>(),
            phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for Dict<'a> {
    type Item = DictEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.prev = ffw_dict_advance_iterator(self.dict, self.prev);

            if !self.prev.is_null() {
                let key =
                    CStr::from_ptr(ffw_dict_entry_get_key(self.prev) as _)
                        .to_str()
                        .expect("Expected valid UTF-8 in dictionary entry.");

                let value =
                    CStr::from_ptr(ffw_dict_entry_get_value(self.prev) as _)
                        .to_str()
                        .expect("Expected valid UTF-8 in dictionary entry.");

                Some(DictEntry { key, value })
            } else {
                None
            }
        }
    }
}

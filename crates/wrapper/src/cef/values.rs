use crate::{
    bindings::{
        cef_binary_value_create, cef_binary_value_t, cef_dictionary_value_create,
        cef_dictionary_value_t, cef_list_value_create, cef_list_value_t, cef_value_create,
        cef_value_t, cef_value_type_t
    },
    ref_counted_ptr, CefString, CefStringList
};
use std::{
    ffi::{c_int, c_void},
    ptr::null_mut
};

/// Supported value types.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueType {
    Invalid,
    Null,
    Bool,
    Int,
    Double,
    String,
    Binary,
    Dictionary,
    List
}

impl From<cef_value_type_t> for ValueType {
    fn from(value: cef_value_type_t) -> Self {
        match value {
            cef_value_type_t::VTYPE_INVALID => Self::Invalid,
            cef_value_type_t::VTYPE_NULL => Self::Null,
            cef_value_type_t::VTYPE_BOOL => Self::Bool,
            cef_value_type_t::VTYPE_INT => Self::Int,
            cef_value_type_t::VTYPE_DOUBLE => Self::Double,
            cef_value_type_t::VTYPE_STRING => Self::String,
            cef_value_type_t::VTYPE_BINARY => Self::Binary,
            cef_value_type_t::VTYPE_DICTIONARY => Self::Dictionary,
            cef_value_type_t::VTYPE_LIST => Self::List
        }
    }
}

impl From<ValueType> for cef_value_type_t {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Invalid => Self::VTYPE_INVALID,
            ValueType::Null => Self::VTYPE_NULL,
            ValueType::Bool => Self::VTYPE_BOOL,
            ValueType::Int => Self::VTYPE_INT,
            ValueType::Double => Self::VTYPE_DOUBLE,
            ValueType::String => Self::VTYPE_STRING,
            ValueType::Binary => Self::VTYPE_BINARY,
            ValueType::Dictionary => Self::VTYPE_DICTIONARY,
            ValueType::List => Self::VTYPE_LIST
        }
    }
}

// Structure that wraps other data value types. Complex types (binary,
// dictionary and list) will be referenced but not owned by this object. Can be
// used on any process and thread.
ref_counted_ptr!(Value, cef_value_t);

impl Value {
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_value_create()) }
    }

    /// Returns true (1) if the underlying data is valid. This will always be true
    /// (1) for simple types. For complex types (binary, dictionary and list) the
    /// underlying data may become invalid if owned by another object (e.g. list
    /// or dictionary) and that other object is then modified or destroyed. This
    /// value object can be re-used by calling Set*() even if the underlying data
    /// is invalid.
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if the underlying data is owned by another object.
    pub fn is_owned(&self) -> bool {
        self.0
            .is_owned
            .map(|is_owned| unsafe { is_owned(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if the underlying data is read-only. Some APIs may expose
    /// read-only objects.
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) != 0 })
            .unwrap_or(true)
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data. If true (1) modifications to this object will also affect |that|
    /// object and vice-versa.
    pub fn is_same(&self, that: Value) -> bool {
        self.0
            .is_same
            .map(|is_same| unsafe { is_same(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: Value) -> bool {
        self.0
            .is_equal
            .map(|is_equal| unsafe { is_equal(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns a copy of this object. The underlying data will also be copied.
    pub fn copy(&self) -> Option<Value> {
        self.0
            .copy
            .map(|copy| unsafe { Self::from_ptr_unchecked(copy(self.as_ptr())) })
    }

    /// Returns the underlying value type.
    pub fn get_type(&self) -> ValueType {
        self.0
            .get_type
            .map(|get_type| unsafe { get_type(self.as_ptr()) }.into())
            .unwrap_or(ValueType::Invalid)
    }

    /// Returns the underlying value as type bool.
    pub fn get_bool(&self) -> Option<bool> {
        self.0
            .get_bool
            .map(|get_bool| unsafe { get_bool(self.as_ptr()) != 0 })
    }

    /// Returns the underlying value as type int.
    pub fn get_int(&self) -> Option<i32> {
        self.0
            .get_int
            .map(|get_int| unsafe { get_int(self.as_ptr()) as i32 })
    }

    /// Returns the underlying value as type double.
    pub fn get_double(&self) -> Option<f64> {
        self.0
            .get_double
            .map(|get_double| unsafe { get_double(self.as_ptr()) })
    }

    /// Returns the underlying value as type string.
    pub fn get_string(&self) -> Option<String> {
        self.0
            .get_string
            .and_then(|get_string| {
                let s = unsafe { get_string(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Returns the underlying value as type binary. The returned reference may
    /// become invalid if the value is owned by another object or if ownership is
    /// transferred to another object in the future. To maintain a reference to
    /// the value after assigning ownership to a dictionary or list pass this
    /// object to the set_value() function instead of passing the returned
    /// reference to set_binary().
    pub fn get_binary(&self) -> Option<BinaryValue> {
        self.0
            .get_binary
            .and_then(|get_binary| unsafe { BinaryValue::from_ptr(get_binary(self.as_ptr())) })
    }

    /// Returns the underlying value as type dictionary. The returned reference
    /// may become invalid if the value is owned by another object or if ownership
    /// is transferred to another object in the future. To maintain a reference to
    /// the value after assigning ownership to a dictionary or list pass this
    /// object to the set_value() function instead of passing the returned
    /// reference to set_dictionary().
    pub fn get_dictionary(&self) -> Option<DictionaryValue> {
        self.0
            .get_dictionary
            .and_then(|get_dictionary| unsafe {
                DictionaryValue::from_ptr(get_dictionary(self.as_ptr()))
            })
    }

    /// Returns the underlying value as type list. The returned reference may
    /// become invalid if the value is owned by another object or if ownership is
    /// transferred to another object in the future. To maintain a reference to
    /// the value after assigning ownership to a dictionary or list pass this
    /// object to the set_value() function instead of passing the returned
    /// reference to set_list().
    pub fn get_list(&self) -> Option<ListValue> {
        self.0
            .get_list
            .and_then(|get_list| unsafe { ListValue::from_ptr(get_list(self.as_ptr())) })
    }

    /// Sets the underlying value as type null. Returns true (1) if the value was
    /// set successfully.
    pub fn set_null(&self) -> bool {
        self.0
            .set_null
            .map(|set_null| unsafe { set_null(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Sets the underlying value as type bool. Returns true (1) if the value was
    /// set successfully.
    pub fn set_bool(&self, value: bool) -> bool {
        self.0
            .set_bool
            .map(|set_bool| unsafe { set_bool(self.as_ptr(), if value { 1 } else { 0 }) != 0 })
            .unwrap_or(false)
    }

    /// Sets the underlying value as type int. Returns true (1) if the value was
    /// set successfully.
    pub fn set_int(&self, value: i32) -> bool {
        self.0
            .set_int
            .map(|set_int| unsafe { set_int(self.as_ptr(), value as c_int) != 0 })
            .unwrap_or(false)
    }

    /// Sets the underlying value as type double. Returns true (1) if the value
    /// was set successfully.
    pub fn set_double(&self, value: f64) -> bool {
        self.0
            .set_double
            .map(|set_double| unsafe { set_double(self.as_ptr(), value) != 0 })
            .unwrap_or(false)
    }

    /// Sets the underlying value as type string. Returns true (1) if the value
    /// was set successfully.
    pub fn set_string(&self, value: &str) -> bool {
        self.0
            .set_string
            .map(|set_string| unsafe {
                let value = CefString::new(value);

                set_string(self.as_ptr(), value.as_ptr()) != 0
            })
            .unwrap_or(false)
    }

    /// Sets the underlying value as type binary. Returns true (1) if the value
    /// was set successfully. This object keeps a reference to |value| and
    /// ownership of the underlying data remains unchanged.
    pub fn set_binary(&self, value: BinaryValue) -> bool {
        self.0
            .set_binary
            .map(|set_binary| unsafe { set_binary(self.as_ptr(), value.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Sets the underlying value as type dict. Returns true (1) if the value was
    /// set successfully. This object keeps a reference to |value| and ownership
    /// of the underlying data remains unchanged.
    pub fn set_dictionary(&self, value: DictionaryValue) -> bool {
        self.0
            .set_dictionary
            .map(|set_dictionary| unsafe { set_dictionary(self.as_ptr(), value.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Sets the underlying value as type list. Returns true (1) if the value was
    /// set successfully. This object keeps a reference to |value| and ownership
    /// of the underlying data remains unchanged.
    pub fn set_list(&self, value: ListValue) -> bool {
        self.0
            .set_list
            .map(|set_list| unsafe { set_list(self.as_ptr(), value.into_raw()) != 0 })
            .unwrap_or(false)
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
    }
}

// Structure representing a binary value. Can be used on any process and
// thread.
ref_counted_ptr!(BinaryValue, cef_binary_value_t);

impl BinaryValue {
    pub fn new(data: &[u8]) -> Self {
        unsafe {
            Self::from_ptr_unchecked(cef_binary_value_create(
                data.as_ptr() as *const c_void,
                data.len()
            ))
        }
    }

    /// Returns true (1) if this object is valid. This object may become invalid
    /// if the underlying data is owned by another object (e.g. list or
    /// dictionary) and that other object is then modified or destroyed. Do not
    /// call any other functions if this function returns false (0).
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object is currently owned by another object.
    pub fn is_owned(&self) -> bool {
        self.0
            .is_owned
            .map(|is_owned| unsafe { is_owned(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data.
    pub fn is_same(&self, that: BinaryValue) -> bool {
        self.0
            .is_same
            .map(|is_same| unsafe { is_same(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: BinaryValue) -> bool {
        self.0
            .is_equal
            .map(|is_equal| unsafe { is_equal(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns a copy of this object. The data in this object will also be
    /// copied.
    pub fn copy(&self) -> Option<BinaryValue> {
        self.0
            .copy
            .map(|copy| unsafe { Self::from_ptr_unchecked(copy(self.as_ptr())) })
    }

    /// Returns a pointer to the beginning of the memory block. The returned
    /// pointer is valid as long as the cef_binary_value_t is alive.
    pub fn get_raw_data(&self) -> *const u8 {
        self.0
            .get_raw_data
            .map(|get_raw_data| unsafe { get_raw_data(self.as_ptr()) as *const u8 })
            .unwrap_or(null_mut())
    }

    /// Returns the data size.
    pub fn get_size(&self) -> usize {
        self.0
            .get_size
            .map(|get_size| unsafe { get_size(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Read up to |buffer_size| number of bytes into |buffer|. Reading begins at
    /// the specified byte |data_offset|. Returns the number of bytes read.
    pub fn get_data(&self, buffer: &mut [u8], offset: usize) -> usize {
        self.0
            .get_data
            .map(|get_data| unsafe {
                get_data(
                    self.as_ptr(),
                    buffer.as_mut_ptr() as *mut c_void,
                    buffer.len(),
                    offset
                )
            })
            .unwrap_or(0)
    }
}

impl PartialEq for BinaryValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
    }
}

// Structure representing a dictionary value. Can be used on any process and
// thread.
ref_counted_ptr!(DictionaryValue, cef_dictionary_value_t);

impl DictionaryValue {
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_dictionary_value_create()) }
    }

    /// Returns true (1) if this object is valid. This object may become invalid
    /// if the underlying data is owned by another object (e.g. list or
    /// dictionary) and that other object is then modified or destroyed. Do not
    /// call any other functions if this function returns false (0).
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object is currently owned by another object.
    pub fn is_owned(&self) -> bool {
        self.0
            .is_owned
            .map(|is_owned| unsafe { is_owned(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) != 0 })
            .unwrap_or(true)
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data. If true (1) modifications to this object will also affect |that|
    /// object and vice-versa.
    pub fn is_same(&self, that: DictionaryValue) -> bool {
        self.0
            .is_same
            .map(|is_same| unsafe { is_same(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: DictionaryValue) -> bool {
        self.0
            .is_equal
            .map(|is_equal| unsafe { is_equal(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns a writable copy of this object. If |exclude_NULL_children| is true
    /// (1) any NULL dictionaries or lists will be excluded from the copy.
    pub fn copy(&self, exclude_empty_children: bool) -> Option<DictionaryValue> {
        self.0.copy.map(|copy| unsafe {
            Self::from_ptr_unchecked(copy(
                self.as_ptr(),
                if exclude_empty_children { 1 } else { 0 }
            ))
        })
    }

    /// Returns the number of values.
    pub fn len(&self) -> usize {
        self.0
            .get_size
            .map(|get_size| unsafe { get_size(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Removes all values. Returns true (1) on success.
    pub fn clear(&self) -> bool {
        self.0
            .clear
            .map(|clear| unsafe { clear(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if the current dictionary has a value for the given key.
    pub fn has_key(&self, key: &str) -> bool {
        self.0
            .has_key
            .map(|has_key| unsafe {
                let key = CefString::new(key);

                has_key(self.as_ptr(), key.as_ptr()) != 0
            })
            .unwrap_or(false)
    }

    /// Reads all keys for this dictionary into the specified vector.
    pub fn get_keys(&self) -> Vec<String> {
        self.0
            .get_keys
            .map(|get_keys| {
                let mut list = CefStringList::new();

                unsafe {
                    get_keys(self.as_ptr(), list.as_mut_ptr());
                }

                list.into()
            })
            .unwrap_or_default()
    }

    /// Removes the value at the specified key. Returns true (1) is the value was
    /// removed successfully.
    pub fn remove(&self, key: &str) -> bool {
        self.0
            .remove
            .map(|remove| unsafe {
                let key = CefString::new(key);

                remove(self.as_ptr(), key.as_ptr()) != 0
            })
            .unwrap_or(false)
    }

    /// Returns the value type for the specified key.
    pub fn get_type(&self, key: &str) -> ValueType {
        self.0
            .get_type
            .map(|get_type| unsafe {
                let key = CefString::new(key);

                get_type(self.as_ptr(), key.as_ptr()).into()
            })
            .unwrap_or(ValueType::Invalid)
    }

    /// Returns the value at the specified key. For simple types the returned
    /// value will copy existing data and modifications to the value will not
    /// modify this object. For complex types (binary, dictionary and list) the
    /// returned value will reference existing data and modifications to the value
    /// will modify this object.
    pub fn get_value(&self, key: &str) -> Option<Value> {
        self.0
            .get_value
            .and_then(|get_value| {
                let key = CefString::new(key);

                unsafe { Value::from_ptr(get_value(self.as_ptr(), key.as_ptr())) }
            })
    }

    /// Returns the value at the specified key as type bool.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.0.get_bool.map(|get_bool| {
            let key = CefString::new(key);

            unsafe { get_bool(self.as_ptr(), key.as_ptr()) != 0 }
        })
    }

    /// Returns the value at the specified key as type int.
    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.0.get_int.map(|get_int| {
            let key = CefString::new(key);

            unsafe { get_int(self.as_ptr(), key.as_ptr()) as i32 }
        })
    }

    /// Returns the value at the specified key as type double.
    pub fn get_double(&self, key: &str) -> Option<f64> {
        self.0.get_double.map(|get_double| {
            let key = CefString::new(key);

            unsafe { get_double(self.as_ptr(), key.as_ptr()) }
        })
    }

    /// Returns the value at the specified key as type string.
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.0
            .get_string
            .and_then(|get_string| {
                let key = CefString::new(key);
                let s = unsafe { get_string(self.as_ptr(), key.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Returns the value at the specified key as type binary. The returned value
    /// will reference existing data.
    pub fn get_binary(&self, key: &str) -> Option<BinaryValue> {
        self.0
            .get_binary
            .and_then(|get_binary| {
                let key = CefString::new(key);

                unsafe { BinaryValue::from_ptr(get_binary(self.as_ptr(), key.as_ptr())) }
            })
    }

    /// Returns the value at the specified key as type dictionary. The returned
    /// value will reference existing data and modifications to the value will
    /// modify this object.
    pub fn get_dictionary(&self, key: &str) -> Option<DictionaryValue> {
        self.0
            .get_dictionary
            .and_then(|get_dictionary| {
                let key = CefString::new(key);

                unsafe { DictionaryValue::from_ptr(get_dictionary(self.as_ptr(), key.as_ptr())) }
            })
    }

    /// Returns the value at the specified key as type list. The returned value
    /// will reference existing data and modifications to the value will modify
    /// this object.
    pub fn get_list(&self, key: &str) -> Option<ListValue> {
        self.0
            .get_list
            .and_then(|get_list| {
                let key = CefString::new(key);

                unsafe { ListValue::from_ptr(get_list(self.as_ptr(), key.as_ptr())) }
            })
    }

    // /// Sets the value at the specified key. Returns true (1) if the value was set
    // /// successfully. If |value| represents simple data then the underlying data
    // /// will be copied and modifications to |value| will not modify this object.
    // /// If |value| represents complex data (binary, dictionary or list) then the
    // /// underlying data will be referenced and modifications to |value| will
    // /// modify this object.
    pub fn set_value(&self, key: &str, value: Value) -> bool {
        self.0
            .set_value
            .map(|set_value| {
                let key = CefString::new(key);

                unsafe { set_value(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type null. Returns true (1) if the
    /// value was set successfully.
    pub fn set_null(&self, key: &str) -> bool {
        self.0
            .set_null
            .map(|set_null| {
                let key = CefString::new(key);

                unsafe { set_null(self.as_ptr(), key.as_ptr()) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type bool. Returns true (1) if the
    /// value was set successfully.
    pub fn set_bool(&self, key: &str, value: bool) -> bool {
        self.0
            .set_bool
            .map(|set_bool| {
                let key = CefString::new(key);

                unsafe { set_bool(self.as_ptr(), key.as_ptr(), if value { 1 } else { 0 }) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type int. Returns true (1) if the
    /// value was set successfully.
    pub fn set_int(&self, key: &str, value: i32) -> bool {
        self.0
            .set_int
            .map(|set_int| {
                let key = CefString::new(key);

                unsafe { set_int(self.as_ptr(), key.as_ptr(), value as c_int) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type double. Returns true (1) if
    /// the value was set successfully.
    pub fn set_double(&self, key: &str, value: f64) -> bool {
        self.0
            .set_double
            .map(|set_double| {
                let key = CefString::new(key);

                unsafe { set_double(self.as_ptr(), key.as_ptr(), value) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type string. Returns true (1) if
    /// the value was set successfully.
    pub fn set_string(&self, key: &str, value: &str) -> bool {
        self.0
            .set_string
            .map(|set_string| {
                let key = CefString::new(key);
                let value = CefString::new(value);

                unsafe { set_string(self.as_ptr(), key.as_ptr(), value.as_ptr()) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type binary. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_binary(&self, key: &str, value: BinaryValue) -> bool {
        self.0
            .set_binary
            .map(|set_binary| {
                let key = CefString::new(key);

                unsafe { set_binary(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type dict. Returns true (1) if the
    /// value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_dictionary(&self, key: &str, value: DictionaryValue) -> bool {
        self.0
            .set_dictionary
            .map(|set_dictionary| {
                let key = CefString::new(key);

                unsafe { set_dictionary(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified key as type list. Returns true (1) if the
    /// value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_list(&self, key: &str, value: ListValue) -> bool {
        self.0
            .set_list
            .map(|set_list| {
                let key = CefString::new(key);

                unsafe { set_list(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0 }
            })
            .unwrap_or(false)
    }
}

impl PartialEq for DictionaryValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
    }
}

// Structure representing a list value. Can be used on any process and thread.
ref_counted_ptr!(ListValue, cef_list_value_t);

impl ListValue {
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_list_value_create()) }
    }

    /// Returns true (1) if this object is valid. This object may become invalid
    /// if the underlying data is owned by another object (e.g. list or
    /// dictionary) and that other object is then modified or destroyed. Do not
    /// call any other functions if this function returns false (0).
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object is currently owned by another object.
    pub fn is_owned(&self) -> bool {
        self.0
            .is_owned
            .map(|is_owned| unsafe { is_owned(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) != 0 })
            .unwrap_or(true)
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data. If true (1) modifications to this object will also affect |that|
    /// object and vice-versa.
    pub fn is_same(&self, that: ListValue) -> bool {
        self.0
            .is_same
            .map(|is_same| unsafe { is_same(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: ListValue) -> bool {
        self.0
            .is_equal
            .map(|is_equal| unsafe { is_equal(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns a writable copy of this object.
    pub fn copy(&self) -> Option<ListValue> {
        self.0
            .copy
            .map(|copy| unsafe { Self::from_ptr_unchecked(copy(self.as_ptr())) })
    }

    /// Sets the number of values. If the number of values is expanded all new
    /// value slots will default to type null. Returns true (1) on success.
    pub fn set_size(&self, size: usize) -> bool {
        self.0
            .set_size
            .map(|set_size| unsafe { set_size(self.as_ptr(), size) != 0 })
            .unwrap_or(false)
    }

    /// Returns the number of values.
    pub fn len(&self) -> usize {
        self.0
            .get_size
            .map(|get_size| unsafe { get_size(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Removes all values. Returns true (1) on success.
    pub fn clear(&self) -> bool {
        self.0
            .clear
            .map(|clear| unsafe { clear(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Removes the value at the specified index.
    pub fn remove(&self, index: usize) -> bool {
        self.0
            .remove
            .map(|remove| unsafe { remove(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
    }

    /// Returns the value type at the specified index.
    pub fn get_type(&self, index: usize) -> ValueType {
        self.0
            .get_type
            .map(|get_type| unsafe { get_type(self.as_ptr(), index) }.into())
            .unwrap_or(ValueType::Invalid)
    }

    /// Returns the value at the specified index. For simple types the returned
    /// value will copy existing data and modifications to the value will not
    /// modify this object. For complex types (binary, dictionary and list) the
    /// returned value will reference existing data and modifications to the value
    /// will modify this object.
    pub fn get_value(&self, index: usize) -> Option<Value> {
        self.0
            .get_value
            .and_then(|get_value| unsafe { Value::from_ptr(get_value(self.as_ptr(), index)) })
    }

    /// Returns the value at the specified index as type bool.
    pub fn get_bool(&self, index: usize) -> Option<bool> {
        self.0
            .get_bool
            .map(|get_bool| unsafe { get_bool(self.as_ptr(), index) != 0 })
    }

    /// Returns the value at the specified index as type int.
    pub fn get_int(&self, index: usize) -> Option<i32> {
        self.0
            .get_int
            .map(|get_int| unsafe { get_int(self.as_ptr(), index) as i32 })
    }

    /// Returns the value at the specified index as type double.
    pub fn get_double(&self, index: usize) -> Option<f64> {
        self.0
            .get_double
            .map(|get_double| unsafe { get_double(self.as_ptr(), index) })
    }

    /// Returns the value at the specified index as type string.
    pub fn get_string(&self, index: usize) -> Option<String> {
        self.0
            .get_string
            .and_then(|get_string| {
                let s = unsafe { get_string(self.as_ptr(), index) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Returns the value at the specified index as type binary. The returned
    /// value will reference existing data.
    pub fn get_binary(&self, index: usize) -> Option<BinaryValue> {
        self.0
            .get_binary
            .and_then(|get_binary| unsafe {
                BinaryValue::from_ptr(get_binary(self.as_ptr(), index))
            })
    }

    /// Returns the value at the specified index as type dictionary. The returned
    /// value will reference existing data and modifications to the value will
    /// modify this object.
    pub fn get_dictionary(&self, index: usize) -> Option<DictionaryValue> {
        self.0
            .get_dictionary
            .and_then(|get_dictionary| unsafe {
                DictionaryValue::from_ptr(get_dictionary(self.as_ptr(), index))
            })
    }

    /// Returns the value at the specified index as type list. The returned value
    /// will reference existing data and modifications to the value will modify
    /// this object.
    pub fn get_list(&self, index: usize) -> Option<ListValue> {
        self.0
            .get_list
            .and_then(|get_list| unsafe { ListValue::from_ptr(get_list(self.as_ptr(), index)) })
    }

    /// Sets the value at the specified index. Returns true (1) if the value was
    /// set successfully. If |value| represents simple data then the underlying
    /// data will be copied and modifications to |value| will not modify this
    /// object. If |value| represents complex data (binary, dictionary or list)
    /// then the underlying data will be referenced and modifications to |value|
    /// will modify this object.
    pub fn set_value(&self, index: usize, value: Value) -> bool {
        self.0
            .set_value
            .map(|set_value| unsafe { set_value(self.as_ptr(), index, value.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type null. Returns true (1) if
    /// the value was set successfully.
    pub fn set_null(&self, index: usize) -> bool {
        self.0
            .set_null
            .map(|set_null| unsafe { set_null(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type bool. Returns true (1) if
    /// the value was set successfully.
    pub fn set_bool(&self, index: usize, value: bool) -> bool {
        self.0
            .set_bool
            .map(|set_bool| unsafe {
                set_bool(self.as_ptr(), index, if value { 1 } else { 0 }) != 0
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type int. Returns true (1) if the
    /// value was set successfully.
    pub fn set_int(&self, index: usize, value: i32) -> bool {
        self.0
            .set_int
            .map(|set_int| unsafe { set_int(self.as_ptr(), index, value as c_int) != 0 })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type double. Returns true (1) if
    /// the value was set successfully.
    pub fn set_double(&self, index: usize, value: f64) -> bool {
        self.0
            .set_double
            .map(|set_double| unsafe { set_double(self.as_ptr(), index, value) != 0 })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type string. Returns true (1) if
    /// the value was set successfully.
    pub fn set_string(&self, index: usize, value: &str) -> bool {
        self.0
            .set_string
            .map(|set_string| {
                let value = CefString::new(value);

                unsafe { set_string(self.as_ptr(), index, value.as_ptr()) != 0 }
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type binary. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_binary(&self, index: usize, value: BinaryValue) -> bool {
        self.0
            .set_binary
            .map(|set_binary| unsafe { set_binary(self.as_ptr(), index, value.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type dict. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_dictionary(&self, index: usize, value: DictionaryValue) -> bool {
        self.0
            .set_dictionary
            .map(|set_dictionary| unsafe {
                set_dictionary(self.as_ptr(), index, value.into_raw()) != 0
            })
            .unwrap_or(false)
    }

    /// Sets the value at the specified index as type list. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_list(&self, index: usize, value: ListValue) -> bool {
        self.0
            .set_list
            .map(|set_list| unsafe { set_list(self.as_ptr(), index, value.into_raw()) != 0 })
            .unwrap_or(false)
    }
}

impl PartialEq for ListValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
    }
}

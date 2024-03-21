use crate::{ref_counted_ptr, try_c, CefString, CefStringList};
use anyhow::Result;
use bindings::{
    cef_binary_value_create, cef_binary_value_t, cef_dictionary_value_create,
    cef_dictionary_value_t, cef_list_value_create, cef_list_value_t, cef_value_create, cef_value_t,
    cef_value_type_t
};
use std::ffi::{c_int, c_void};

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
    /// Creates a new object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_value_create()) }
    }

    /// Returns true (1) if the underlying data is valid. This will always be true
    /// (1) for simple types. For complex types (binary, dictionary and list) the
    /// underlying data may become invalid if owned by another object (e.g. list
    /// or dictionary) and that other object is then modified or destroyed. This
    /// value object can be re-used by calling Set*() even if the underlying data
    /// is invalid.
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the underlying data is owned by another object.
    pub fn is_owned(&self) -> Result<bool> {
        try_c!(self, is_owned, { Ok(is_owned(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the underlying data is read-only. Some APIs may expose
    /// read-only objects.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data. If true (1) modifications to this object will also affect |that|
    /// object and vice-versa.
    pub fn is_same(&self, that: Value) -> Result<bool> {
        try_c!(self, is_same, {
            Ok(is_same(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: Value) -> Result<bool> {
        try_c!(self, is_equal, {
            Ok(is_equal(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns a copy of this object. The underlying data will also be copied.
    pub fn copy(&self) -> Result<Value> {
        try_c!(self, copy, {
            Ok(Value::from_ptr_unchecked(copy(self.as_ptr())))
        })
    }

    /// Returns the underlying value type.
    pub fn get_type(&self) -> Result<ValueType> {
        try_c!(self, get_type, { Ok(get_type(self.as_ptr()).into()) })
    }

    /// Returns the underlying value as type bool.
    pub fn get_bool(&self) -> Result<bool> {
        try_c!(self, get_bool, { Ok(get_bool(self.as_ptr()) != 0) })
    }

    /// Returns the underlying value as type int.
    pub fn get_int(&self) -> Result<i32> {
        try_c!(self, get_int, { Ok(get_int(self.as_ptr()) as i32) })
    }

    /// Returns the underlying value as type double.
    pub fn get_double(&self) -> Result<f64> {
        try_c!(self, get_double, { Ok(get_double(self.as_ptr())) })
    }

    /// Returns the underlying value as type string.
    pub fn get_string(&self) -> Result<Option<String>> {
        try_c!(self, get_string, {
            let s = get_string(self.as_ptr());

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Returns the underlying value as type binary. The returned reference may
    /// become invalid if the value is owned by another object or if ownership is
    /// transferred to another object in the future. To maintain a reference to
    /// the value after assigning ownership to a dictionary or list pass this
    /// object to the set_value() function instead of passing the returned
    /// reference to set_binary().
    pub fn get_binary(&self) -> Result<Option<BinaryValue>> {
        try_c!(self, get_binary, {
            Ok(BinaryValue::from_ptr(get_binary(self.as_ptr())))
        })
    }

    /// Returns the underlying value as type dictionary. The returned reference
    /// may become invalid if the value is owned by another object or if ownership
    /// is transferred to another object in the future. To maintain a reference to
    /// the value after assigning ownership to a dictionary or list pass this
    /// object to the set_value() function instead of passing the returned
    /// reference to set_dictionary().
    pub fn get_dictionary(&self) -> Result<Option<DictionaryValue>> {
        try_c!(self, get_dictionary, {
            Ok(DictionaryValue::from_ptr(get_dictionary(self.as_ptr())))
        })
    }

    /// Returns the underlying value as type list. The returned reference may
    /// become invalid if the value is owned by another object or if ownership is
    /// transferred to another object in the future. To maintain a reference to
    /// the value after assigning ownership to a dictionary or list pass this
    /// object to the set_value() function instead of passing the returned
    /// reference to set_list().
    pub fn get_list(&self) -> Result<Option<ListValue>> {
        try_c!(self, get_list, {
            Ok(ListValue::from_ptr(get_list(self.as_ptr())))
        })
    }

    /// Sets the underlying value as type null. Returns true (1) if the value was
    /// set successfully.
    pub fn set_null(&self) -> Result<bool> {
        try_c!(self, set_null, { Ok(set_null(self.as_ptr()) != 0) })
    }

    /// Sets the underlying value as type bool. Returns true (1) if the value was
    /// set successfully.
    pub fn set_bool(&self, value: bool) -> Result<bool> {
        try_c!(self, set_bool, {
            Ok(set_bool(self.as_ptr(), if value { 1 } else { 0 }) != 0)
        })
    }

    /// Sets the underlying value as type int. Returns true (1) if the value was
    /// set successfully.
    pub fn set_int(&self, value: i32) -> Result<bool> {
        try_c!(self, set_int, {
            Ok(set_int(self.as_ptr(), value as c_int) != 0)
        })
    }

    /// Sets the underlying value as type double. Returns true (1) if the value
    /// was set successfully.
    pub fn set_double(&self, value: f64) -> Result<bool> {
        try_c!(self, set_double, {
            Ok(set_double(self.as_ptr(), value) != 0)
        })
    }

    /// Sets the underlying value as type string. Returns true (1) if the value
    /// was set successfully.
    pub fn set_string(&self, value: &str) -> Result<bool> {
        try_c!(self, set_string, {
            let value = CefString::new(value);

            Ok(set_string(self.as_ptr(), value.as_ptr()) != 0)
        })
    }

    /// Sets the underlying value as type binary. Returns true (1) if the value
    /// was set successfully. This object keeps a reference to |value| and
    /// ownership of the underlying data remains unchanged.
    pub fn set_binary(&self, value: BinaryValue) -> Result<bool> {
        try_c!(self, set_binary, {
            Ok(set_binary(self.as_ptr(), value.into_raw()) != 0)
        })
    }

    /// Sets the underlying value as type dict. Returns true (1) if the value was
    /// set successfully. This object keeps a reference to |value| and ownership
    /// of the underlying data remains unchanged.
    pub fn set_dictionary(&self, value: DictionaryValue) -> Result<bool> {
        try_c!(self, set_dictionary, {
            Ok(set_dictionary(self.as_ptr(), value.into_raw()) != 0)
        })
    }

    /// Sets the underlying value as type list. Returns true (1) if the value was
    /// set successfully. This object keeps a reference to |value| and ownership
    /// of the underlying data remains unchanged.
    pub fn set_list(&self, value: ListValue) -> Result<bool> {
        try_c!(self, set_list, {
            Ok(set_list(self.as_ptr(), value.into_raw()) != 0)
        })
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
            .unwrap_or_else(|_| false)
    }
}

// Structure representing a binary value. Can be used on any process and
// thread.
ref_counted_ptr!(BinaryValue, cef_binary_value_t);

impl BinaryValue {
    /// Creates a new object that is not owned by any other object. The specified
    /// |data| will be copied.
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
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this object is currently owned by another object.
    pub fn is_owned(&self) -> Result<bool> {
        try_c!(self, is_owned, { Ok(is_owned(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data.
    pub fn is_same(&self, that: BinaryValue) -> Result<bool> {
        try_c!(self, is_same, {
            Ok(is_same(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: BinaryValue) -> Result<bool> {
        try_c!(self, is_equal, {
            Ok(is_equal(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns a copy of this object. The data in this object will also be
    /// copied.
    pub fn copy(&self) -> Result<BinaryValue> {
        try_c!(self, copy, {
            Ok(BinaryValue::from_ptr_unchecked(copy(self.as_ptr())))
        })
    }

    /// Returns a pointer to the beginning of the memory block. The returned
    /// pointer is valid as long as the cef_binary_value_t is alive.
    pub fn get_raw_data(&self) -> Result<*const u8> {
        try_c!(self, get_raw_data, {
            Ok(get_raw_data(self.as_ptr()) as *const u8)
        })
    }

    /// Returns the data size.
    pub fn get_size(&self) -> Result<usize> {
        try_c!(self, get_size, { Ok(get_size(self.as_ptr())) })
    }

    /// Read up to |buffer_size| number of bytes into |buffer|. Reading begins at
    /// the specified byte |data_offset|. Returns the number of bytes read.
    pub fn get_data(&self, buffer: &mut [u8], offset: usize) -> Result<usize> {
        try_c!(self, get_data, {
            Ok(get_data(
                self.as_ptr(),
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len(),
                offset
            ))
        })
    }
}

impl PartialEq for BinaryValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
            .unwrap_or_else(|_| false)
    }
}

// Structure representing a dictionary value. Can be used on any process and
// thread.
ref_counted_ptr!(DictionaryValue, cef_dictionary_value_t);

impl DictionaryValue {
    /// Creates a new object that is not owned by any other object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_dictionary_value_create()) }
    }

    /// Returns true (1) if this object is valid. This object may become invalid
    /// if the underlying data is owned by another object (e.g. list or
    /// dictionary) and that other object is then modified or destroyed. Do not
    /// call any other functions if this function returns false (0).
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this object is currently owned by another object.
    pub fn is_owned(&self) -> Result<bool> {
        try_c!(self, is_owned, { Ok(is_owned(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data. If true (1) modifications to this object will also affect |that|
    /// object and vice-versa.
    pub fn is_same(&self, that: DictionaryValue) -> Result<bool> {
        try_c!(self, is_same, {
            Ok(is_same(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: DictionaryValue) -> Result<bool> {
        try_c!(self, is_equal, {
            Ok(is_equal(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns a writable copy of this object. If |exclude_NULL_children| is true
    /// (1) any NULL dictionaries or lists will be excluded from the copy.
    pub fn copy(&self, exclude_empty_children: bool) -> Result<DictionaryValue> {
        try_c!(self, copy, {
            Ok(DictionaryValue::from_ptr_unchecked(copy(
                self.as_ptr(),
                if exclude_empty_children { 1 } else { 0 }
            )))
        })
    }

    /// Returns the number of values.
    pub fn len(&self) -> Result<usize> {
        try_c!(self, get_size, { Ok(get_size(self.as_ptr())) })
    }

    /// Removes all values. Returns true (1) on success.
    pub fn clear(&self) -> Result<bool> {
        try_c!(self, clear, { Ok(clear(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the current dictionary has a value for the given key.
    pub fn has_key(&self, key: &str) -> Result<bool> {
        try_c!(self, has_key, {
            let key = CefString::new(key);

            Ok(has_key(self.as_ptr(), key.as_ptr()) != 0)
        })
    }

    /// Reads all keys for this dictionary into the specified vector.
    pub fn get_keys(&self) -> Result<Vec<String>> {
        try_c!(self, get_keys, {
            let mut list = CefStringList::new();

            get_keys(self.as_ptr(), list.as_mut_ptr());

            Ok(list.into())
        })
    }

    /// Removes the value at the specified key. Returns true (1) is the value was
    /// removed successfully.
    pub fn remove(&self, key: &str) -> Result<bool> {
        try_c!(self, remove, {
            let key = CefString::new(key);

            Ok(remove(self.as_ptr(), key.as_ptr()) != 0)
        })
    }

    /// Returns the value type for the specified key.
    pub fn get_type(&self, key: &str) -> Result<ValueType> {
        try_c!(self, get_type, {
            let key = CefString::new(key);

            Ok(get_type(self.as_ptr(), key.as_ptr()).into())
        })
    }

    /// Returns the value at the specified key. For simple types the returned
    /// value will copy existing data and modifications to the value will not
    /// modify this object. For complex types (binary, dictionary and list) the
    /// returned value will reference existing data and modifications to the value
    /// will modify this object.
    pub fn get_value(&self, key: &str) -> Result<Option<Value>> {
        try_c!(self, get_value, {
            let key = CefString::new(key);

            Ok(Value::from_ptr(get_value(self.as_ptr(), key.as_ptr())))
        })
    }

    /// Returns the value at the specified key as type bool.
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        try_c!(self, get_bool, {
            let key = CefString::new(key);

            Ok(get_bool(self.as_ptr(), key.as_ptr()) != 0)
        })
    }

    /// Returns the value at the specified key as type int.
    pub fn get_int(&self, key: &str) -> Result<i32> {
        try_c!(self, get_int, {
            let key = CefString::new(key);

            Ok(get_int(self.as_ptr(), key.as_ptr()) as i32)
        })
    }

    /// Returns the value at the specified key as type double.
    pub fn get_double(&self, key: &str) -> Result<f64> {
        try_c!(self, get_double, {
            let key = CefString::new(key);

            Ok(get_double(self.as_ptr(), key.as_ptr()))
        })
    }

    /// Returns the value at the specified key as type string.
    pub fn get_string(&self, key: &str) -> Result<Option<String>> {
        try_c!(self, get_string, {
            let key = CefString::new(key);
            let s = get_string(self.as_ptr(), key.as_ptr());

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Returns the value at the specified key as type binary. The returned value
    /// will reference existing data.
    pub fn get_binary(&self, key: &str) -> Result<Option<BinaryValue>> {
        try_c!(self, get_binary, {
            let key = CefString::new(key);

            Ok(BinaryValue::from_ptr(get_binary(
                self.as_ptr(),
                key.as_ptr()
            )))
        })
    }

    /// Returns the value at the specified key as type dictionary. The returned
    /// value will reference existing data and modifications to the value will
    /// modify this object.
    pub fn get_dictionary(&self, key: &str) -> Result<Option<DictionaryValue>> {
        try_c!(self, get_dictionary, {
            let key = CefString::new(key);

            Ok(DictionaryValue::from_ptr(get_dictionary(
                self.as_ptr(),
                key.as_ptr()
            )))
        })
    }

    /// Returns the value at the specified key as type list. The returned value
    /// will reference existing data and modifications to the value will modify
    /// this object.
    pub fn get_list(&self, key: &str) -> Result<Option<ListValue>> {
        try_c!(self, get_list, {
            let key = CefString::new(key);

            Ok(ListValue::from_ptr(get_list(self.as_ptr(), key.as_ptr())))
        })
    }

    // /// Sets the value at the specified key. Returns true (1) if the value was set
    // /// successfully. If |value| represents simple data then the underlying data
    // /// will be copied and modifications to |value| will not modify this object.
    // /// If |value| represents complex data (binary, dictionary or list) then the
    // /// underlying data will be referenced and modifications to |value| will
    // /// modify this object.
    pub fn set_value(&self, key: &str, value: Value) -> Result<bool> {
        try_c!(self, set_value, {
            let key = CefString::new(key);

            Ok(set_value(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0)
        })
    }

    /// Sets the value at the specified key as type null. Returns true (1) if the
    /// value was set successfully.
    pub fn set_null(&self, key: &str) -> Result<bool> {
        try_c!(self, set_null, {
            let key = CefString::new(key);

            Ok(set_null(self.as_ptr(), key.as_ptr()) != 0)
        })
    }

    /// Sets the value at the specified key as type bool. Returns true (1) if the
    /// value was set successfully.
    pub fn set_bool(&self, key: &str, value: bool) -> Result<bool> {
        try_c!(self, set_bool, {
            let key = CefString::new(key);

            Ok(set_bool(self.as_ptr(), key.as_ptr(), if value { 1 } else { 0 }) != 0)
        })
    }

    /// Sets the value at the specified key as type int. Returns true (1) if the
    /// value was set successfully.
    pub fn set_int(&self, key: &str, value: i32) -> Result<bool> {
        try_c!(self, set_int, {
            let key = CefString::new(key);

            Ok(set_int(self.as_ptr(), key.as_ptr(), value as c_int) != 0)
        })
    }

    /// Sets the value at the specified key as type double. Returns true (1) if
    /// the value was set successfully.
    pub fn set_double(&self, key: &str, value: f64) -> Result<bool> {
        try_c!(self, set_double, {
            let key = CefString::new(key);

            Ok(set_double(self.as_ptr(), key.as_ptr(), value) != 0)
        })
    }

    /// Sets the value at the specified key as type string. Returns true (1) if
    /// the value was set successfully.
    pub fn set_string(&self, key: &str, value: &str) -> Result<bool> {
        try_c!(self, set_string, {
            let key = CefString::new(key);
            let value = CefString::new(value);

            Ok(set_string(self.as_ptr(), key.as_ptr(), value.as_ptr()) != 0)
        })
    }

    /// Sets the value at the specified key as type binary. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_binary(&self, key: &str, value: BinaryValue) -> Result<bool> {
        try_c!(self, set_binary, {
            let key = CefString::new(key);

            Ok(set_binary(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0)
        })
    }

    /// Sets the value at the specified key as type dict. Returns true (1) if the
    /// value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_dictionary(&self, key: &str, value: DictionaryValue) -> Result<bool> {
        try_c!(self, set_dictionary, {
            let key = CefString::new(key);

            Ok(set_dictionary(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0)
        })
    }

    /// Sets the value at the specified key as type list. Returns true (1) if the
    /// value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_list(&self, key: &str, value: ListValue) -> Result<bool> {
        try_c!(self, set_list, {
            let key = CefString::new(key);

            Ok(set_list(self.as_ptr(), key.as_ptr(), value.into_raw()) != 0)
        })
    }
}

impl PartialEq for DictionaryValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
            .unwrap_or_else(|_| false)
    }
}

// Structure representing a list value. Can be used on any process and thread.
ref_counted_ptr!(ListValue, cef_list_value_t);

impl ListValue {
    /// Creates a new object that is not owned by any other object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_list_value_create()) }
    }

    /// Returns true (1) if this object is valid. This object may become invalid
    /// if the underlying data is owned by another object (e.g. list or
    /// dictionary) and that other object is then modified or destroyed. Do not
    /// call any other functions if this function returns false (0).
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this object is currently owned by another object.
    pub fn is_owned(&self) -> Result<bool> {
        try_c!(self, is_owned, { Ok(is_owned(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this object and |that| object have the same underlying
    /// data. If true (1) modifications to this object will also affect |that|
    /// object and vice-versa.
    pub fn is_same(&self, that: ListValue) -> Result<bool> {
        try_c!(self, is_same, {
            Ok(is_same(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns true (1) if this object and |that| object have an equivalent
    /// underlying value but are not necessarily the same object.
    pub fn is_equal(&self, that: ListValue) -> Result<bool> {
        try_c!(self, is_equal, {
            Ok(is_equal(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns a writable copy of this object.
    pub fn copy(&self) -> Result<ListValue> {
        try_c!(self, copy, {
            Ok(ListValue::from_ptr_unchecked(copy(self.as_ptr())))
        })
    }

    /// Sets the number of values. If the number of values is expanded all new
    /// value slots will default to type null. Returns true (1) on success.
    pub fn set_size(&self, size: usize) -> Result<bool> {
        try_c!(self, set_size, { Ok(set_size(self.as_ptr(), size) != 0) })
    }

    /// Returns the number of values.
    pub fn len(&self) -> Result<usize> {
        try_c!(self, get_size, { Ok(get_size(self.as_ptr())) })
    }

    /// Removes all values. Returns true (1) on success.
    pub fn clear(&self) -> Result<bool> {
        try_c!(self, clear, { Ok(clear(self.as_ptr()) != 0) })
    }

    /// Removes the value at the specified index.
    pub fn remove(&self, index: usize) -> Result<bool> {
        try_c!(self, remove, { Ok(remove(self.as_ptr(), index) != 0) })
    }

    /// Returns the value type at the specified index.
    pub fn get_type(&self, index: usize) -> Result<ValueType> {
        try_c!(self, get_type, {
            Ok(get_type(self.as_ptr(), index).into())
        })
    }

    /// Returns the value at the specified index. For simple types the returned
    /// value will copy existing data and modifications to the value will not
    /// modify this object. For complex types (binary, dictionary and list) the
    /// returned value will reference existing data and modifications to the value
    /// will modify this object.
    pub fn get_value(&self, index: usize) -> Result<Option<Value>> {
        try_c!(self, get_value, {
            Ok(Value::from_ptr(get_value(self.as_ptr(), index)))
        })
    }

    /// Returns the value at the specified index as type bool.
    pub fn get_bool(&self, index: usize) -> Result<bool> {
        try_c!(self, get_bool, { Ok(get_bool(self.as_ptr(), index) != 0) })
    }

    /// Returns the value at the specified index as type int.
    pub fn get_int(&self, index: usize) -> Result<i32> {
        try_c!(self, get_int, { Ok(get_int(self.as_ptr(), index) as i32) })
    }

    /// Returns the value at the specified index as type double.
    pub fn get_double(&self, index: usize) -> Result<f64> {
        try_c!(self, get_double, { Ok(get_double(self.as_ptr(), index)) })
    }

    /// Returns the value at the specified index as type string.
    pub fn get_string(&self, index: usize) -> Result<Option<String>> {
        try_c!(self, get_string, {
            let s = get_string(self.as_ptr(), index);

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Returns the value at the specified index as type binary. The returned
    /// value will reference existing data.
    pub fn get_binary(&self, index: usize) -> Result<Option<BinaryValue>> {
        try_c!(self, get_binary, {
            Ok(BinaryValue::from_ptr(get_binary(self.as_ptr(), index)))
        })
    }

    /// Returns the value at the specified index as type dictionary. The returned
    /// value will reference existing data and modifications to the value will
    /// modify this object.
    pub fn get_dictionary(&self, index: usize) -> Result<Option<DictionaryValue>> {
        try_c!(self, get_dictionary, {
            Ok(DictionaryValue::from_ptr(get_dictionary(
                self.as_ptr(),
                index
            )))
        })
    }

    /// Returns the value at the specified index as type list. The returned value
    /// will reference existing data and modifications to the value will modify
    /// this object.
    pub fn get_list(&self, index: usize) -> Result<Option<ListValue>> {
        try_c!(self, get_list, {
            Ok(ListValue::from_ptr(get_list(self.as_ptr(), index)))
        })
    }

    /// Sets the value at the specified index. Returns true (1) if the value was
    /// set successfully. If |value| represents simple data then the underlying
    /// data will be copied and modifications to |value| will not modify this
    /// object. If |value| represents complex data (binary, dictionary or list)
    /// then the underlying data will be referenced and modifications to |value|
    /// will modify this object.
    pub fn set_value(&self, index: usize, value: Value) -> Result<bool> {
        try_c!(self, set_value, {
            Ok(set_value(self.as_ptr(), index, value.into_raw()) != 0)
        })
    }

    /// Sets the value at the specified index as type null. Returns true (1) if
    /// the value was set successfully.
    pub fn set_null(&self, index: usize) -> Result<bool> {
        try_c!(self, set_null, { Ok(set_null(self.as_ptr(), index) != 0) })
    }

    /// Sets the value at the specified index as type bool. Returns true (1) if
    /// the value was set successfully.
    pub fn set_bool(&self, index: usize, value: bool) -> Result<bool> {
        try_c!(self, set_bool, {
            Ok(set_bool(self.as_ptr(), index, if value { 1 } else { 0 }) != 0)
        })
    }

    /// Sets the value at the specified index as type int. Returns true (1) if the
    /// value was set successfully.
    pub fn set_int(&self, index: usize, value: i32) -> Result<bool> {
        try_c!(self, set_int, {
            Ok(set_int(self.as_ptr(), index, value as c_int) != 0)
        })
    }

    /// Sets the value at the specified index as type double. Returns true (1) if
    /// the value was set successfully.
    pub fn set_double(&self, index: usize, value: f64) -> Result<bool> {
        try_c!(self, set_double, {
            Ok(set_double(self.as_ptr(), index, value) != 0)
        })
    }

    /// Sets the value at the specified index as type string. Returns true (1) if
    /// the value was set successfully.
    pub fn set_string(&self, index: usize, value: &str) -> Result<bool> {
        try_c!(self, set_string, {
            let value = CefString::new(value);

            Ok(set_string(self.as_ptr(), index, value.as_ptr()) != 0)
        })
    }

    /// Sets the value at the specified index as type binary. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_binary(&self, index: usize, value: BinaryValue) -> Result<bool> {
        try_c!(self, set_binary, {
            Ok(set_binary(self.as_ptr(), index, value.into_raw()) != 0)
        })
    }

    /// Sets the value at the specified index as type dict. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_dictionary(&self, index: usize, value: DictionaryValue) -> Result<bool> {
        try_c!(self, set_dictionary, {
            Ok(set_dictionary(self.as_ptr(), index, value.into_raw()) != 0)
        })
    }

    /// Sets the value at the specified index as type list. Returns true (1) if
    /// the value was set successfully. If |value| is currently owned by another
    /// object then the value will be copied and the |value| reference will not
    /// change. Otherwise, ownership will be transferred to this object and the
    /// |value| reference will be invalidated.
    pub fn set_list(&self, index: usize, value: ListValue) -> Result<bool> {
        try_c!(self, set_list, {
            Ok(set_list(self.as_ptr(), index, value.into_raw()) != 0)
        })
    }
}

impl PartialEq for ListValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
            .unwrap_or_else(|_| false)
    }
}

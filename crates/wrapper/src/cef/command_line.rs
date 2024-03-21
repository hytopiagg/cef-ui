use crate::{ref_counted_ptr, try_c, CefString, CefStringList, CefStringMap};
use anyhow::Result;
use bindings::{cef_command_line_create, cef_command_line_get_global, cef_command_line_t};
use std::{
    collections::HashMap,
    ffi::{c_char, c_int, CString},
    ptr::null_mut
};

// Structure used to create and/or parse command line arguments. Arguments with
// "--", "-" and, on Windows, "/" prefixes are considered switches. Switches
// will always precede any arguments without switch prefixes. Switches can
// optionally have a value specified using the "=" delimiter (e.g.
// "-switch=value"). An argument of "--" will terminate switch parsing with all
// subsequent tokens, regardless of prefix, being interpreted as non-switch
// arguments. Switch names should be lowercase ASCII and will be converted to
// such if necessary. Switch values will retain the original case and UTF8
// encoding. This structure can be used before cef_initialize() is called.
ref_counted_ptr!(CommandLine, cef_command_line_t);

impl CommandLine {
    /// Create a new cef_command_line_t instance.
    pub fn new() -> CommandLine {
        unsafe { CommandLine::from_ptr_unchecked(cef_command_line_create()) }
    }

    /// Returns the singleton global cef_command_line_t object. The returned object
    /// will be read-only.
    pub fn get_global() -> CommandLine {
        unsafe { CommandLine::from_ptr_unchecked(cef_command_line_get_global()) }
    }

    /// Returns true (1) if this object is valid. Do not call any other functions
    /// if this function returns false (0).
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Returns a writable copy of this object.
    pub fn copy(&self) -> Result<CommandLine> {
        try_c!(self, copy, {
            Ok(CommandLine::from_ptr_unchecked(copy(self.as_ptr())))
        })
    }

    /// Initialize the command line with the specified |argc| and |argv| values.
    /// The first argument must be the name of the program. This function is only
    /// supported on non-Windows platforms.
    #[cfg(not(target_os = "windows"))]
    pub fn init_from_argv(&self, argv: &[&str]) -> Result<()> {
        try_c!(self, init_from_argv, {
            let cstrs = argv
                .iter()
                .map(|arg| CString::new(*arg).unwrap())
                .collect::<Vec<CString>>();

            let argv = cstrs
                .iter()
                .map(|arg| arg.as_ptr())
                .collect::<Vec<*const c_char>>();

            init_from_argv(
                self.as_ptr(),
                argv.len() as c_int,
                argv.as_ptr() as *const *const c_char
            );

            Ok(())
        })
    }

    /// Initialize the command line with the string returned by calling
    /// GetCommandLineW(). This function is only supported on Windows.
    #[cfg(target_os = "windows")]
    pub fn init_from_string(&self, command_line: &str) -> Result<()> {
        try_c!(self, init_from_string, {
            let command_line = CefString::new(command_line);

            init_from_string(self.as_ptr(), command_line.as_ptr());

            Ok(())
        })
    }

    /// Reset the command-line switches and arguments but leave the program
    /// component unchanged.
    pub fn reset(&self) -> Result<()> {
        try_c!(self, reset, {
            reset(self.as_ptr());

            Ok(())
        })
    }

    /// Retrieve the original command line string as a vector of strings. The argv
    /// array: `{ program, [(--|-|/)switch[=value]]*, [--], [argument]* }`
    pub fn get_argv(&self) -> Result<Vec<String>> {
        try_c!(self, get_argv, {
            let mut list = CefStringList::new();

            get_argv(self.as_ptr(), list.as_mut_ptr());

            Ok(list.into())
        })
    }

    /// Constructs and returns the represented command line string. Use this
    /// function cautiously because quoting behavior is unclear.
    pub fn get_command_line_string(&self) -> Result<String> {
        try_c!(self, get_command_line_string, {
            let s = get_command_line_string(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Get the program part of the command line string (the first item).
    pub fn get_program(&self) -> Result<String> {
        try_c!(self, get_program, {
            let s = get_program(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the program part of the command line string (the first item).
    pub fn set_program(&self, program: &str) -> Result<()> {
        try_c!(self, set_program, {
            let program = CefString::new(program);

            set_program(self.as_ptr(), program.as_ptr());

            Ok(())
        })
    }

    /// Returns true (1) if the command line has switches.
    pub fn has_switches(&self) -> Result<bool> {
        try_c!(self, has_switches, { Ok(has_switches(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the command line contains the given switch.
    pub fn has_switch(&self, name: &str) -> Result<bool> {
        try_c!(self, has_switch, {
            let name = CefString::new(name);

            Ok(has_switch(self.as_ptr(), name.as_ptr()) != 0)
        })
    }

    /// Returns the value associated with the given switch. If the switch has no
    /// value or isn't present this function returns the NULL string.
    pub fn get_switch_value(&self, name: &str) -> Result<Option<String>> {
        try_c!(self, get_switch_value, {
            let name = CefString::new(name);
            let s = match get_switch_value(self.as_ptr(), name.as_ptr()) {
                s if s.is_null() => None,
                s => Some(CefString::from_userfree_ptr_unchecked(s).into())
            };

            Ok(s)
        })
    }

    /// Returns the map of switch names and values. If a switch has no value an
    /// NULL string is returned.
    pub fn get_switches(&self) -> Result<HashMap<String, Option<String>>> {
        try_c!(self, get_switches, {
            let mut switches = CefStringMap::new();

            get_switches(self.as_ptr(), switches.as_mut_ptr());

            // This is so silly, the docs say that a map can contain null values,
            // but this isn't actually true. It will still emit a valid CefString,
            // but it will contain an empty string (a null internal pointer). So
            // we have to check for that and convert it to None.
            let switches = switches
                .iter()
                .map(|(k, v)| {
                    (
                        k.into(),
                        match v.is_empty() {
                            true => None,
                            false => Some(v.into())
                        }
                    )
                })
                .collect();

            Ok(switches)
        })
    }

    /// Add a switch to the end of the command line.
    pub fn append_switch(&self, name: &str) -> Result<()> {
        try_c!(self, append_switch, {
            let name = CefString::new(name);

            append_switch(self.as_ptr(), name.as_ptr());

            Ok(())
        })
    }

    /// Add a switch with the specified value to the end of the command line. If
    /// the switch has no value pass an NULL value string.
    pub fn append_switch_with_value(&self, name: &str, value: Option<&str>) -> Result<()> {
        try_c!(self, append_switch_with_value, {
            let name = CefString::new(name);
            let value = value.map(CefString::new);
            let value = value
                .as_ref()
                .map(|value| value.as_ptr())
                .unwrap_or(null_mut());

            append_switch_with_value(self.as_ptr(), name.as_ptr(), value);

            Ok(())
        })
    }

    /// True if there are remaining command line arguments.
    pub fn has_arguments(&self) -> Result<bool> {
        try_c!(self, has_arguments, {
            Ok(has_arguments(self.as_ptr()) != 0)
        })
    }

    /// Get the remaining command line arguments.
    pub fn get_arguments(&self) -> Result<Vec<String>> {
        try_c!(self, get_arguments, {
            let mut list = CefStringList::new();

            get_arguments(self.as_ptr(), list.as_mut_ptr());

            Ok(list.into())
        })
    }

    /// Add an argument to the end of the command line.
    pub fn append_argument(&self, argument: &str) -> Result<()> {
        try_c!(self, append_argument, {
            let argument = CefString::new(argument);

            append_argument(self.as_ptr(), argument.as_ptr());

            Ok(())
        })
    }

    /// Insert a command before the current command. Common for debuggers, like
    /// "valgrind" or "gdb --args".
    pub fn prepend_wrapper(&self, wrapper: &str) -> Result<()> {
        try_c!(self, prepend_wrapper, {
            let wrapper = CefString::new(wrapper);

            prepend_wrapper(self.as_ptr(), wrapper.as_ptr());

            Ok(())
        })
    }
}

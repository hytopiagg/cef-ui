use crate::{ref_counted_ptr, CefString, CefStringList};
use cef_ui_bindings_linux_x86_64::{
    cef_command_line_create, cef_command_line_get_global, cef_command_line_t
};
use std::{
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
    pub fn new() -> CommandLine {
        unsafe { CommandLine::from_ptr_unchecked(cef_command_line_create()) }
    }

    /// Returns the singleton global cef_command_line_t object. The returned object
    /// will be read-only.
    pub fn get_global() -> Option<CommandLine> {
        unsafe { CommandLine::from_ptr(cef_command_line_get_global()) }
    }

    /// Returns true (1) if this object is valid. Do not call any other functions
    /// if this function returns false (0).
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) } != 0)
            .unwrap_or(false)
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) } != 0)
            .unwrap_or(true)
    }

    /// Returns a writable copy of this object.
    pub fn copy(&self) -> Option<CommandLine> {
        self.0
            .copy
            .map(|copy| unsafe { CommandLine::from_ptr_unchecked(copy(self.as_ptr())) })
    }

    /// Initialize the command line with the specified |argc| and |argv| values.
    /// The first argument must be the name of the program. This function is only
    /// supported on non-Windows platforms.
    #[cfg(not(target_os = "windows"))]
    pub fn init_from_argv(&self, argv: &[&str]) {
        if let Some(init_from_argv) = self.0.init_from_argv {
            let cstrs = argv
                .iter()
                .map(|arg| CString::new(*arg).unwrap())
                .collect::<Vec<CString>>();

            let argv = cstrs
                .iter()
                .map(|arg| arg.as_ptr())
                .collect::<Vec<*const c_char>>();

            unsafe {
                init_from_argv(
                    self.as_ptr(),
                    argv.len() as c_int,
                    argv.as_ptr() as *const *const c_char
                );
            }
        }
    }

    /// Initialize the command line with the string returned by calling
    /// GetCommandLineW(). This function is only supported on Windows.
    #[cfg(target_os = "windows")]
    pub fn init_from_string(&self, command_line: &str) {
        if let Some(init_from_string) = self.0.init_from_string {
            unsafe {
                let command_line = CefString::new(command_line);

                init_from_string(self.as_ptr(), command_line.as_ptr());
            }
        }
    }

    /// Reset the command-line switches and arguments but leave the program
    /// component unchanged.
    pub fn reset(&self) {
        if let Some(reset) = self.0.reset {
            unsafe {
                reset(self.as_ptr());
            }
        }
    }

    /// Retrieve the original command line string as a vector of strings. The argv
    /// array: `{ program, [(--|-|/)switch[=value]]*, [--], [argument]* }`
    pub fn get_argv(&self) -> Vec<String> {
        self.0
            .get_argv
            .map(|get_argv| {
                let mut list = CefStringList::new();

                unsafe {
                    get_argv(self.as_ptr(), list.as_mut_ptr());
                }

                list.into()
            })
            .unwrap_or_default()
    }

    /// Constructs and returns the represented command line string. Use this
    /// function cautiously because quoting behavior is unclear.
    pub fn get_command_line_string(&self) -> Option<String> {
        self.0
            .get_command_line_string
            .and_then(|get_command_line_string| {
                let s = unsafe { get_command_line_string(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Get the program part of the command line string (the first item).
    pub fn get_program(&self) -> Option<String> {
        self.0
            .get_program
            .and_then(|get_program| {
                let s = unsafe { get_program(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Set the program part of the command line string (the first item).
    pub fn set_program(&self, program: &str) {
        if let Some(set_program) = self.0.set_program {
            let program = CefString::new(program);

            unsafe {
                set_program(self.as_ptr(), program.as_ptr());
            }
        }
    }

    /// Returns true (1) if the command line has switches.
    pub fn has_switches(&self) -> bool {
        self.0
            .has_switches
            .map(|has_switches| unsafe { has_switches(self.as_ptr()) } != 0)
            .unwrap_or(false)
    }

    /// Returns true (1) if the command line contains the given switch.
    pub fn has_switch(&self, name: &str) -> bool {
        self.0
            .has_switch
            .map(|has_switch| unsafe {
                let name = CefString::new(name);

                has_switch(self.as_ptr(), name.as_ptr()) != 0
            })
            .unwrap_or(false)
    }

    /// Returns the value associated with the given switch. If the switch has no
    /// value or isn't present this function returns the NULL string.
    pub fn get_switch_value(&self, name: &str) -> Option<String> {
        self.0
            .get_switch_value
            .and_then(|get_switch_value| {
                let name = CefString::new(name);

                match unsafe { get_switch_value(self.as_ptr(), name.as_ptr()) } {
                    s if s.is_null() => None,
                    s => CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
                }
            })
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns the map of switch names and values. If a switch has no value an
    //     /// NULL string is returned.
    //     ///
    //     void(CEF_CALLBACK* get_switches)(struct _cef_command_line_t* self,
    //     cef_string_map_t switches);

    /// Add a switch to the end of the command line.
    pub fn append_switch(&self, name: &str) {
        if let Some(append_switch) = self.0.append_switch {
            let name = CefString::new(name);

            unsafe {
                append_switch(self.as_ptr(), name.as_ptr());
            }
        }
    }

    /// Add a switch with the specified value to the end of the command line. If
    /// the switch has no value pass an NULL value string.
    pub fn append_switch_with_value(&self, name: &str, value: Option<&str>) {
        if let Some(append_switch_with_value) = self.0.append_switch_with_value {
            let name = CefString::new(name);
            let value = value.map(CefString::new);
            let value = value
                .as_ref()
                .map(|value| value.as_ptr())
                .unwrap_or(null_mut());

            unsafe {
                append_switch_with_value(self.as_ptr(), name.as_ptr(), value);
            }
        }
    }

    /// True if there are remaining command line arguments.
    pub fn has_arguments(&self) -> bool {
        self.0
            .has_arguments
            .map(|has_arguments| unsafe { has_arguments(self.as_ptr()) } != 0)
            .unwrap_or(false)
    }

    /// Get the remaining command line arguments.
    pub fn get_arguments(&self) -> Vec<String> {
        self.0
            .get_arguments
            .map(|get_arguments| {
                let mut list = CefStringList::new();

                unsafe {
                    get_arguments(self.as_ptr(), list.as_mut_ptr());
                }

                list.into()
            })
            .unwrap_or_default()
    }

    /// Add an argument to the end of the command line.
    pub fn append_argument(&self, argument: &str) {
        if let Some(append_argument) = self.0.append_argument {
            let argument = CefString::new(argument);

            unsafe {
                append_argument(self.as_ptr(), argument.as_ptr());
            }
        }
    }

    /// Insert a command before the current command. Common for debuggers, like
    /// "valgrind" or "gdb --args".
    pub fn prepend_wrapper(&self, wrapper: &str) {
        if let Some(prepend_wrapper) = self.0.prepend_wrapper {
            let wrapper = CefString::new(wrapper);

            unsafe {
                prepend_wrapper(self.as_ptr(), wrapper.as_ptr());
            }
        }
    }
}

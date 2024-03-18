use crate::{
    ref_counted_ptr, CefString, Client, CommandLine, RefCountedPtr, Value, Wrappable, Wrapped
};
use bindings::{
    cef_browser_process_handler_t, cef_client_t, cef_command_line_t, cef_preference_registrar_t,
    cef_preferences_type_t, cef_string_t
};
use std::{ffi::c_int, mem::zeroed, ptr::null_mut};

/// Preferences type passed to
/// CefBrowserProcessHandler::OnRegisterCustomPreferences.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PreferencesType {
    /// Global preferences registered a single time at application startup.
    Global,

    /// Request context preferences registered each time a new CefRequestContext
    /// is created.
    RequestContext
}

impl From<cef_preferences_type_t> for PreferencesType {
    fn from(value: cef_preferences_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_preferences_type_t> for PreferencesType {
    fn from(value: &cef_preferences_type_t) -> Self {
        match value {
            cef_preferences_type_t::CEF_PREFERENCES_TYPE_GLOBAL => PreferencesType::Global,
            cef_preferences_type_t::CEF_PREFERENCES_TYPE_REQUEST_CONTEXT => {
                PreferencesType::RequestContext
            },
        }
    }
}

impl From<PreferencesType> for cef_preferences_type_t {
    fn from(value: PreferencesType) -> Self {
        Self::from(&value)
    }
}

impl From<&PreferencesType> for cef_preferences_type_t {
    fn from(value: &PreferencesType) -> Self {
        match value {
            PreferencesType::Global => cef_preferences_type_t::CEF_PREFERENCES_TYPE_GLOBAL,
            PreferencesType::RequestContext => {
                cef_preferences_type_t::CEF_PREFERENCES_TYPE_REQUEST_CONTEXT
            },
        }
    }
}

/// Structure that manages custom preference registrations.
/// Note: This is meant to be ephemeral and should not be stored!
pub struct PreferenceRegistrar(*mut cef_preference_registrar_t);

impl PreferenceRegistrar {
    pub fn from_ptr_unchecked(ptr: *mut cef_preference_registrar_t) -> Self {
        Self(ptr)
    }

    /// Register a preference with the specified |name| and |default_value|. To
    /// avoid conflicts with built-in preferences the |name| value should contain
    /// an application-specific prefix followed by a period (e.g. "myapp.value").
    /// The contents of |default_value| will be copied. The data type for the
    /// preference will be inferred from |default_value|'s type and cannot be
    /// changed after registration. Returns true (1) on success. Returns false (0)
    /// if |name| is already registered or if |default_value| has an invalid type.
    /// This function must be called from within the scope of the
    /// cef_browser_process_handler_t::OnRegisterCustomPreferences callback.
    pub fn add_preference(&mut self, name: &str, default_value: Value) -> bool {
        unsafe {
            self.0
                .as_mut()
                .and_then(|this| {
                    this.add_preference
                        .map(|add_preference| {
                            let name = CefString::new(name);

                            add_preference(this, name.as_ptr(), default_value.into_raw()) != 0
                        })
                })
                .unwrap_or(false)
        }
    }
}

/// Structure used to implement browser process callbacks. The functions of this
/// structure will be called on the browser process main thread unless otherwise
/// indicated.
#[allow(unused_variables)]
pub trait BrowserProcessHandlerCallbacks: Send + Sync + 'static {
    /// Provides an opportunity to register custom preferences prior to global and
    /// request context initialization.
    ///
    /// If |type| is CEF_PREFERENCES_TYPE_GLOBAL the registered preferences can be
    /// accessed via cef_preference_manager_t::GetGlobalPreferences after
    /// OnContextInitialized is called. Global preferences are registered a single
    /// time at application startup. See related cef_settings_t.cache_path and
    /// cef_settings_t.persist_user_preferences configuration.
    ///
    /// If |type| is CEF_PREFERENCES_TYPE_REQUEST_CONTEXT the preferences can be
    /// accessed via the cef_request_context_t after
    /// cef_request_context_handler_t::OnRequestContextInitialized is called.
    /// Request context preferences are registered each time a new
    /// cef_request_context_t is created. It is intended but not required that all
    /// request contexts have the same registered preferences. See related
    /// cef_request_context_settings_t.cache_path and
    /// cef_request_context_settings_t.persist_user_preferences configuration.
    ///
    /// Do not keep a reference to the |registrar| object. This function is called
    /// on the browser process UI thread.
    fn on_register_custom_preferences(
        &self,
        preferences_type: PreferencesType,
        registrar: &mut PreferenceRegistrar
    ) {
    }

    /// Called on the browser process UI thread immediately after the CEF context
    /// has been initialized.
    fn on_context_initialized(&self) {}

    /// Called before a child process is launched. Will be called on the browser
    /// process UI thread when launching a render process and on the browser
    /// process IO thread when launching a GPU process. Provides an opportunity to
    /// modify the child process command line. Do not keep a reference to
    /// |command_line| outside of this function.
    fn on_before_child_process_launch(&self, command_line: CommandLine) {}

    /// Implement this function to provide app-specific behavior when an already
    /// running app is relaunched with the same CefSettings.root_cache_path value.
    /// For example, activate an existing app window or create a new app window.
    /// |command_line| will be read-only. Do not keep a reference to
    /// |command_line| outside of this function. Return true (1) if the relaunch
    /// is handled or false (0) for default relaunch behavior. Default behavior
    /// will create a new default styled Chrome window.
    ///
    /// To avoid cache corruption only a single app instance is allowed to run for
    /// a given CefSettings.root_cache_path value. On relaunch the app checks a
    /// process singleton lock and then forwards the new launch arguments to the
    /// already running app process before exiting early. Client apps should
    /// therefore check the cef_initialize() return value for early exit before
    /// proceeding.
    ///
    /// This function will be called on the browser process UI thread.
    fn on_already_running_app_relaunch(
        &self,
        command_line: CommandLine,
        current_directory: &str
    ) -> bool {
        false
    }

    /// Called from any thread when work has been scheduled for the browser
    /// process main (UI) thread. This callback is used in combination with
    /// cef_settings_t.external_message_pump and cef_do_message_loop_work() in
    /// cases where the CEF message loop must be integrated into an existing
    /// application message loop (see additional comments and warnings on
    /// CefDoMessageLoopWork). This callback should schedule a
    /// cef_do_message_loop_work() call to happen on the main (UI) thread.
    /// |delay_ms| is the requested delay in milliseconds. If |delay_ms| is <= 0
    /// then the call should happen reasonably soon. If |delay_ms| is > 0 then the
    /// call should be scheduled to happen after the specified delay and any
    /// currently pending scheduled call should be cancelled.
    fn on_schedule_message_pump_work(&self, delay_ms: i64) {}

    /// Return the default client for use with a newly created browser window. If
    /// null is returned the browser will be unmanaged (no callbacks will be
    /// executed for that browser) and application shutdown will be blocked until
    /// the browser window is closed manually. This function is currently only
    /// used with the chrome runtime.
    fn get_default_client(&self) -> Option<Client> {
        None
    }
}

// Structure used to implement browser process callbacks. The functions of this
// structure will be called on the browser process main thread unless otherwise
// indicated.
ref_counted_ptr!(BrowserProcessHandler, cef_browser_process_handler_t);

impl BrowserProcessHandler {
    pub fn new<C: BrowserProcessHandlerCallbacks>(delegate: C) -> Self {
        Self(BrowserProcessHandlerWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct BrowserProcessHandlerWrapper(Box<dyn BrowserProcessHandlerCallbacks>);

impl BrowserProcessHandlerWrapper {
    pub fn new<C: BrowserProcessHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Provides an opportunity to register custom preferences prior to global and
    /// request context initialization.
    ///
    /// If |type| is CEF_PREFERENCES_TYPE_GLOBAL the registered preferences can be
    /// accessed via cef_preference_manager_t::GetGlobalPreferences after
    /// OnContextInitialized is called. Global preferences are registered a single
    /// time at application startup. See related cef_settings_t.cache_path and
    /// cef_settings_t.persist_user_preferences configuration.
    ///
    /// If |type| is CEF_PREFERENCES_TYPE_REQUEST_CONTEXT the preferences can be
    /// accessed via the cef_request_context_t after
    /// cef_request_context_handler_t::OnRequestContextInitialized is called.
    /// Request context preferences are registered each time a new
    /// cef_request_context_t is created. It is intended but not required that all
    /// request contexts have the same registered preferences. See related
    /// cef_request_context_settings_t.cache_path and
    /// cef_request_context_settings_t.persist_user_preferences configuration.
    ///
    /// Do not keep a reference to the |registrar| object. This function is called
    /// on the browser process UI thread.
    unsafe extern "C" fn c_on_register_custom_preferences(
        this: *mut cef_browser_process_handler_t,
        preferences_type: cef_preferences_type_t,
        registrar: *mut cef_preference_registrar_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let mut registrar = PreferenceRegistrar::from_ptr_unchecked(registrar);

        this.0
            .on_register_custom_preferences(preferences_type.into(), &mut registrar);
    }

    /// Called on the browser process UI thread immediately after the CEF context
    /// has been initialized.
    unsafe extern "C" fn c_on_context_initialized(this: *mut cef_browser_process_handler_t) {
        let this: &Self = Wrapped::wrappable(this);

        this.0.on_context_initialized();
    }

    /// Called before a child process is launched. Will be called on the browser
    /// process UI thread when launching a render process and on the browser
    /// process IO thread when launching a GPU process. Provides an opportunity to
    /// modify the child process command line. Do not keep a reference to
    /// |command_line| outside of this function.
    unsafe extern "C" fn c_on_before_child_process_launch(
        this: *mut cef_browser_process_handler_t,
        command_line: *mut cef_command_line_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let command_line = CommandLine::from_ptr_unchecked(command_line);

        this.0
            .on_before_child_process_launch(command_line);
    }

    /// Implement this function to provide app-specific behavior when an already
    /// running app is relaunched with the same CefSettings.root_cache_path value.
    /// For example, activate an existing app window or create a new app window.
    /// |command_line| will be read-only. Do not keep a reference to
    /// |command_line| outside of this function. Return true (1) if the relaunch
    /// is handled or false (0) for default relaunch behavior. Default behavior
    /// will create a new default styled Chrome window.
    ///
    /// To avoid cache corruption only a single app instance is allowed to run for
    /// a given CefSettings.root_cache_path value. On relaunch the app checks a
    /// process singleton lock and then forwards the new launch arguments to the
    /// already running app process before exiting early. Client apps should
    /// therefore check the cef_initialize() return value for early exit before
    /// proceeding.
    ///
    /// This function will be called on the browser process UI thread.
    unsafe extern "C" fn c_on_already_running_app_relaunch(
        this: *mut cef_browser_process_handler_t,
        command_line: *mut cef_command_line_t,
        current_directory: *const cef_string_t
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let command_line = CommandLine::from_ptr_unchecked(command_line);
        let current_directory: String = CefString::from_ptr_unchecked(current_directory).into();

        this.0
            .on_already_running_app_relaunch(command_line, &current_directory) as c_int
    }

    /// Called from any thread when work has been scheduled for the browser
    /// process main (UI) thread. This callback is used in combination with
    /// cef_settings_t.external_message_pump and cef_do_message_loop_work() in
    /// cases where the CEF message loop must be integrated into an existing
    /// application message loop (see additional comments and warnings on
    /// CefDoMessageLoopWork). This callback should schedule a
    /// cef_do_message_loop_work() call to happen on the main (UI) thread.
    /// |delay_ms| is the requested delay in milliseconds. If |delay_ms| is <= 0
    /// then the call should happen reasonably soon. If |delay_ms| is > 0 then the
    /// call should be scheduled to happen after the specified delay and any
    /// currently pending scheduled call should be cancelled.
    unsafe extern "C" fn c_on_schedule_message_pump_work(
        this: *mut cef_browser_process_handler_t,
        delay_ms: i64
    ) {
        let this: &Self = Wrapped::wrappable(this);

        this.0
            .on_schedule_message_pump_work(delay_ms);
    }

    /// Return the default client for use with a newly created browser window. If
    /// null is returned the browser will be unmanaged (no callbacks will be
    /// executed for that browser) and application shutdown will be blocked until
    /// the browser window is closed manually. This function is currently only
    /// used with the chrome runtime.
    unsafe extern "C" fn c_get_default_client(
        this: *mut cef_browser_process_handler_t
    ) -> *mut cef_client_t {
        let this: &Self = Wrapped::wrappable(this);

        this.0
            .get_default_client()
            .map(|client| client.into_raw())
            .unwrap_or_else(null_mut)
    }
}

impl Wrappable for BrowserProcessHandlerWrapper {
    type Cef = cef_browser_process_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_browser_process_handler_t> {
        RefCountedPtr::wrap(
            cef_browser_process_handler_t {
                base:                            unsafe { zeroed() },
                on_register_custom_preferences:  Some(Self::c_on_register_custom_preferences),
                on_context_initialized:          Some(Self::c_on_context_initialized),
                on_before_child_process_launch:  Some(Self::c_on_before_child_process_launch),
                on_already_running_app_relaunch: Some(Self::c_on_already_running_app_relaunch),
                on_schedule_message_pump_work:   Some(Self::c_on_schedule_message_pump_work),
                get_default_client:              Some(Self::c_get_default_client)
            },
            self
        )
    }
}

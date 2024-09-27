use crate::{cef::types::LogItems, free_cef_string, CefString, Color, LogSeverity};
use anyhow::{anyhow, Result};
use cef_ui_sys::{cef_settings_t, cef_string_t};
use dunce::canonicalize;
use std::{
    ffi::c_int,
    mem::{size_of, zeroed},
    path::PathBuf
};

/// Initialization settings. Specify NULL or 0 to get the recommended default
/// values. Many of these and other settings can also configured using command-
/// line switches.
#[derive(Debug)]
pub struct Settings(cef_settings_t);

impl Settings {
    pub fn new() -> Self {
        let mut cef: cef_settings_t = unsafe { zeroed() };

        cef.size = size_of::<cef_settings_t>();

        Self(cef)
    }

    /// Set to true (1) to disable the sandbox for sub-processes. See
    /// cef_sandbox_win.h for requirements to enable the sandbox on Windows. Also
    /// configurable using the "no-sandbox" command-line switch.
    pub fn no_sandbox(mut self, value: bool) -> Self {
        self.0.no_sandbox = value as c_int;
        self
    }

    /// Returns true if the sandbox is enabled.
    pub fn is_sandbox_enabled(&self) -> bool {
        self.0.no_sandbox == 0
    }

    /// The path to a separate executable that will be launched for sub-processes.
    /// If this value is empty on Windows or Linux then the main process
    /// executable will be used. If this value is empty on macOS then a helper
    /// executable must exist at "Contents/Frameworks/<app>
    /// Helper.app/Contents/MacOS/<app> Helper" in the top-level app bundle. See
    /// the comments on CefExecuteProcess() for details. If this value is
    /// non-empty then it must be an absolute path. Also configurable using the
    /// "browser-subprocess-path" command-line switch.
    pub fn browser_subprocess_path(mut self, path: &PathBuf) -> Result<Self> {
        Self::set_path(path, &mut self.0.browser_subprocess_path)?;

        Ok(self)
    }

    /// The path to the CEF framework directory on macOS. If this value is empty
    /// then the framework must exist at "Contents/Frameworks/Chromium Embedded
    /// Framework.framework" in the top-level app bundle. If this value is
    /// non-empty then it must be an absolute path. Also configurable using the
    /// "framework-dir-path" command-line switch.
    pub fn framework_dir_path(mut self, path: &PathBuf) -> Result<Self> {
        Self::set_path(path, &mut self.0.framework_dir_path)?;

        Ok(self)
    }

    /// The path to the main bundle on macOS. If this value is empty then it
    /// defaults to the top-level app bundle. If this value is non-empty then it
    /// must be an absolute path. Also configurable using the "main-bundle-path"
    /// command-line switch.
    pub fn main_bundle_path(mut self, path: &PathBuf) -> Result<Self> {
        Self::set_path(path, &mut self.0.main_bundle_path)?;

        Ok(self)
    }

    /// Set to true (1) to enable use of the Chrome runtime in CEF. This feature
    /// is considered experimental and is not recommended for most users at this
    /// time. See issue #2969 for details.
    pub fn chrome_runtime(mut self, value: bool) -> Self {
        self.0.chrome_runtime = value as c_int;
        self
    }

    /// Set to true (1) to have the browser process message loop run in a separate
    /// thread. If false (0) then the CefDoMessageLoopWork() function must be
    /// called from your application message loop. This option is only supported
    /// on Windows and Linux.
    pub fn multi_threaded_message_loop(mut self, value: bool) -> Self {
        self.0.multi_threaded_message_loop = value as c_int;
        self
    }

    /// Set to true (1) to control browser process main (UI) thread message pump
    /// scheduling via the CefBrowserProcessHandler::OnScheduleMessagePumpWork()
    /// callback. This option is recommended for use in combination with the
    /// CefDoMessageLoopWork() function in cases where the CEF message loop must
    /// be integrated into an existing application message loop (see additional
    /// comments and warnings on CefDoMessageLoopWork). Enabling this option is
    /// not recommended for most users; leave this option disabled and use either
    /// the CefRunMessageLoop() function or multi_threaded_message_loop if
    /// possible.
    pub fn external_message_pump(mut self, value: bool) -> Self {
        self.0.external_message_pump = value as c_int;
        self
    }

    /// Set to true (1) to enable windowless (off-screen) rendering support. Do
    /// not enable this value if the application does not use windowless rendering
    /// as it may reduce rendering performance on some systems.
    pub fn windowless_rendering_enabled(mut self, value: bool) -> Self {
        self.0.windowless_rendering_enabled = value as c_int;
        self
    }

    /// Set to true (1) to disable configuration of browser process features using
    /// standard CEF and Chromium command-line arguments. Configuration can still
    /// be specified using CEF data structures or via the
    /// CefApp::OnBeforeCommandLineProcessing() method.
    pub fn command_line_args_disabled(mut self, value: bool) -> Self {
        self.0.command_line_args_disabled = value as c_int;
        self
    }

    /// The directory where data for the global browser cache will be stored on
    /// disk. If this value is non-empty then it must be an absolute path that is
    /// either equal to or a child directory of CefSettings.root_cache_path. If
    /// this value is empty then browsers will be created in "incognito mode"
    /// where in-memory caches are used for storage and no profile-specific data
    /// is persisted to disk (installation-specific data will still be persisted
    /// in root_cache_path). HTML5 databases such as localStorage will only
    /// persist across sessions if a cache path is specified. Can be overridden
    /// for individual CefRequestContext instances via the
    /// CefRequestContextSettings.cache_path value. When using the Chrome runtime
    /// any child directory value will be ignored and the "default" profile (also
    /// a child directory) will be used instead.
    pub fn cache_path(mut self, path: &PathBuf) -> Result<Self> {
        Self::set_path(path, &mut self.0.cache_path)?;

        Ok(self)
    }

    /// The root directory for installation-specific data and the parent directory
    /// for profile-specific data. All CefSettings.cache_path and
    /// CefRequestContextSettings.cache_path values must have this parent
    /// directory in common. If this value is empty and CefSettings.cache_path is
    /// non-empty then it will default to the CefSettings.cache_path value. Any
    /// non-empty value must be an absolute path. If both values are empty then
    /// the default platform-specific directory will be used
    /// ("~/.config/cef_user_data" directory on Linux, "~/Library/Application
    /// Support/CEF/User Data" directory on MacOS, "AppData\Local\CEF\User Data"
    /// directory under the user profile directory on Windows). Use of the default
    /// directory is not recommended in production applications (see below).
    ///
    /// Multiple application instances writing to the same root_cache_path
    /// directory could result in data corruption. A process singleton lock based
    /// on the root_cache_path value is therefore used to protect against this.
    /// This singleton behavior applies to all CEF-based applications using
    /// version 120 or newer. You should customize root_cache_path for your
    /// application and implement CefBrowserProcessHandler::
    /// OnAlreadyRunningAppRelaunch, which will then be called on any app relaunch
    /// with the same root_cache_path value.
    ///
    /// Failure to set the root_cache_path value correctly may result in startup
    /// crashes or other unexpected behaviors (for example, the sandbox blocking
    /// read/write access to certain files).
    pub fn root_cache_path(mut self, path: &PathBuf) -> Result<Self> {
        Self::set_path(path, &mut self.0.root_cache_path)?;

        Ok(self)
    }

    /// To persist session cookies (cookies without an expiry date or validity
    /// interval) by default when using the global cookie manager set this value
    /// to true (1). Session cookies are generally intended to be transient and
    /// most Web browsers do not persist them. A |cache_path| value must also be
    /// specified to enable this feature. Also configurable using the
    /// "persist-session-cookies" command-line switch. Can be overridden for
    /// individual CefRequestContext instances via the
    /// CefRequestContextSettings.persist_session_cookies value.
    pub fn persist_session_cookies(mut self, value: bool) -> Self {
        self.0.persist_session_cookies = value as c_int;
        self
    }

    /// To persist user preferences as a JSON file in the cache path directory set
    /// this value to true (1). A |cache_path| value must also be specified
    /// to enable this feature. Also configurable using the
    /// "persist-user-preferences" command-line switch. Can be overridden for
    /// individual CefRequestContext instances via the
    /// CefRequestContextSettings.persist_user_preferences value.
    pub fn persist_user_preferences(mut self, value: bool) -> Self {
        self.0.persist_user_preferences = value as c_int;
        self
    }

    /// Value that will be returned as the User-Agent HTTP header. If empty the
    /// default User-Agent string will be used. Also configurable using the
    /// "user-agent" command-line switch.
    pub fn user_agent(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.user_agent);

        self
    }

    /// Value that will be inserted as the product portion of the default
    /// User-Agent string. If empty the Chromium product version will be used. If
    /// |userAgent| is specified this value will be ignored. Also configurable
    /// using the "user-agent-product" command-line switch.
    pub fn user_agent_product(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.user_agent_product);

        self
    }

    /// The locale string that will be passed to WebKit. If empty the default
    /// locale of "en-US" will be used. This value is ignored on Linux where
    /// locale is determined using environment variable parsing with the
    /// precedence order: LANGUAGE, LC_ALL, LC_MESSAGES and LANG. Also
    /// configurable using the "lang" command-line switch.
    pub fn locale(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.locale);

        self
    }

    /// The directory and file name to use for the debug log. If empty a default
    /// log file name and location will be used. On Windows and Linux a
    /// "debug.log" file will be written in the main executable directory. On
    /// MacOS a "~/Library/Logs/[app name]_debug.log" file will be written where
    /// [app name] is the name of the main app executable. Also configurable using
    /// the "log-file" command-line switch.
    pub fn log_file(mut self, value: &PathBuf) -> Result<Self> {
        Self::set_path(value, &mut self.0.log_file)?;

        Ok(self)
    }

    /// The log severity. Only messages of this severity level or higher will be
    /// logged. When set to DISABLE no messages will be written to the log file,
    /// but FATAL messages will still be output to stderr. Also configurable using
    /// the "log-severity" command-line switch with a value of "verbose", "info",
    /// "warning", "error", "fatal" or "disable".
    pub fn log_severity(mut self, value: LogSeverity) -> Self {
        self.0.log_severity = value.into();
        self
    }

    /// The log items prepended to each log line. If not set the default log items
    /// will be used. Also configurable using the "log-items" command-line switch
    /// with a value of "none" for no log items, or a comma-delimited list of
    /// values "pid", "tid", "timestamp" or "tickcount" for custom log items.
    pub fn log_items(mut self, value: LogItems) -> Self {
        self.0.log_items = value.into();
        self
    }

    /// Custom flags that will be used when initializing the V8 JavaScript engine.
    /// The consequences of using custom flags may not be well tested. Also
    /// configurable using the "js-flags" command-line switch.
    pub fn javascript_flags(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.javascript_flags);

        self
    }

    /// The fully qualified path for the resources directory. If this value is
    /// empty the *.pak files must be located in the module directory on
    /// Windows/Linux or the app bundle Resources directory on MacOS. If this
    /// value is non-empty then it must be an absolute path. Also configurable
    /// using the "resources-dir-path" command-line switch.
    pub fn resources_dir_path(mut self, value: &PathBuf) -> Result<Self> {
        Self::set_path(value, &mut self.0.resources_dir_path)?;

        Ok(self)
    }

    /// The fully qualified path for the locales directory. If this value is empty
    /// the locales directory must be located in the module directory. If this
    /// value is non-empty then it must be an absolute path. This value is ignored
    /// on MacOS where pack files are always loaded from the app bundle Resources
    /// directory. Also configurable using the "locales-dir-path" command-line
    /// switch.
    pub fn locales_dir_path(mut self, value: &PathBuf) -> Result<Self> {
        Self::set_path(value, &mut self.0.locales_dir_path)?;

        Ok(self)
    }

    /// Set to true (1) to disable loading of pack files for resources and
    /// locales. A resource bundle handler must be provided for the browser and
    /// render processes via CefApp::GetResourceBundleHandler() if loading of pack
    /// files is disabled. Also configurable using the "disable-pack-loading"
    /// command- line switch.
    pub fn pack_loading_disabled(mut self, value: bool) -> Self {
        self.0.pack_loading_disabled = value as c_int;
        self
    }

    /// Set to a value between 1024 and 65535 to enable remote debugging on the
    /// specified port. Also configurable using the "remote-debugging-port"
    /// command-line switch. Remote debugging can be accessed by loading the
    /// chrome://inspect page in Google Chrome. Port numbers 9222 and 9229 are
    /// discoverable by default. Other port numbers may need to be configured via
    /// "Discover network targets" on the Devices tab.
    pub fn remote_debugging_port(mut self, value: u16) -> Self {
        self.0.remote_debugging_port = value as c_int;
        self
    }

    /// The number of stack trace frames to capture for uncaught exceptions.
    /// Specify a positive value to enable the
    /// CefRenderProcessHandler::OnUncaughtException() callback. Specify 0
    /// (default value) and OnUncaughtException() will not be called. Also
    /// configurable using the "uncaught-exception-stack-size" command-line
    /// switch.
    pub fn uncaught_exception_stack_size(mut self, value: u32) -> Self {
        self.0.uncaught_exception_stack_size = value as c_int;
        self
    }

    /// Background color used for the browser before a document is loaded and when
    /// no document color is specified. The alpha component must be either fully
    /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
    /// opaque then the RGB components will be used as the background color. If
    /// the alpha component is fully transparent for a windowed browser then the
    /// default value of opaque white be used. If the alpha component is fully
    /// transparent for a windowless (off-screen) browser then transparent
    /// painting will be enabled.
    pub fn background_color(mut self, value: Color) -> Self {
        self.0.background_color = value.into();
        self
    }

    /// Comma delimited ordered list of language codes without any whitespace that
    /// will be used in the "Accept-Language" HTTP request header and
    /// "navigator.language" JS attribute. Can be overridden for individual
    /// CefRequestContext instances via the
    /// CefRequestContextSettings.accept_language_list value.
    pub fn accept_language_list(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.accept_language_list);

        self
    }

    /// Comma delimited list of schemes supported by the associated
    /// CefCookieManager. If |cookieable_schemes_exclude_defaults| is false (0)
    /// the default schemes ("http", "https", "ws" and "wss") will also be
    /// supported. Not specifying a |cookieable_schemes_list| value and setting
    /// |cookieable_schemes_exclude_defaults| to true (1) will disable all loading
    /// and saving of cookies. These settings will only impact the global
    /// CefRequestContext. Individual CefRequestContext instances can be
    /// configured via the CefRequestContextSettings.cookieable_schemes_list and
    /// CefRequestContextSettings.cookieable_schemes_exclude_defaults values.
    pub fn cookieable_schemes_list(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.cookieable_schemes_list);

        self
    }

    pub fn cookieable_schemes_exclude_defaults(mut self, value: bool) -> Self {
        self.0
            .cookieable_schemes_exclude_defaults = value as c_int;
        self
    }

    /// Specify an ID to enable Chrome policy management via Platform and OS-user
    /// policies. On Windows, this is a registry key like
    /// "SOFTWARE\\Policies\\Google\\Chrome". On MacOS, this is a bundle ID like
    /// "com.google.Chrome". On Linux, this is an absolute directory path like
    /// "/etc/opt/chrome/policies". Only supported with the Chrome runtime. See
    /// https://support.google.com/chrome/a/answer/9037717 for details.
    ///
    /// Chrome Browser Cloud Management integration, when enabled via the
    /// "enable-chrome-browser-cloud-management" command-line flag, will also use
    /// the specified ID. See https://support.google.com/chrome/a/answer/9116814
    /// for details.
    pub fn chrome_policy_id(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.chrome_policy_id);

        self
    }

    /// Specify an ID for an ICON resource that can be loaded from the main
    /// executable and used when creating default Chrome windows such as DevTools
    /// and Task Manager. If unspecified the default Chromium ICON (IDR_MAINFRAME
    /// [101]) will be loaded from libcef.dll. Only supported with the Chrome
    /// runtime on Windows.
    pub fn chrome_app_icon_id(mut self, value: i32) -> Self {
        self.0.chrome_app_icon_id = value;
        self
    }

    /// Converts to the raw cef type.
    pub fn as_raw(&self) -> &cef_settings_t {
        &self.0
    }

    /// Tries to assign a PathBuf to a cef_string_t.
    fn set_path(path: &PathBuf, cef: &mut cef_string_t) -> Result<()> {
        let path = canonicalize(path)?;
        let path = path
            .to_str()
            .ok_or_else(|| anyhow!("Failed to convert path to utf8."))?;

        *cef = CefString::new(path).into_raw();

        Ok(())
    }

    /// Tries to assign a String to a cef_string_t.
    fn set_string(s: &String, cef: &mut cef_string_t) {
        *cef = CefString::new(s.as_str()).into_raw();
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        free_cef_string(&mut self.0.browser_subprocess_path);
        free_cef_string(&mut self.0.framework_dir_path);
        free_cef_string(&mut self.0.main_bundle_path);
        free_cef_string(&mut self.0.cache_path);
        free_cef_string(&mut self.0.root_cache_path);
        free_cef_string(&mut self.0.user_agent);
        free_cef_string(&mut self.0.user_agent_product);
        free_cef_string(&mut self.0.locale);
        free_cef_string(&mut self.0.log_file);
        free_cef_string(&mut self.0.javascript_flags);
        free_cef_string(&mut self.0.resources_dir_path);
        free_cef_string(&mut self.0.locales_dir_path);
        free_cef_string(&mut self.0.accept_language_list);
        free_cef_string(&mut self.0.cookieable_schemes_list);
        free_cef_string(&mut self.0.chrome_policy_id);
    }
}

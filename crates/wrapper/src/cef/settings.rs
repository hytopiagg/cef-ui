use crate::{
    cef::types::{Color, LogItems, LogSeverity},
    CefString
};
use cef_ui_bindings_linux_x86_64::cef_settings_t;
use std::{ffi::c_int, mem::size_of, path::PathBuf};

/// Wraps cef_settings_t.
#[derive(Debug, Default)]
pub struct Settings {
    /// Set to true (1) to disable the sandbox for sub-processes. See
    /// cef_sandbox_win.h for requirements to enable the sandbox on Windows. Also
    /// configurable using the "no-sandbox" command-line switch.
    pub no_sandbox: bool,

    /// The path to a separate executable that will be launched for sub-processes.
    /// If this value is empty on Windows or Linux then the main process
    /// executable will be used. If this value is empty on macOS then a helper
    /// executable must exist at "Contents/Frameworks/<app>
    /// Helper.app/Contents/MacOS/<app> Helper" in the top-level app bundle. See
    /// the comments on CefExecuteProcess() for details. If this value is
    /// non-empty then it must be an absolute path. Also configurable using the
    /// "browser-subprocess-path" command-line switch.
    pub browser_subprocess_path: Option<PathBuf>,

    /// The path to the CEF framework directory on macOS. If this value is empty
    /// then the framework must exist at "Contents/Frameworks/Chromium Embedded
    /// Framework.framework" in the top-level app bundle. If this value is
    /// non-empty then it must be an absolute path. Also configurable using the
    /// "framework-dir-path" command-line switch.
    pub framework_dir_path: Option<PathBuf>,

    /// The path to the main bundle on macOS. If this value is empty then it
    /// defaults to the top-level app bundle. If this value is non-empty then it
    /// must be an absolute path. Also configurable using the "main-bundle-path"
    /// command-line switch.
    pub main_bundle_path: Option<PathBuf>,

    /// Set to true (1) to enable use of the Chrome runtime in CEF. This feature
    /// is considered experimental and is not recommended for most users at this
    /// time. See issue #2969 for details.
    pub chrome_runtime: bool,

    /// Set to true (1) to have the browser process message loop run in a separate
    /// thread. If false (0) then the CefDoMessageLoopWork() function must be
    /// called from your application message loop. This option is only supported
    /// on Windows and Linux.
    pub multi_threaded_message_loop: bool,

    /// Set to true (1) to control browser process main (UI) thread message pump
    /// scheduling via the CefBrowserProcessHandler::OnScheduleMessagePumpWork()
    /// callback. This option is recommended for use in combination with the
    /// CefDoMessageLoopWork() function in cases where the CEF message loop must
    /// be integrated into an existing application message loop (see additional
    /// comments and warnings on CefDoMessageLoopWork). Enabling this option is
    /// not recommended for most users; leave this option disabled and use either
    /// the CefRunMessageLoop() function or multi_threaded_message_loop if
    /// possible.
    pub external_message_pump: bool,

    /// Set to true (1) to enable windowless (off-screen) rendering support. Do
    /// not enable this value if the application does not use windowless rendering
    /// as it may reduce rendering performance on some systems.
    pub windowless_rendering_enabled: bool,

    /// Set to true (1) to disable configuration of browser process features using
    /// standard CEF and Chromium command-line arguments. Configuration can still
    /// be specified using CEF data structures or via the
    /// CefApp::OnBeforeCommandLineProcessing() method.
    pub command_line_args_disabled: bool,

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
    pub cache_path: Option<PathBuf>,

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
    pub root_cache_path: Option<PathBuf>,

    /// To persist session cookies (cookies without an expiry date or validity
    /// interval) by default when using the global cookie manager set this value
    /// to true (1). Session cookies are generally intended to be transient and
    /// most Web browsers do not persist them. A |cache_path| value must also be
    /// specified to enable this feature. Also configurable using the
    /// "persist-session-cookies" command-line switch. Can be overridden for
    /// individual CefRequestContext instances via the
    /// CefRequestContextSettings.persist_session_cookies value.
    pub persist_session_cookies: bool,

    /// To persist user preferences as a JSON file in the cache path directory set
    /// this value to true (1). A |cache_path| value must also be specified
    /// to enable this feature. Also configurable using the
    /// "persist-user-preferences" command-line switch. Can be overridden for
    /// individual CefRequestContext instances via the
    /// CefRequestContextSettings.persist_user_preferences value.
    pub persist_user_preferences: bool,

    /// Value that will be returned as the User-Agent HTTP header. If empty the
    /// default User-Agent string will be used. Also configurable using the
    /// "user-agent" command-line switch.
    pub user_agent: Option<String>,

    /// Value that will be inserted as the product portion of the default
    /// User-Agent string. If empty the Chromium product version will be used. If
    /// |userAgent| is specified this value will be ignored. Also configurable
    /// using the "user-agent-product" command-line switch.
    pub user_agent_product: Option<String>,

    /// The locale string that will be passed to WebKit. If empty the default
    /// locale of "en-US" will be used. This value is ignored on Linux where
    /// locale is determined using environment variable parsing with the
    /// precedence order: LANGUAGE, LC_ALL, LC_MESSAGES and LANG. Also
    /// configurable using the "lang" command-line switch.
    pub locale: Option<String>,

    /// The directory and file name to use for the debug log. If empty a default
    /// log file name and location will be used. On Windows and Linux a
    /// "debug.log" file will be written in the main executable directory. On
    /// MacOS a "~/Library/Logs/[app name]_debug.log" file will be written where
    /// [app name] is the name of the main app executable. Also configurable using
    /// the "log-file" command-line switch.
    pub log_file: Option<PathBuf>,

    /// The log severity. Only messages of this severity level or higher will be
    /// logged. When set to DISABLE no messages will be written to the log file,
    /// but FATAL messages will still be output to stderr. Also configurable using
    /// the "log-severity" command-line switch with a value of "verbose", "info",
    /// "warning", "error", "fatal" or "disable".
    pub log_severity: LogSeverity,

    /// The log items prepended to each log line. If not set the default log items
    /// will be used. Also configurable using the "log-items" command-line switch
    /// with a value of "none" for no log items, or a comma-delimited list of
    /// values "pid", "tid", "timestamp" or "tickcount" for custom log items.
    pub log_items: LogItems,

    /// Custom flags that will be used when initializing the V8 JavaScript engine.
    /// The consequences of using custom flags may not be well tested. Also
    /// configurable using the "js-flags" command-line switch.
    pub javascript_flags: Option<String>,

    /// The fully qualified path for the resources directory. If this value is
    /// empty the *.pak files must be located in the module directory on
    /// Windows/Linux or the app bundle Resources directory on MacOS. If this
    /// value is non-empty then it must be an absolute path. Also configurable
    /// using the "resources-dir-path" command-line switch.
    pub resources_dir_path: Option<PathBuf>,

    /// The fully qualified path for the locales directory. If this value is empty
    /// the locales directory must be located in the module directory. If this
    /// value is non-empty then it must be an absolute path. This value is ignored
    /// on MacOS where pack files are always loaded from the app bundle Resources
    /// directory. Also configurable using the "locales-dir-path" command-line
    /// switch.
    pub locales_dir_path: Option<PathBuf>,

    /// Set to true (1) to disable loading of pack files for resources and
    /// locales. A resource bundle handler must be provided for the browser and
    /// render processes via CefApp::GetResourceBundleHandler() if loading of pack
    /// files is disabled. Also configurable using the "disable-pack-loading"
    /// command- line switch.
    pub pack_loading_disabled: bool,

    /// Set to a value between 1024 and 65535 to enable remote debugging on the
    /// specified port. Also configurable using the "remote-debugging-port"
    /// command-line switch. Remote debugging can be accessed by loading the
    /// chrome://inspect page in Google Chrome. Port numbers 9222 and 9229 are
    /// discoverable by default. Other port numbers may need to be configured via
    /// "Discover network targets" on the Devices tab.
    pub remote_debugging_port: u16,

    /// The number of stack trace frames to capture for uncaught exceptions.
    /// Specify a positive value to enable the
    /// CefRenderProcessHandler::OnUncaughtException() callback. Specify 0
    /// (default value) and OnUncaughtException() will not be called. Also
    /// configurable using the "uncaught-exception-stack-size" command-line
    /// switch.
    pub uncaught_exception_stack_size: u32,

    /// Background color used for the browser before a document is loaded and when
    /// no document color is specified. The alpha component must be either fully
    /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
    /// opaque then the RGB components will be used as the background color. If
    /// the alpha component is fully transparent for a windowed browser then the
    /// default value of opaque white be used. If the alpha component is fully
    /// transparent for a windowless (off-screen) browser then transparent
    /// painting will be enabled.
    pub background_color: Color,

    /// Comma delimited ordered list of language codes without any whitespace that
    /// will be used in the "Accept-Language" HTTP request header and
    /// "navigator.language" JS attribute. Can be overridden for individual
    /// CefRequestContext instances via the
    /// CefRequestContextSettings.accept_language_list value.
    pub accept_language_list: Option<String>,

    /// Comma delimited list of schemes supported by the associated
    /// CefCookieManager. If |cookieable_schemes_exclude_defaults| is false (0)
    /// the default schemes ("http", "https", "ws" and "wss") will also be
    /// supported. Not specifying a |cookieable_schemes_list| value and setting
    /// |cookieable_schemes_exclude_defaults| to true (1) will disable all loading
    /// and saving of cookies. These settings will only impact the global
    /// CefRequestContext. Individual CefRequestContext instances can be
    /// configured via the CefRequestContextSettings.cookieable_schemes_list and
    /// CefRequestContextSettings.cookieable_schemes_exclude_defaults values.
    pub cookieable_schemes_list:             Option<String>,
    pub cookieable_schemes_exclude_defaults: bool,

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
    pub chrome_policy_id: Option<String>,

    /// Specify an ID for an ICON resource that can be loaded from the main
    /// executable and used when creating default Chrome windows such as DevTools
    /// and Task Manager. If unspecified the default Chromium ICON (IDR_MAINFRAME
    /// [101]) will be loaded from libcef.dll. Only supported with the Chrome
    /// runtime on Windows.
    pub chrome_app_icon_id: i32
}

impl Settings {
    pub fn as_raw(&self) -> cef_settings_t {
        cef_settings_t {
            size:                                size_of::<cef_settings_t>(),
            no_sandbox:                          self.no_sandbox as c_int,
            browser_subprocess_path:             self.browser_subprocess_path.as_raw(),
            framework_dir_path:                  self.framework_dir_path.as_raw(),
            main_bundle_path:                    self.main_bundle_path.as_raw(),
            chrome_runtime:                      self.chrome_runtime as c_int,
            multi_threaded_message_loop:         self.multi_threaded_message_loop as c_int,
            external_message_pump:               self.external_message_pump as c_int,
            windowless_rendering_enabled:        self.windowless_rendering_enabled as c_int,
            command_line_args_disabled:          self.command_line_args_disabled as c_int,
            cache_path:                          self.cache_path.as_raw(),
            root_cache_path:                     self.root_cache_path.as_raw(),
            persist_session_cookies:             self.persist_session_cookies as c_int,
            persist_user_preferences:            self.persist_user_preferences as c_int,
            user_agent:                          self.user_agent.as_raw(),
            user_agent_product:                  self.user_agent_product.as_raw(),
            locale:                              self.locale.as_raw(),
            log_file:                            self.log_file.as_raw(),
            log_severity:                        self.log_severity,
            log_items:                           self.log_items,
            javascript_flags:                    self.javascript_flags.as_raw(),
            resources_dir_path:                  self.resources_dir_path.as_raw(),
            locales_dir_path:                    self.locales_dir_path.as_raw(),
            pack_loading_disabled:               self.pack_loading_disabled as c_int,
            remote_debugging_port:               self.remote_debugging_port as c_int,
            uncaught_exception_stack_size:       self.uncaught_exception_stack_size as c_int,
            background_color:                    self.background_color,
            accept_language_list:                self.accept_language_list.as_raw(),
            cookieable_schemes_list:             self.cookieable_schemes_list.as_raw(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults as c_int,
            chrome_policy_id:                    self.chrome_policy_id.as_raw(),
            chrome_app_icon_id:                  self.chrome_app_icon_id as c_int
        }
    }
}

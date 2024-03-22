use crate::{
    ref_counted_ptr, Browser, BrowserSettings, CefString, Client, DictionaryValue, Frame,
    RefCountedPtr, WindowInfo, WindowOpenDisposition, Wrappable, Wrapped
};
use bindings::{
    cef_browser_settings_t, cef_browser_t, cef_client_t, cef_dictionary_value_t, cef_frame_t,
    cef_life_span_handler_t, cef_popup_features_t, cef_string_t, cef_window_info_t,
    cef_window_open_disposition_t
};
use std::{ffi::c_int, mem::zeroed, ptr::null_mut};

/// Popup window features.
pub struct PopupFeatures {
    pub x:        Option<i32>,
    pub y:        Option<i32>,
    pub width:    Option<i32>,
    pub height:   Option<i32>,
    pub is_popup: bool
}

impl PopupFeatures {
    /// Convert from a pointer.
    pub fn from_ptr(ptr: *const cef_popup_features_t) -> Option<PopupFeatures> {
        unsafe { ptr.as_ref().map(|v| v.into()) }
    }

    /// Convert from a pointer without checking if the pointer is null.
    pub fn from_ptr_unchecked(ptr: *const cef_popup_features_t) -> PopupFeatures {
        unsafe { (*ptr).into() }
    }
}

impl From<cef_popup_features_t> for PopupFeatures {
    fn from(value: cef_popup_features_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_popup_features_t> for PopupFeatures {
    fn from(value: &cef_popup_features_t) -> Self {
        Self {
            x:        match value.xSet {
                0 => None,
                _ => Some(value.x)
            },
            y:        match value.ySet {
                0 => None,
                _ => Some(value.y)
            },
            width:    match value.widthSet {
                0 => None,
                _ => Some(value.width)
            },
            height:   match value.heightSet {
                0 => None,
                _ => Some(value.height)
            },
            is_popup: value.isPopup != 0
        }
    }
}

impl From<PopupFeatures> for cef_popup_features_t {
    fn from(value: PopupFeatures) -> Self {
        Self::from(&value)
    }
}

impl From<&PopupFeatures> for cef_popup_features_t {
    fn from(value: &PopupFeatures) -> Self {
        cef_popup_features_t {
            x:         value.x.unwrap_or(0),
            xSet:      value.x.is_some() as c_int,
            y:         value.y.unwrap_or(0),
            ySet:      value.y.is_some() as c_int,
            width:     value.width.unwrap_or(0),
            widthSet:  value.width.is_some() as c_int,
            height:    value.height.unwrap_or(0),
            heightSet: value.height.is_some() as c_int,
            isPopup:   value.is_popup as c_int
        }
    }
}

/// Implement this structure to handle events related to browser life span. The
/// functions of this structure will be called on the UI thread unless otherwise
/// indicated.
pub trait LifeSpanHandlerCallbacks: Send + Sync + 'static {
    /// Called on the UI thread before a new popup browser is created. The
    /// |browser| and |frame| values represent the source of the popup request.
    /// The |target_url| and |target_frame_name| values indicate where the popup
    /// browser should navigate and may be NULL if not specified with the request.
    /// The |target_disposition| value indicates where the user intended to open
    /// the popup (e.g. current tab, new tab, etc). The |user_gesture| value will
    /// be true (1) if the popup was opened via explicit user gesture (e.g.
    /// clicking a link) or false (0) if the popup opened automatically (e.g. via
    /// the DomContentLoaded event). The |popupFeatures| structure contains
    /// additional information about the requested popup window. To allow creation
    /// of the popup browser optionally modify |windowInfo|, |client|, |settings|
    /// and |no_javascript_access| and return false (0). To cancel creation of the
    /// popup browser return true (1). The |client| and |settings| values will
    /// default to the source browser's values. If the |no_javascript_access|
    /// value is set to false (0) the new browser will not be scriptable and may
    /// not be hosted in the same renderer process as the source browser. Any
    /// modifications to |windowInfo| will be ignored if the parent browser is
    /// wrapped in a cef_browser_view_t. Popup browser creation will be canceled
    /// if the parent browser is destroyed before the popup browser creation
    /// completes (indicated by a call to OnAfterCreated for the popup browser).
    /// The |extra_info| parameter provides an opportunity to specify extra
    /// information specific to the created popup browser that will be passed to
    /// cef_render_process_handler_t::on_browser_created() in the render process.
    unsafe fn on_before_popup(
        &mut self,
        browser: Browser,
        frame: Frame,
        target_url: Option<String>,
        target_frame_name: Option<String>,
        target_disposition: WindowOpenDisposition,
        user_gesture: bool,
        popup_features: PopupFeatures,
        window_info: &mut WindowInfo,
        client: &mut Option<Client>,
        settings: &mut BrowserSettings,
        extra_info: &mut Option<DictionaryValue>,
        no_javascript_access: &mut bool
    ) -> bool;

    /// Called on the UI thread before a new DevTools popup browser is created.
    /// The |browser| value represents the source of the popup request. Optionally
    /// modify |windowInfo|, |client|, |settings| and |extra_info| values. The
    /// |client|, |settings| and |extra_info| values will default to the source
    /// browser's values. Any modifications to |windowInfo| will be ignored if the
    /// parent browser is Views-hosted (wrapped in a cef_browser_view_t).
    ///
    /// The |extra_info| parameter provides an opportunity to specify extra
    /// information specific to the created popup browser that will be passed to
    /// cef_render_process_handler_t::on_browser_created() in the render process.
    /// The existing |extra_info| object, if any, will be read-only but may be
    /// replaced with a new object.
    ///
    /// Views-hosted source browsers will create Views-hosted DevTools popups
    /// unless |use_default_window| is set to to true (1). DevTools popups can be
    /// blocked by returning true (1) from cef_command_handler_t::OnChromeCommand
    /// for IDC_DEV_TOOLS. Only used with the Chrome runtime.
    fn on_before_dev_tools_popup(
        &mut self,
        browser: Browser,
        window_info: &mut WindowInfo,
        client: &mut Option<Client>,
        settings: &mut BrowserSettings,
        extra_info: &mut Option<DictionaryValue>,
        use_default_window: &mut bool
    );

    /// Called after a new browser is created. It is now safe to begin performing
    /// actions with |browser|. cef_frame_handler_t callbacks related to initial
    /// main frame creation will arrive before this callback. See
    /// cef_frame_handler_t documentation for additional usage information.
    fn on_after_created(&mut self, browser: Browser);

    ///
    /// Called when a browser has received a request to close. This may result
    /// directly from a call to cef_browser_host_t::*close_browser() or indirectly
    /// if the browser is parented to a top-level window created by CEF and the
    /// user attempts to close that window (by clicking the 'X', for example). The
    /// do_close() function will be called after the JavaScript 'onunload' event
    /// has been fired.
    ///
    /// An application should handle top-level owner window close notifications by
    /// calling cef_browser_host_t::try_close_browser() or
    /// cef_browser_host_t::CloseBrowser(false (0)) instead of allowing the window
    /// to close immediately (see the examples below). This gives CEF an
    /// opportunity to process the 'onbeforeunload' event and optionally cancel
    /// the close before do_close() is called.
    ///
    /// When windowed rendering is enabled CEF will internally create a window or
    /// view to host the browser. In that case returning false (0) from do_close()
    /// will send the standard close notification to the browser's top-level owner
    /// window (e.g. WM_CLOSE on Windows, performClose: on OS X, "delete_event" on
    /// Linux or cef_window_delegate_t::can_close() callback from Views). If the
    /// browser's host window/view has already been destroyed (via view hierarchy
    /// tear-down, for example) then do_close() will not be called for that
    /// browser since is no longer possible to cancel the close.
    ///
    /// When windowed rendering is disabled returning false (0) from do_close()
    /// will cause the browser object to be destroyed immediately.
    ///
    /// If the browser's top-level owner window requires a non-standard close
    /// notification then send that notification from do_close() and return true
    /// (1).
    ///
    /// The cef_life_span_handler_t::on_before_close() function will be called
    /// after do_close() (if do_close() is called) and immediately before the
    /// browser object is destroyed. The application should only exit after
    /// on_before_close() has been called for all existing browsers.
    ///
    /// The below examples describe what should happen during window close when
    /// the browser is parented to an application-provided top-level window.
    ///
    /// Example 1: Using cef_browser_host_t::try_close_browser(). This is
    /// recommended for clients using standard close handling and windows created
    /// on the browser process UI thread. 1.  User clicks the window close button
    /// which sends a close notification
    ///     to the application's top-level window.
    /// 2.  Application's top-level window receives the close notification and
    ///     calls TryCloseBrowser() (which internally calls CloseBrowser(false)).
    ///     TryCloseBrowser() returns false so the client cancels the window
    ///     close.
    /// 3.  JavaScript 'onbeforeunload' handler executes and shows the close
    ///     confirmation dialog (which can be overridden via
    ///     CefJSDialogHandler::OnBeforeUnloadDialog()).
    /// 4.  User approves the close. 5.  JavaScript 'onunload' handler executes.
    /// 6.  CEF sends a close notification to the application's top-level window
    ///     (because DoClose() returned false by default).
    /// 7.  Application's top-level window receives the close notification and
    ///     calls TryCloseBrowser(). TryCloseBrowser() returns true so the client
    ///     allows the window close.
    /// 8.  Application's top-level window is destroyed. 9.  Application's
    /// on_before_close() handler is called and the browser object
    ///     is destroyed.
    /// 10. Application exits by calling cef_quit_message_loop() if no other
    /// browsers
    ///     exist.
    ///
    /// Example 2: Using cef_browser_host_t::CloseBrowser(false (0)) and
    /// implementing the do_close() callback. This is recommended for clients
    /// using non-standard close handling or windows that were not created on the
    /// browser process UI thread. 1.  User clicks the window close button which
    /// sends a close notification
    ///     to the application's top-level window.
    /// 2.  Application's top-level window receives the close notification and:
    ///     A. Calls CefBrowserHost::CloseBrowser(false).
    ///     B. Cancels the window close.
    /// 3.  JavaScript 'onbeforeunload' handler executes and shows the close
    ///     confirmation dialog (which can be overridden via
    ///     CefJSDialogHandler::OnBeforeUnloadDialog()).
    /// 4.  User approves the close. 5.  JavaScript 'onunload' handler executes.
    /// 6.  Application's do_close() handler is called. Application will:
    ///     A. Set a flag to indicate that the next close attempt will be allowed.
    ///     B. Return false.
    /// 7.  CEF sends an close notification to the application's top-level window.
    /// 8.  Application's top-level window receives the close notification and
    ///     allows the window to close based on the flag from #6B.
    /// 9.  Application's top-level window is destroyed. 10. Application's
    /// on_before_close() handler is called and the browser object
    ///     is destroyed.
    /// 11. Application exits by calling cef_quit_message_loop() if no other
    /// browsers exist.
    fn do_close(&mut self, browser: Browser) -> bool;

    /// Called just before a browser is destroyed. Release all references to the
    /// browser object and do not attempt to execute any functions on the browser
    /// object (other than IsValid, GetIdentifier or IsSame) after this callback
    /// returns. cef_frame_handler_t callbacks related to final main frame
    /// destruction will arrive after this callback and cef_browser_t::IsValid
    /// will return false (0) at that time. Any in-progress network requests
    /// associated with |browser| will be aborted when the browser is destroyed,
    /// and cef_resource_request_handler_t callbacks related to those requests may
    /// still arrive on the IO thread after this callback. See cef_frame_handler_t
    /// and do_close() documentation for additional usage information.
    fn on_before_close(&mut self, browser: Browser);
}

// Implement this structure to handle events related to browser life span. The
// functions of this structure will be called on the UI thread unless otherwise
// indicated.
ref_counted_ptr!(LifeSpanHandler, cef_life_span_handler_t);

impl LifeSpanHandler {
    pub fn new<C: LifeSpanHandlerCallbacks>(delegate: C) -> Self {
        Self(LifeSpanHandlerWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct LifeSpanHandlerWrapper(Box<dyn LifeSpanHandlerCallbacks>);

impl LifeSpanHandlerWrapper {
    pub fn new<C: LifeSpanHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Called on the UI thread before a new popup browser is created. The
    /// |browser| and |frame| values represent the source of the popup request.
    /// The |target_url| and |target_frame_name| values indicate where the popup
    /// browser should navigate and may be NULL if not specified with the request.
    /// The |target_disposition| value indicates where the user intended to open
    /// the popup (e.g. current tab, new tab, etc). The |user_gesture| value will
    /// be true (1) if the popup was opened via explicit user gesture (e.g.
    /// clicking a link) or false (0) if the popup opened automatically (e.g. via
    /// the DomContentLoaded event). The |popupFeatures| structure contains
    /// additional information about the requested popup window. To allow creation
    /// of the popup browser optionally modify |windowInfo|, |client|, |settings|
    /// and |no_javascript_access| and return false (0). To cancel creation of the
    /// popup browser return true (1). The |client| and |settings| values will
    /// default to the source browser's values. If the |no_javascript_access|
    /// value is set to false (0) the new browser will not be scriptable and may
    /// not be hosted in the same renderer process as the source browser. Any
    /// modifications to |windowInfo| will be ignored if the parent browser is
    /// wrapped in a cef_browser_view_t. Popup browser creation will be canceled
    /// if the parent browser is destroyed before the popup browser creation
    /// completes (indicated by a call to OnAfterCreated for the popup browser).
    /// The |extra_info| parameter provides an opportunity to specify extra
    /// information specific to the created popup browser that will be passed to
    /// cef_render_process_handler_t::on_browser_created() in the render process.
    unsafe extern "C" fn c_on_before_popup(
        this: *mut cef_life_span_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        target_url: *const cef_string_t,
        target_frame_name: *const cef_string_t,
        target_disposition: cef_window_open_disposition_t,
        user_gesture: c_int,
        popup_features: *const cef_popup_features_t,
        window_info: *mut cef_window_info_t,
        client: *mut *mut cef_client_t,
        settings: *mut cef_browser_settings_t,
        extra_info: *mut *mut cef_dictionary_value_t,
        no_javascript_access: *mut c_int
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let target_url: Option<String> = CefString::from_ptr(target_url).map(|s| s.into());
        let target_frame_name: Option<String> =
            CefString::from_ptr(target_frame_name).map(|s| s.into());
        let target_disposition = target_disposition.into();
        let user_gesture = user_gesture != 0;
        let popup_features = PopupFeatures::from_ptr_unchecked(popup_features);
        let window_info = WindowInfo::from_ptr_mut_unchecked(window_info);
        let local_client_ptr = *client;
        let mut local_client = Client::from_ptr(local_client_ptr);
        let settings = BrowserSettings::from_ptr_mut_unchecked(settings);
        let local_extra_info_ptr = *extra_info;
        let mut local_extra_info = DictionaryValue::from_ptr(local_extra_info_ptr);
        let mut local_no_javascript_access = *no_javascript_access != 0;

        let ret = this.0.on_before_popup(
            browser,
            frame,
            target_url,
            target_frame_name,
            target_disposition,
            user_gesture,
            popup_features,
            window_info,
            &mut local_client,
            settings,
            &mut local_extra_info,
            &mut local_no_javascript_access
        );

        // This is terribly gross and continues to increase my hatred for C.
        // CEF passes the client and extra_info parameters as *mut *mut so you
        // can either modify or replace the value. These are refcounted values
        // though so we have to be very careful. In any case, our smart pointer
        // wrapper will decrease the reference count when dropped as expected.
        // However, if the value was replaced, we must "forget" the new smart
        // pointer wrapper so that it doesn't get dropped before we hand it off
        // to C. We can't forget the original smart pointer wrapper because it
        // would prevent the reference count from being decreased, hence the
        // pointer comparison here. :^(
        *client = local_client
            .map(|c| match c.as_ptr() == local_client_ptr {
                true => local_client_ptr,
                false => c.into_raw()
            })
            .unwrap_or_else(null_mut);

        *extra_info = local_extra_info
            .map(|e| match e.as_ptr() == local_extra_info_ptr {
                true => local_extra_info_ptr,
                false => e.into_raw()
            })
            .unwrap_or_else(null_mut);

        *no_javascript_access = local_no_javascript_access as c_int;

        ret as c_int
    }

    /// Called on the UI thread before a new DevTools popup browser is created.
    /// The |browser| value represents the source of the popup request. Optionally
    /// modify |windowInfo|, |client|, |settings| and |extra_info| values. The
    /// |client|, |settings| and |extra_info| values will default to the source
    /// browser's values. Any modifications to |windowInfo| will be ignored if the
    /// parent browser is Views-hosted (wrapped in a cef_browser_view_t).
    ///
    /// The |extra_info| parameter provides an opportunity to specify extra
    /// information specific to the created popup browser that will be passed to
    /// cef_render_process_handler_t::on_browser_created() in the render process.
    /// The existing |extra_info| object, if any, will be read-only but may be
    /// replaced with a new object.
    ///
    /// Views-hosted source browsers will create Views-hosted DevTools popups
    /// unless |use_default_window| is set to to true (1). DevTools popups can be
    /// blocked by returning true (1) from cef_command_handler_t::OnChromeCommand
    /// for IDC_DEV_TOOLS. Only used with the Chrome runtime.
    unsafe extern "C" fn c_on_before_dev_tools_popup(
        this: *mut cef_life_span_handler_t,
        browser: *mut cef_browser_t,
        window_info: *mut cef_window_info_t,
        client: *mut *mut cef_client_t,
        settings: *mut cef_browser_settings_t,
        extra_info: *mut *mut cef_dictionary_value_t,
        use_default_window: *mut c_int
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let window_info = WindowInfo::from_ptr_mut_unchecked(window_info);
        let local_client_ptr = *client;
        let mut local_client = Client::from_ptr(local_client_ptr);
        let settings = BrowserSettings::from_ptr_mut_unchecked(settings);
        let local_extra_info_ptr = *extra_info;
        let mut local_extra_info = DictionaryValue::from_ptr(local_extra_info_ptr);
        let mut local_use_default_window = *use_default_window != 0;

        this.0.on_before_dev_tools_popup(
            browser,
            window_info,
            &mut local_client,
            settings,
            &mut local_extra_info,
            &mut local_use_default_window
        );

        // Same horrible pointer comparison dance here. :^(
        *client = local_client
            .map(|c| match c.as_ptr() == local_client_ptr {
                true => local_client_ptr,
                false => c.into_raw()
            })
            .unwrap_or_else(null_mut);

        *extra_info = local_extra_info
            .map(|e| match e.as_ptr() == local_extra_info_ptr {
                true => local_extra_info_ptr,
                false => e.into_raw()
            })
            .unwrap_or_else(null_mut);

        *use_default_window = local_use_default_window as c_int;
    }

    /// Called after a new browser is created. It is now safe to begin performing
    /// actions with |browser|. cef_frame_handler_t callbacks related to initial
    /// main frame creation will arrive before this callback. See
    /// cef_frame_handler_t documentation for additional usage information.
    unsafe extern "C" fn c_on_after_created(
        this: *mut cef_life_span_handler_t,
        browser: *mut cef_browser_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0.on_after_created(browser);
    }

    /// Called when a browser has received a request to close. This may result
    /// directly from a call to cef_browser_host_t::*close_browser() or indirectly
    /// if the browser is parented to a top-level window created by CEF and the
    /// user attempts to close that window (by clicking the 'X', for example). The
    /// do_close() function will be called after the JavaScript 'onunload' event
    /// has been fired.
    ///
    /// An application should handle top-level owner window close notifications by
    /// calling cef_browser_host_t::try_close_browser() or
    /// cef_browser_host_t::CloseBrowser(false (0)) instead of allowing the window
    /// to close immediately (see the examples below). This gives CEF an
    /// opportunity to process the 'onbeforeunload' event and optionally cancel
    /// the close before do_close() is called.
    ///
    /// When windowed rendering is enabled CEF will internally create a window or
    /// view to host the browser. In that case returning false (0) from do_close()
    /// will send the standard close notification to the browser's top-level owner
    /// window (e.g. WM_CLOSE on Windows, performClose: on OS X, "delete_event" on
    /// Linux or cef_window_delegate_t::can_close() callback from Views). If the
    /// browser's host window/view has already been destroyed (via view hierarchy
    /// tear-down, for example) then do_close() will not be called for that
    /// browser since is no longer possible to cancel the close.
    ///
    /// When windowed rendering is disabled returning false (0) from do_close()
    /// will cause the browser object to be destroyed immediately.
    ///
    /// If the browser's top-level owner window requires a non-standard close
    /// notification then send that notification from do_close() and return true
    /// (1).
    ///
    /// The cef_life_span_handler_t::on_before_close() function will be called
    /// after do_close() (if do_close() is called) and immediately before the
    /// browser object is destroyed. The application should only exit after
    /// on_before_close() has been called for all existing browsers.
    ///
    /// The below examples describe what should happen during window close when
    /// the browser is parented to an application-provided top-level window.
    ///
    /// Example 1: Using cef_browser_host_t::try_close_browser(). This is
    /// recommended for clients using standard close handling and windows created
    /// on the browser process UI thread. 1.  User clicks the window close button
    /// which sends a close notification
    ///     to the application's top-level window.
    /// 2.  Application's top-level window receives the close notification and
    ///     calls TryCloseBrowser() (which internally calls CloseBrowser(false)).
    ///     TryCloseBrowser() returns false so the client cancels the window
    ///     close.
    /// 3.  JavaScript 'onbeforeunload' handler executes and shows the close
    ///     confirmation dialog (which can be overridden via
    ///     CefJSDialogHandler::OnBeforeUnloadDialog()).
    /// 4.  User approves the close. 5.  JavaScript 'onunload' handler executes.
    /// 6.  CEF sends a close notification to the application's top-level window
    ///     (because DoClose() returned false by default).
    /// 7.  Application's top-level window receives the close notification and
    ///     calls TryCloseBrowser(). TryCloseBrowser() returns true so the client
    ///     allows the window close.
    /// 8.  Application's top-level window is destroyed. 9.  Application's
    /// on_before_close() handler is called and the browser object
    ///     is destroyed.
    /// 10. Application exits by calling cef_quit_message_loop() if no other
    /// browsers
    ///     exist.
    ///
    /// Example 2: Using cef_browser_host_t::CloseBrowser(false (0)) and
    /// implementing the do_close() callback. This is recommended for clients
    /// using non-standard close handling or windows that were not created on the
    /// browser process UI thread. 1.  User clicks the window close button which
    /// sends a close notification
    ///     to the application's top-level window.
    /// 2.  Application's top-level window receives the close notification and:
    ///     A. Calls CefBrowserHost::CloseBrowser(false).
    ///     B. Cancels the window close.
    /// 3.  JavaScript 'onbeforeunload' handler executes and shows the close
    ///     confirmation dialog (which can be overridden via
    ///     CefJSDialogHandler::OnBeforeUnloadDialog()).
    /// 4.  User approves the close. 5.  JavaScript 'onunload' handler executes.
    /// 6.  Application's do_close() handler is called. Application will:
    ///     A. Set a flag to indicate that the next close attempt will be allowed.
    ///     B. Return false.
    /// 7.  CEF sends an close notification to the application's top-level window.
    /// 8.  Application's top-level window receives the close notification and
    ///     allows the window to close based on the flag from #6B.
    /// 9.  Application's top-level window is destroyed. 10. Application's
    /// on_before_close() handler is called and the browser object
    ///     is destroyed.
    /// 11. Application exits by calling cef_quit_message_loop() if no other
    /// browsers
    ///     exist.
    unsafe extern "C" fn c_do_close(
        this: *mut cef_life_span_handler_t,
        browser: *mut cef_browser_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0.do_close(browser) as c_int
    }

    /// Called just before a browser is destroyed. Release all references to the
    /// browser object and do not attempt to execute any functions on the browser
    /// object (other than IsValid, GetIdentifier or IsSame) after this callback
    /// returns. cef_frame_handler_t callbacks related to final main frame
    /// destruction will arrive after this callback and cef_browser_t::IsValid
    /// will return false (0) at that time. Any in-progress network requests
    /// associated with |browser| will be aborted when the browser is destroyed,
    /// and cef_resource_request_handler_t callbacks related to those requests may
    /// still arrive on the IO thread after this callback. See cef_frame_handler_t
    /// and do_close() documentation for additional usage information.
    unsafe extern "C" fn c_on_before_close(
        this: *mut cef_life_span_handler_t,
        browser: *mut cef_browser_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0.on_before_close(browser);
    }
}

impl Wrappable for LifeSpanHandlerWrapper {
    type Cef = cef_life_span_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_life_span_handler_t> {
        RefCountedPtr::wrap(
            cef_life_span_handler_t {
                base:                      unsafe { zeroed() },
                on_before_popup:           Some(Self::c_on_before_popup),
                on_before_dev_tools_popup: Some(Self::c_on_before_dev_tools_popup),
                on_after_created:          Some(Self::c_on_after_created),
                do_close:                  Some(Self::c_do_close),
                on_before_close:           Some(Self::c_on_before_close)
            },
            self
        )
    }
}

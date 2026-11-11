//! The tray icon: a hidden window that owns the notification icon, polls the
//! lock keys on a timer and re-adds the icon when Explorer restarts.
//!
//! The window is a hidden top-level one, not `HWND_MESSAGE`: message-only
//! windows do not receive broadcasts such as `TaskbarCreated` and
//! `WM_SETTINGCHANGE`.

use std::cell::RefCell;
use std::io;
use std::mem;
use std::ptr;

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Shell::{
    NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NIM_SETVERSION,
    NOTIFYICON_VERSION_4, NOTIFYICONDATAW, Shell_NotifyIconW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateIcon, CreateWindowExW, DefWindowProcW, DestroyIcon, DispatchMessageW, GetMessageW,
    GetSystemMetrics, HICON, KillTimer, MSG, PostQuitMessage, RegisterClassExW,
    RegisterWindowMessageW, SM_CXSMICON, SetTimer, TranslateMessage, WM_APP, WM_DESTROY,
    WM_SETTINGCHANGE, WM_TIMER, WNDCLASSEXW, WS_OVERLAPPED,
};

use super::{keyboard, locale, theme, wide};
use crate::{ChangeDetector, Lang, LockState, POLL_INTERVAL, Theme, icon};

const CLASS_NAME: &str = "caps-widget";
const ICON_ID: u32 = 1;
const TIMER_ID: usize = 1;
/// Notifications from the tray icon arrive as this message.
const WM_TRAY: u32 = WM_APP + 1;

struct App {
    hwnd: HWND,
    taskbar_created: u32,
    detector: ChangeDetector,
    state: LockState,
    theme: Theme,
    lang: Lang,
    icon: HICON,
}

thread_local! {
    static APP: RefCell<Option<App>> = const { RefCell::new(None) };
}

/// Runs `f` on the app state. Returns `None` (and skips `f`) when called
/// re-entrantly — e.g. a timer tick dispatched while a Shell call is pending —
/// so a tick is dropped instead of panicking across the FFI boundary.
fn with_app<R>(f: impl FnOnce(&mut App) -> R) -> Option<R> {
    APP.with(|app| app.try_borrow_mut().ok()?.as_mut().map(f))
}

/// Creates the window and icon, then runs the message loop until exit.
pub fn run() -> io::Result<()> {
    // SAFETY: null asks for the handle of the current executable.
    let instance = unsafe { GetModuleHandleW(ptr::null()) };
    let class_name = wide(CLASS_NAME);
    let class = WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(window_proc),
        hInstance: instance,
        lpszClassName: class_name.as_ptr(),
        // SAFETY: every other field is a nullable handle, pointer or integer,
        // for which all-zero means "none".
        ..unsafe { mem::zeroed() }
    };
    // SAFETY: `class` is fully initialized; `class_name` outlives the window.
    if unsafe { RegisterClassExW(&class) } == 0 {
        return Err(io::Error::last_os_error());
    }

    // SAFETY: the class was registered above; no parent, menu or create data.
    // Without WS_VISIBLE the window is never shown (R11).
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPED,
            0,
            0,
            0,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            instance,
            ptr::null(),
        )
    };
    if hwnd.is_null() {
        return Err(io::Error::last_os_error());
    }

    let taskbar_created_name = wide("TaskbarCreated");
    // SAFETY: the name is NUL-terminated.
    let taskbar_created = unsafe { RegisterWindowMessageW(taskbar_created_name.as_ptr()) };

    let mut app = App {
        hwnd,
        taskbar_created,
        detector: ChangeDetector::default(),
        state: LockState::default(),
        theme: theme::taskbar_theme(),
        lang: locale::ui_lang(),
        icon: ptr::null_mut(),
    };
    if let Some(state) = app.detector.observe(keyboard::read()) {
        app.state = state;
    }
    app.refresh_icon();
    // The tray may not exist yet (slow logon); TaskbarCreated re-adds it then.
    app.add_icon();
    APP.with(|cell| *cell.borrow_mut() = Some(app));

    // SAFETY: `hwnd` is our window; no timer callback, so WM_TIMER is posted.
    if unsafe { SetTimer(hwnd, TIMER_ID, POLL_INTERVAL.as_millis() as u32, None) } == 0 {
        return Err(io::Error::last_os_error());
    }

    // SAFETY: MSG is plain data; all-zero is a valid initial value.
    let mut msg: MSG = unsafe { mem::zeroed() };
    // SAFETY: `msg` is a valid out-pointer; null hwnd = all thread messages.
    // GetMessageW returns 0 on WM_QUIT and -1 on error.
    while unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) } > 0 {
        // SAFETY: `msg` was just filled by GetMessageW.
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}

impl App {
    fn notify_data(&self) -> NOTIFYICONDATAW {
        let mut data = NOTIFYICONDATAW {
            cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.hwnd,
            uID: ICON_ID,
            uFlags: NIF_ICON | NIF_TIP | NIF_MESSAGE | NIF_SHOWTIP,
            uCallbackMessage: WM_TRAY,
            hIcon: self.icon,
            // SAFETY: the rest are integers, arrays and a GUID; zero is valid.
            ..unsafe { mem::zeroed() }
        };
        let tip: Vec<u16> = self.state.tooltip(self.lang).encode_utf16().collect();
        let len = tip.len().min(data.szTip.len() - 1);
        data.szTip[..len].copy_from_slice(&tip[..len]);
        data
    }

    fn add_icon(&self) {
        let mut data = self.notify_data();
        // SAFETY: `data` is fully initialized and lives across the calls.
        unsafe {
            if Shell_NotifyIconW(NIM_ADD, &data) != 0 {
                data.Anonymous.uVersion = NOTIFYICON_VERSION_4;
                Shell_NotifyIconW(NIM_SETVERSION, &data);
            }
        }
    }

    fn update_icon(&self) {
        let data = self.notify_data();
        // SAFETY: `data` is fully initialized. Failure (tray gone) is fine:
        // TaskbarCreated re-adds the icon with the current state.
        unsafe { Shell_NotifyIconW(NIM_MODIFY, &data) };
    }

    fn remove_icon(&self) {
        let data = self.notify_data();
        // SAFETY: `data` identifies our icon by hWnd + uID.
        unsafe { Shell_NotifyIconW(NIM_DELETE, &data) };
    }

    /// Re-renders `self.icon` for the current state, theme and icon size.
    /// The size is re-read every time so DPI changes are picked up.
    fn refresh_icon(&mut self) {
        // SAFETY: no preconditions.
        let size = unsafe { GetSystemMetrics(SM_CXSMICON) }.clamp(16, 256) as u32;
        let Some(new_icon) = create_icon(self.state, self.theme, size) else {
            return; // keep the previous icon rather than showing none
        };
        let old = mem::replace(&mut self.icon, new_icon);
        if !old.is_null() {
            // SAFETY: `old` was created by CreateIcon and is no longer referenced.
            unsafe { DestroyIcon(old) };
        }
    }

    fn poll(&mut self) {
        if let Some(state) = self.detector.observe(keyboard::read()) {
            self.state = state;
            self.refresh_icon();
            self.update_icon();
        }
    }

    fn on_settings_changed(&mut self) {
        let theme = theme::taskbar_theme();
        if theme != self.theme {
            self.theme = theme;
            self.refresh_icon();
            self.update_icon();
        }
    }

    fn on_taskbar_created(&mut self) {
        self.refresh_icon();
        self.add_icon();
    }

    fn on_destroy(&mut self) {
        // SAFETY: the timer was set on this window; killing it twice is harmless.
        unsafe { KillTimer(self.hwnd, TIMER_ID) };
        self.remove_icon();
        if !self.icon.is_null() {
            // SAFETY: the icon was created by CreateIcon and the tray no longer uses it.
            unsafe { DestroyIcon(self.icon) };
            self.icon = ptr::null_mut();
        }
    }
}

/// Builds an `HICON` from [`icon::render`]'s straight-alpha RGBA.
fn create_icon(state: LockState, theme: Theme, size: u32) -> Option<HICON> {
    let rgba = icon::render(state, theme, size);
    // 32-bpp icon bits are BGRA, top-down.
    let bgra: Vec<u8> = rgba
        .chunks_exact(4)
        .flat_map(|p| [p[2], p[1], p[0], p[3]])
        .collect();
    // The AND mask is ignored when the color bits carry alpha, but must exist:
    // one bit per pixel, rows padded to 16 bits.
    let mask = vec![0u8; size.div_ceil(16) as usize * 2 * size as usize];
    let side = i32::try_from(size).ok()?;
    // SAFETY: `mask` and `bgra` hold exactly the bytes CreateIcon reads for a
    // `size`×`size` icon with 1 plane and 32 bits per pixel.
    let handle = unsafe {
        CreateIcon(
            GetModuleHandleW(ptr::null()),
            side,
            side,
            1,
            32,
            mask.as_ptr(),
            bgra.as_ptr(),
        )
    };
    (!handle.is_null()).then_some(handle)
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_TIMER if wparam == TIMER_ID => {
            with_app(App::poll);
            0
        }
        WM_SETTINGCHANGE => {
            with_app(App::on_settings_changed);
            0
        }
        WM_DESTROY => {
            with_app(App::on_destroy);
            // SAFETY: no preconditions; ends the message loop.
            unsafe { PostQuitMessage(0) };
            0
        }
        _ if msg != 0 && with_app(|app| app.taskbar_created == msg) == Some(true) => {
            with_app(App::on_taskbar_created);
            0
        }
        // SAFETY: forwarding the unmodified arguments we were called with.
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

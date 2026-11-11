//! The tray icon: a hidden window that owns the notification icon, polls the
//! lock keys on a timer and re-adds the icon when Explorer restarts.
//!
//! The window is a hidden top-level one, not `HWND_MESSAGE`: message-only
//! windows do not receive broadcasts such as `TaskbarCreated` and
//! `WM_SETTINGCHANGE`.

use std::cell::{Cell, RefCell};
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
    AppendMenuW, CreateIcon, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyIcon,
    DestroyMenu, DestroyWindow, DispatchMessageW, GetMessageW, HICON, KillTimer, MF_CHECKED,
    MF_SEPARATOR, MF_STRING, MF_UNCHECKED, MSG, PostMessageW, PostQuitMessage, RegisterClassExW,
    RegisterWindowMessageW, SetForegroundWindow, SetTimer, TPM_NONOTIFY, TPM_RETURNCMD,
    TPM_RIGHTBUTTON, TrackPopupMenu, TranslateMessage, WM_APP, WM_CONTEXTMENU, WM_DESTROY,
    WM_DISPLAYCHANGE, WM_DPICHANGED, WM_NULL, WM_SETTINGCHANGE, WM_TIMER, WNDCLASSEXW,
    WS_OVERLAPPED,
};

use super::autostart_registry::RegistryRunKey;
use super::{dpi, keyboard, locale, theme, wide};
use crate::{ChangeDetector, Lang, LockState, POLL_INTERVAL, Theme, autostart, icon};

const CLASS_NAME: &str = "caps-widget";
const ICON_ID: u32 = 1;
const TIMER_ID: usize = 1;
/// Notifications from the tray icon arrive as this message.
const WM_TRAY: u32 = WM_APP + 1;
const MENU_AUTOSTART: usize = 1;
const MENU_EXIT: usize = 2;

struct App {
    hwnd: HWND,
    detector: ChangeDetector,
    state: LockState,
    theme: Theme,
    lang: Lang,
    icon: HICON,
    icon_size: u32,
}

/// Window messages that act on the app state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    Tick,
    SettingsChanged,
    TaskbarCreated,
}

/// Events that arrived while the state was busy and must not be lost.
/// A missed tick needs no replay: the next one reads the keys again.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Pending {
    settings_changed: bool,
    taskbar_created: bool,
}

impl Pending {
    fn with(mut self, event: Event) -> Self {
        match event {
            Event::Tick => {}
            Event::SettingsChanged => self.settings_changed = true,
            Event::TaskbarCreated => self.taskbar_created = true,
        }
        self
    }
}

thread_local! {
    static APP: RefCell<Option<App>> = const { RefCell::new(None) };
    static PENDING: Cell<Pending> = Cell::new(Pending::default());
    /// Id of the `TaskbarCreated` broadcast; readable without borrowing `APP`.
    static TASKBAR_CREATED: Cell<u32> = const { Cell::new(0) };
}

/// Applies `event` to the state, first replaying any deferred events.
///
/// Win32 can re-enter the window procedure while the state is borrowed (e.g. a
/// broadcast dispatched during a synchronous Shell call). Then — or before the
/// state exists — the event is recorded in `pending` instead of being dropped
/// or panicking across the FFI boundary.
fn run_or_defer<T>(
    state: &RefCell<Option<T>>,
    pending: &Cell<Pending>,
    event: Event,
    mut apply: impl FnMut(&mut T, Event),
) {
    let Ok(mut guard) = state.try_borrow_mut() else {
        pending.set(pending.get().with(event));
        return;
    };
    let Some(state) = guard.as_mut() else {
        pending.set(pending.get().with(event));
        return;
    };
    let deferred = pending.take();
    if deferred.taskbar_created {
        apply(state, Event::TaskbarCreated);
    }
    if deferred.settings_changed {
        apply(state, Event::SettingsChanged);
    }
    apply(state, event);
}

fn handle(event: Event) {
    APP.with(|app| {
        PENDING.with(|pending| run_or_defer(app, pending, event, App::apply));
    });
}

/// Runs `f` on the app state, or returns `None` if it is busy or gone.
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
    TASKBAR_CREATED.with(|id| id.set(taskbar_created));

    let mut app = App {
        hwnd,
        detector: ChangeDetector::default(),
        state: LockState::default(),
        theme: theme::taskbar_theme(),
        lang: locale::ui_lang(),
        icon: ptr::null_mut(),
        icon_size: 0,
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

    /// Small-icon size for the window's current DPI. The hidden window sits at
    /// the origin, so this is the primary monitor's DPI — where the taskbar's
    /// notification area usually is.
    fn current_icon_size(&self) -> u32 {
        dpi::small_icon_size(dpi::window_dpi(self.hwnd))
    }

    /// Re-renders `self.icon` for the current state, theme and icon size.
    fn refresh_icon(&mut self) {
        let size = self.current_icon_size();
        let Some(new_icon) = create_icon(self.state, self.theme, size) else {
            return; // keep the previous icon rather than showing none
        };
        self.icon_size = size;
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

    /// Theme or display scale may have changed: redraw if either did.
    fn on_settings_changed(&mut self) {
        let theme = theme::taskbar_theme();
        if theme != self.theme || self.current_icon_size() != self.icon_size {
            self.theme = theme;
            self.refresh_icon();
            self.update_icon();
        }
    }

    fn apply(&mut self, event: Event) {
        match event {
            Event::Tick => self.poll(),
            Event::SettingsChanged => self.on_settings_changed(),
            Event::TaskbarCreated => self.on_taskbar_created(),
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

/// Shows the context menu at `(x, y)` and runs the chosen command.
///
/// Must not run inside [`with_app`]: the menu's modal loop keeps dispatching
/// timer ticks, and "Exit" destroys the window, both of which need the state.
fn show_menu(hwnd: HWND, x: i32, y: i32) {
    let strings = with_app(|app| app.lang).unwrap_or_default().strings();
    let mut store = RegistryRunKey::current_user_run();
    let exe = std::env::current_exe().ok();
    // The check mark always reflects the registry as it is now (R7, CA15).
    let enabled = exe
        .as_deref()
        .is_some_and(|exe| autostart::is_enabled(&store, exe).unwrap_or(false));

    let autostart_label = wide(strings.start_with_windows);
    let exit_label = wide(strings.exit);
    // SAFETY: the menu is created, used and destroyed within this function;
    // the label buffers outlive every call that reads them.
    let command = unsafe {
        let menu = CreatePopupMenu();
        if menu.is_null() {
            return;
        }
        let check = if enabled { MF_CHECKED } else { MF_UNCHECKED };
        AppendMenuW(
            menu,
            MF_STRING | check,
            MENU_AUTOSTART,
            autostart_label.as_ptr(),
        );
        AppendMenuW(menu, MF_SEPARATOR, 0, ptr::null());
        AppendMenuW(menu, MF_STRING, MENU_EXIT, exit_label.as_ptr());

        // Documented workaround so the menu closes when clicking elsewhere.
        SetForegroundWindow(hwnd);
        let command = TrackPopupMenu(
            menu,
            TPM_RIGHTBUTTON | TPM_RETURNCMD | TPM_NONOTIFY,
            x,
            y,
            0,
            hwnd,
            ptr::null(),
        );
        PostMessageW(hwnd, WM_NULL, 0, 0);
        DestroyMenu(menu);
        command
    };

    match usize::try_from(command) {
        Ok(MENU_AUTOSTART) => {
            if let Some(exe) = exe {
                // Errors are not fatal: the next menu re-reads the real state.
                let _ = autostart::set_enabled(&mut store, &exe, !enabled);
            }
        }
        Ok(MENU_EXIT) => {
            // SAFETY: `hwnd` is our window; WM_DESTROY removes the icon and
            // ends the message loop (R6).
            unsafe { DestroyWindow(hwnd) };
        }
        _ => {}
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        // With NOTIFYICON_VERSION_4 the event is LOWORD(lParam) and the anchor
        // point (mouse or keyboard) is packed in wParam.
        WM_TRAY if (lparam as u32 & 0xFFFF) == WM_CONTEXTMENU => {
            let x = i32::from((wparam & 0xFFFF) as u16 as i16);
            let y = i32::from(((wparam >> 16) & 0xFFFF) as u16 as i16);
            show_menu(hwnd, x, y);
            0
        }
        WM_TIMER if wparam == TIMER_ID => {
            handle(Event::Tick);
            0
        }
        // Theme switches arrive as WM_SETTINGCHANGE; scale changes as
        // WM_DPICHANGED / WM_DISPLAYCHANGE. All re-check theme and icon size.
        WM_SETTINGCHANGE | WM_DPICHANGED | WM_DISPLAYCHANGE => {
            handle(Event::SettingsChanged);
            0
        }
        WM_DESTROY => {
            with_app(App::on_destroy);
            // SAFETY: no preconditions; ends the message loop.
            unsafe { PostQuitMessage(0) };
            0
        }
        _ if msg != 0 && msg == TASKBAR_CREATED.with(Cell::get) => {
            handle(Event::TaskbarCreated);
            0
        }
        // SAFETY: forwarding the unmodified arguments we were called with.
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

    use super::{Event, Pending, run_or_defer};

    fn record(log: &mut Vec<Event>, event: Event) {
        log.push(event);
    }

    #[test]
    fn busy_state_defers_settings_change_and_replays_it() {
        let state = RefCell::new(Some(Vec::new()));
        let pending = Cell::new(Pending::default());

        let busy = state.borrow_mut();
        run_or_defer(&state, &pending, Event::SettingsChanged, record);
        drop(busy);
        run_or_defer(&state, &pending, Event::Tick, record);

        assert_eq!(
            state.borrow().as_deref(),
            Some(&[Event::SettingsChanged, Event::Tick][..])
        );
        assert_eq!(pending.get(), Pending::default());
    }

    #[test]
    fn busy_state_defers_taskbar_created_and_replays_it() {
        let state = RefCell::new(Some(Vec::new()));
        let pending = Cell::new(Pending::default());

        let busy = state.borrow_mut();
        run_or_defer(&state, &pending, Event::TaskbarCreated, record);
        drop(busy);
        run_or_defer(&state, &pending, Event::Tick, record);

        assert_eq!(
            state.borrow().as_deref(),
            Some(&[Event::TaskbarCreated, Event::Tick][..])
        );
    }

    #[test]
    fn missed_tick_is_not_replayed() {
        let state = RefCell::new(Some(Vec::new()));
        let pending = Cell::new(Pending::default());

        let busy = state.borrow_mut();
        run_or_defer(&state, &pending, Event::Tick, record);
        drop(busy);

        assert_eq!(pending.get(), Pending::default());
        assert_eq!(state.borrow().as_deref(), Some(&[][..]));
    }
}

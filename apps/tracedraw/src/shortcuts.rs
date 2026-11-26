//! Keyboard shortcuts as text ("Ctrl+Shift+E", "F5", "Alt+Enter"), the
//! way the menus show them and the preferences keep them
//! (`Settings::shortcuts`: a tool id or a menu item's key, and its
//! shortcut). Menu commands with a shortcut of the user's run through
//! `ui::menus::run_command`.
//!
//! Every key can be a shortcut: the keys egui reports ([`Key`]), the keys
//! it does not (Pause, Print Screen, the keypad, media and browser keys:
//! [`Extra`], read by `extra_keys.rs`), and Shift, Ctrl, Alt or the
//! Windows key pressed and released alone ([`Tap`]).

use egui::{Key, Modifiers};

/// A key egui does not report, read from the system (Windows).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extra {
    Pause,
    CapsLock,
    PrintScreen,
    ScrollLock,
    NumLock,
    /// The context menu (Application) key.
    Menu,
    Numpad(u8),
    NumpadMultiply,
    NumpadAdd,
    NumpadSubtract,
    NumpadDecimal,
    NumpadDivide,
    /// The keypad 5 with Num Lock off.
    Clear,
    Help,
    Select,
    Execute,
    Sleep,
    BrowserForward,
    BrowserRefresh,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserHome,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    MediaNext,
    MediaPrevious,
    MediaStop,
    MediaPlayPause,
    LaunchMail,
    LaunchMedia,
    LaunchApp1,
    LaunchApp2,
    /// Input method keys of Asian layouts.
    Convert,
    NonConvert,
    Kana,
    Kanji,
}

impl Extra {
    /// Every extra key with its Windows virtual-key code.
    pub const ALL: [(Extra, u8); 47] = [
        (Extra::Pause, 0x13),
        (Extra::CapsLock, 0x14),
        (Extra::PrintScreen, 0x2C),
        (Extra::ScrollLock, 0x91),
        (Extra::NumLock, 0x90),
        (Extra::Menu, 0x5D),
        (Extra::Numpad(0), 0x60),
        (Extra::Numpad(1), 0x61),
        (Extra::Numpad(2), 0x62),
        (Extra::Numpad(3), 0x63),
        (Extra::Numpad(4), 0x64),
        (Extra::Numpad(5), 0x65),
        (Extra::Numpad(6), 0x66),
        (Extra::Numpad(7), 0x67),
        (Extra::Numpad(8), 0x68),
        (Extra::Numpad(9), 0x69),
        (Extra::NumpadMultiply, 0x6A),
        (Extra::NumpadAdd, 0x6B),
        (Extra::NumpadSubtract, 0x6D),
        (Extra::NumpadDecimal, 0x6E),
        (Extra::NumpadDivide, 0x6F),
        (Extra::Clear, 0x0C),
        (Extra::Help, 0x2F),
        (Extra::Select, 0x29),
        (Extra::Execute, 0x2B),
        (Extra::Sleep, 0x5F),
        (Extra::BrowserForward, 0xA7),
        (Extra::BrowserRefresh, 0xA8),
        (Extra::BrowserStop, 0xA9),
        (Extra::BrowserSearch, 0xAA),
        (Extra::BrowserFavorites, 0xAB),
        (Extra::BrowserHome, 0xAC),
        (Extra::VolumeMute, 0xAD),
        (Extra::VolumeDown, 0xAE),
        (Extra::VolumeUp, 0xAF),
        (Extra::MediaNext, 0xB0),
        (Extra::MediaPrevious, 0xB1),
        (Extra::MediaStop, 0xB2),
        (Extra::MediaPlayPause, 0xB3),
        (Extra::LaunchMail, 0xB4),
        (Extra::LaunchMedia, 0xB5),
        (Extra::LaunchApp1, 0xB6),
        (Extra::LaunchApp2, 0xB7),
        (Extra::Convert, 0x1C),
        (Extra::NonConvert, 0x1D),
        (Extra::Kana, 0x15),
        (Extra::Kanji, 0x19),
    ];

    /// Its name in shortcut text.
    pub fn name(self) -> String {
        match self {
            Extra::Numpad(n) => format!("Numpad{n}"),
            other => format!("{other:?}"),
        }
    }

    pub fn from_name(name: &str) -> Option<Extra> {
        let lower = name.to_ascii_lowercase();
        Extra::ALL
            .iter()
            .map(|(e, _)| *e)
            .find(|e| e.name().to_ascii_lowercase() == lower)
    }

    /// The key egui reports for it too (the keypad's digits and signs), so
    /// a shortcut on the keypad key does not also run the plain key's.
    pub fn twin(self) -> Option<Key> {
        Some(match self {
            Extra::Numpad(n) => Key::from_name(&n.to_string())?,
            Extra::NumpadAdd => Key::Plus,
            Extra::NumpadSubtract => Key::Minus,
            Extra::NumpadDecimal => Key::Period,
            Extra::NumpadDivide => Key::Slash,
            _ => return None,
        })
    }
}

/// Shift, Ctrl, Alt or the Windows (Super) key pressed and released with
/// no other key between.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tap {
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    LWin,
    RWin,
}

impl Tap {
    pub const ALL: [Tap; 8] = [
        Tap::LShift,
        Tap::RShift,
        Tap::LCtrl,
        Tap::RCtrl,
        Tap::LAlt,
        Tap::RAlt,
        Tap::LWin,
        Tap::RWin,
    ];

    pub fn name(self) -> String {
        format!("{self:?}")
    }

    pub fn from_name(name: &str) -> Option<Tap> {
        let lower = name.to_ascii_lowercase();
        Tap::ALL
            .into_iter()
            .find(|t| t.name().to_ascii_lowercase() == lower)
    }

    /// The tap of a modifier key egui reports.
    pub fn of(key: Key) -> Option<Tap> {
        Some(match key {
            Key::ShiftLeft => Tap::LShift,
            Key::ShiftRight => Tap::RShift,
            Key::ControlLeft => Tap::LCtrl,
            Key::ControlRight => Tap::RCtrl,
            Key::AltLeft => Tap::LAlt,
            Key::AltRight => Tap::RAlt,
            Key::SuperLeft => Tap::LWin,
            Key::SuperRight => Tap::RWin,
            _ => return None,
        })
    }

    /// The modifiers without the tapped key's own (holding Ctrl and
    /// tapping Shift is Ctrl+LShift).
    pub fn without_own(self, mut mods: Modifiers) -> Modifiers {
        match self {
            Tap::LShift | Tap::RShift => mods.shift = false,
            Tap::LCtrl | Tap::RCtrl => {
                mods.ctrl = false;
                mods.command = false;
            }
            Tap::LAlt | Tap::RAlt => mods.alt = false,
            Tap::LWin | Tap::RWin => mods.mac_cmd = false,
        }
        mods
    }
}

/// Any key a shortcut can end with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    Key(Key),
    Extra(Extra),
    Tap(Tap),
}

impl Code {
    pub fn name(self) -> String {
        match self {
            Code::Key(k) => key_name(k),
            Code::Extra(e) => e.name(),
            Code::Tap(t) => t.name(),
        }
    }
}

/// The name a key goes by in shortcut text.
pub fn key_name(key: Key) -> String {
    match key {
        Key::PageUp => "PgUp".into(),
        Key::PageDown => "PgDn".into(),
        Key::Plus => "+".into(),
        Key::Minus => "-".into(),
        Key::Equals => "=".into(),
        Key::Escape => "Esc".into(),
        Key::Semicolon => ";".into(),
        Key::Quote => "'".into(),
        Key::OpenBracket => "[".into(),
        Key::CloseBracket => "]".into(),
        Key::ArrowLeft => "Left".into(),
        Key::ArrowRight => "Right".into(),
        Key::ArrowUp => "Up".into(),
        Key::ArrowDown => "Down".into(),
        other => other.name().to_string(),
    }
}

fn key_from_name(name: &str) -> Option<Key> {
    Some(match name {
        "PgUp" => Key::PageUp,
        "PgDn" => Key::PageDown,
        "+" => Key::Plus,
        "-" => Key::Minus,
        "=" => Key::Equals,
        "Esc" => Key::Escape,
        "Left" => Key::ArrowLeft,
        "Right" => Key::ArrowRight,
        "Up" => Key::ArrowUp,
        "Down" => Key::ArrowDown,
        // Keypad keys and modifier taps are not the keys egui would
        // read for those names ("Numpad1" is not "1").
        other if Extra::from_name(other).is_some() || Tap::from_name(other).is_some() => {
            return None
        }
        other => {
            return Key::from_name(other).or_else(|| Key::from_name(&other.to_ascii_uppercase()))
        }
    })
}

/// Shortcut text: Ctrl, Alt and Shift in that order, then the key.
pub fn format(mods: Modifiers, key: Key) -> String {
    format_code(mods, Code::Key(key))
}

/// Shortcut text for any key.
pub fn format_code(mods: Modifiers, code: Code) -> String {
    let mut s = String::new();
    if mods.command || mods.ctrl {
        s.push_str("Ctrl+");
    }
    if mods.alt {
        s.push_str("Alt+");
    }
    if mods.shift {
        s.push_str("Shift+");
    }
    s.push_str(&code.name());
    s
}

/// The modifiers and key of shortcut text; `None` when it names no key
/// egui reports (see [`parse_code`] for every key).
pub fn parse(text: &str) -> Option<(Modifiers, Key)> {
    match parse_code(text)? {
        (m, Code::Key(k)) => Some((m, k)),
        _ => None,
    }
}

/// The modifiers and key of shortcut text, any key.
pub fn parse_code(text: &str) -> Option<(Modifiers, Code)> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    // "Ctrl++": the key itself is a plus.
    let (mods_part, key_part) = match text.strip_suffix("++") {
        Some(rest) => (rest, "+"),
        None => match text.rfind('+') {
            Some(i) if i + 1 < text.len() => (&text[..i], &text[i + 1..]),
            _ => ("", text),
        },
    };
    let mut mods = Modifiers::NONE;
    for m in mods_part
        .split('+')
        .map(str::trim)
        .filter(|m| !m.is_empty())
    {
        match m.to_ascii_lowercase().as_str() {
            "ctrl" | "cmd" | "command" => mods |= Modifiers::COMMAND,
            "alt" | "option" => mods.alt = true,
            "shift" => mods.shift = true,
            _ => return None,
        }
    }
    let name = key_part.trim();
    let code = match key_from_name(name) {
        Some(k) => Code::Key(k),
        None => match Extra::from_name(name) {
            Some(e) => Code::Extra(e),
            None => Code::Tap(Tap::from_name(name)?),
        },
    };
    Some((mods, code))
}

/// Whether two shortcut texts name the same keys.
pub fn same(a: &str, b: &str) -> bool {
    match (parse_code(a), parse_code(b)) {
        (Some((ma, ka)), Some((mb, kb))) => {
            ka == kb
                && (ma.command || ma.ctrl) == (mb.command || mb.ctrl)
                && ma.alt == mb.alt
                && ma.shift == mb.shift
        }
        _ => false,
    }
}

/// The keys that run `id` (a menu command's key or a tool's id): the
/// user's keys for it when there are any (a command may have several,
/// #822), else its `built_in` keys. An empty entry of the user's means
/// the built-in keys were deleted.
pub fn keys_for(user: &[(String, String)], id: &str, built_in: Option<&str>) -> Vec<String> {
    let mut mine = user.iter().filter(|(k, _)| k == id).peekable();
    if mine.peek().is_none() {
        return built_in
            .filter(|b| !b.is_empty())
            .map(|b| vec![b.to_string()])
            .unwrap_or_default();
    }
    mine.filter(|(_, v)| !v.is_empty())
        .map(|(_, v)| v.clone())
        .collect()
}

/// The editor's shortcuts a browser may keep for itself (#1409), with the
/// key that runs the same command in the browser version: `None` where
/// the command needs none there (Exit: close the tab). The page keeps
/// every Ctrl key it can from the browser (`web_page`); these are the
/// ones browsers never hand to a page (Ctrl+N, Ctrl+T, Ctrl+W,
/// Ctrl+Shift+T, Ctrl+F4, Alt+F4) or keep in some browsers or systems
/// (Ctrl+PgUp/PgDn and Ctrl+1 to Ctrl+9 switch tabs, Ctrl+Q and
/// Ctrl+Shift+Q quit on Linux). Each alternative is a combination the
/// editor binds to nothing else and the browsers leave to pages
/// (`tests::browser_alternatives_are_free`).
pub const BROWSER_KEYS: &[(&str, Option<&str>)] = &[
    ("Ctrl+N", Some("Alt+Shift+N")),
    ("Ctrl+T", Some("Alt+Shift+X")),
    ("Ctrl+Shift+T", Some("Alt+Shift+T")),
    ("Ctrl+W", Some("Ctrl+F5")),
    ("Ctrl+F4", Some("Alt+Shift+W")),
    ("Alt+F4", None),
    ("Ctrl+PgUp", Some("Alt+PgUp")),
    ("Ctrl+PgDn", Some("Alt+PgDn")),
    ("Ctrl+Q", Some("Alt+Shift+C")),
    ("Ctrl+Shift+Q", Some("Alt+Shift+Q")),
    ("Ctrl+0", Some("Alt+Shift+0")),
    ("Ctrl+1", Some("Alt+Shift+1")),
    ("Ctrl+2", Some("Alt+Shift+2")),
    ("Ctrl+3", Some("Alt+Shift+3")),
    ("Ctrl+4", Some("Alt+Shift+4")),
    ("Ctrl+5", Some("Alt+Shift+5")),
    ("Ctrl+6", Some("Alt+Shift+6")),
    ("Ctrl+7", Some("Alt+Shift+7")),
    ("Ctrl+8", Some("Alt+Shift+8")),
    ("Ctrl+9", Some("Alt+Shift+9")),
];

/// The browser version's key for a command whose `shortcut` the browser
/// may keep ([`BROWSER_KEYS`]); `None` for any other shortcut, or one
/// with no alternative.
pub fn browser_alternative(shortcut: &str) -> Option<&'static str> {
    BROWSER_KEYS
        .iter()
        .find(|(k, _)| same(k, shortcut))
        .and_then(|(_, alt)| *alt)
}

#[cfg(test)]
thread_local! {
    /// Tests switch the browser version's shortcuts on in a desktop build.
    static WEB_KEYS_FOR_TEST: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Whether the browser version's shortcuts apply: in the browser build,
/// and in tests that ask for them.
pub fn web_keys() -> bool {
    #[cfg(test)]
    if WEB_KEYS_FOR_TEST.with(|w| w.get()) {
        return true;
    }
    crate::files::WEB
}

#[cfg(test)]
pub fn set_web_keys_for_test(on: bool) {
    WEB_KEYS_FOR_TEST.with(|w| w.set(on));
}

/// The Command keys macOS keeps for itself (#1409), in shortcut text
/// (`Ctrl` is the Command key there): Quit and Log Out (Command+Q,
/// Command+Shift+Q), Hide and Hide Others (Command+H, Command+Option+H),
/// Minimize (Command+M), the application switcher (Command+Tab,
/// Command+Shift+Tab), Spotlight and the Finder's search
/// (Command+Space, Command+Option+Space), Force Quit
/// (Command+Option+Esc), the screenshots (Command+Shift+3, 4 and 5) and
/// hiding the Dock (Command+Option+D). The application menu or the
/// system takes them before the window sees them.
pub const MAC_SYSTEM_KEYS: &[&str] = &[
    "Ctrl+Q",
    "Ctrl+Shift+Q",
    "Ctrl+H",
    "Ctrl+Alt+H",
    "Ctrl+M",
    "Ctrl+Tab",
    "Ctrl+Shift+Tab",
    "Ctrl+Space",
    "Ctrl+Alt+Space",
    "Ctrl+Alt+Esc",
    "Ctrl+Shift+3",
    "Ctrl+Shift+4",
    "Ctrl+Shift+5",
    "Ctrl+Alt+D",
];

/// The editor's own keys among [`MAC_SYSTEM_KEYS`], with the key that
/// runs the same command in the macOS desktop app (#1409): Convert to
/// Curves, Convert Outline to Object, Merge Cells, the Pick tool from any
/// tool and Force Justify while typing. Each alternative is bound to
/// nothing else (`ui::options::tests::no_two_default_commands_share_a_shortcut`
/// in its macOS run). Quit keeps Command+Q ([`MAC_QUIT`]).
pub const MAC_KEYS: &[(&str, &str)] = &[
    ("Ctrl+Q", "Alt+Shift+C"),
    ("Ctrl+Shift+Q", "Alt+Shift+Q"),
    ("Ctrl+M", "Ctrl+Shift+M"),
    ("Ctrl+Space", "Ctrl+Shift+Space"),
    ("Ctrl+H", "Ctrl+Shift+H"),
];

/// File > Exit's key in the macOS desktop app: Command+Q, the application
/// menu's Quit, which asks about unsaved drawings as Exit does (#1669).
pub const MAC_QUIT: &str = "Ctrl+Q";

/// File > Exit's key on the other systems.
const EXIT_KEY: &str = "Alt+F4";

#[cfg(test)]
thread_local! {
    /// Tests switch the macOS desktop app's shortcuts on elsewhere.
    static MAC_KEYS_FOR_TEST: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Whether the macOS desktop app's shortcuts apply: in the desktop build
/// for macOS, and in tests that ask for them.
pub fn mac_keys() -> bool {
    #[cfg(test)]
    if MAC_KEYS_FOR_TEST.with(|w| w.get()) {
        return true;
    }
    cfg!(target_os = "macos") && !crate::files::WEB
}

#[cfg(test)]
pub fn set_mac_keys_for_test(on: bool) {
    MAC_KEYS_FOR_TEST.with(|w| w.set(on));
}

/// Whether `shortcut` is one of the Command keys macOS keeps
/// ([`MAC_SYSTEM_KEYS`]).
pub fn mac_system_key(shortcut: &str) -> bool {
    MAC_SYSTEM_KEYS.iter().any(|k| same(k, shortcut))
}

/// The macOS desktop app's key for a command whose `shortcut` macOS
/// keeps ([`MAC_KEYS`]); `None` for any other shortcut.
pub fn mac_alternative(shortcut: &str) -> Option<&'static str> {
    MAC_KEYS
        .iter()
        .find(|(k, _)| same(k, shortcut))
        .map(|(_, alt)| *alt)
}

/// A key press in the macOS desktop app: the editor shortcut it stands
/// for when it is the alternative of a key macOS keeps ([`MAC_KEYS`]);
/// `None` in other builds and for other keys.
pub fn mac_key_for(key: Key, mods: Modifiers) -> Option<(Modifiers, Key)> {
    if !mac_keys() {
        return None;
    }
    let text = format(mods, key);
    MAC_KEYS
        .iter()
        .find(|(_, alt)| same(alt, &text))
        .and_then(|(k, _)| parse(k))
}

/// Whether a key press is one macOS keeps, in the macOS desktop app: the
/// window does not act on it should it arrive.
pub fn mac_kept_press(key: Key, mods: Modifiers) -> bool {
    mac_keys() && mac_system_key(&format(mods, key))
}

/// The shortcut a menu row shows: in the browser version the alternative
/// of a key the browser keeps, in the macOS desktop app the alternative
/// of a key macOS keeps (and Exit's Command+Q), else `shortcut` itself.
pub fn shown(shortcut: String) -> String {
    if web_keys() {
        return browser_alternative(&shortcut)
            .map(str::to_string)
            .unwrap_or(shortcut);
    }
    mac_shown(shortcut)
}

/// The key a command's built-in `shortcut` stands for in the macOS
/// desktop app: the alternative of a key macOS keeps, Exit's Command+Q
/// ([`MAC_QUIT`]), else `shortcut` itself. Other builds keep `shortcut`.
/// The menus and Options > Customization > Commands show it there.
pub fn mac_shown(shortcut: String) -> String {
    if !mac_keys() {
        return shortcut;
    }
    if same(&shortcut, EXIT_KEY) {
        return MAC_QUIT.to_string();
    }
    mac_alternative(&shortcut)
        .map(str::to_string)
        .unwrap_or(shortcut)
}

/// A tip that names its shortcut in brackets ("New (Ctrl+N)", "(Ctrl+Q
/// converts to curves)"): in the browser version the alternative of a
/// key the browser keeps takes its place ("New (Alt+Shift+N)"), in the
/// macOS desktop app the alternative of a key macOS keeps.
pub fn shown_in(text: String) -> String {
    let swap = |t: String, k: &str, alt: &str| {
        t.replace(&format!("({k})"), &format!("({alt})"))
            .replace(&format!("({k} "), &format!("({alt} "))
    };
    if web_keys() {
        return BROWSER_KEYS.iter().fold(text, |t, (k, alt)| match alt {
            Some(alt) => swap(t, k, alt),
            None => t,
        });
    }
    if mac_keys() {
        let text = swap(text, EXIT_KEY, MAC_QUIT);
        return MAC_KEYS.iter().fold(text, |t, (k, alt)| swap(t, k, alt));
    }
    text
}

/// The browser's keys and their alternatives as the Options notice lists
/// them: "Ctrl+N: Alt+Shift+N, ...", with the digits in one entry.
pub fn browser_keys_list() -> String {
    let mut out: Vec<String> = Vec::new();
    let mut digits = false;
    for (k, alt) in BROWSER_KEYS {
        let Some(alt) = alt else { continue };
        let digit = k
            .strip_prefix("Ctrl+")
            .is_some_and(|d| d.chars().all(|c| c.is_ascii_digit()));
        if digit {
            if !digits {
                out.push("Ctrl+0..9: Alt+Shift+0..9".into());
                digits = true;
            }
            continue;
        }
        out.push(format!("{k}: {alt}"));
    }
    out.join(", ")
}

/// The editor shortcut a key press stands for in the browser version: the
/// key the browser keeps when the press is its alternative. The digits
/// are read from the physical key, since Shift turns them into other
/// characters (`!`, `@`, `§`) depending on the keyboard layout.
pub fn browser_key_for(
    key: Key,
    physical: Option<Key>,
    mods: Modifiers,
) -> Option<(Modifiers, Key)> {
    let digit = |k: Key| {
        matches!(
            k,
            Key::Num0
                | Key::Num1
                | Key::Num2
                | Key::Num3
                | Key::Num4
                | Key::Num5
                | Key::Num6
                | Key::Num7
                | Key::Num8
                | Key::Num9
        )
    };
    let key = physical.filter(|p| digit(*p)).unwrap_or(key);
    let text = format(mods, key);
    BROWSER_KEYS
        .iter()
        .find(|(_, alt)| alt.is_some_and(|a| same(a, &text)))
        .and_then(|(k, _)| parse(k))
}

/// Shift, Ctrl, Alt and the system key pressed on their own (they arrive
/// as key presses too, before the key they modify).
pub fn is_modifier(key: Key) -> bool {
    matches!(
        key,
        Key::ShiftLeft
            | Key::ShiftRight
            | Key::ControlLeft
            | Key::ControlRight
            | Key::AltLeft
            | Key::AltRight
            | Key::SuperLeft
            | Key::SuperRight
    )
}

/// Whether a key press can be a shortcut: every key can, Esc, Tab,
/// Enter, Backspace and Space alone too. A modifier's press is not one
/// (it waits for the key it modifies); its tap alone is ([`Tap`]).
pub fn allowed(_mods: Modifiers, key: Key) -> bool {
    !is_modifier(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcut_text_round_trips() {
        let ctrl_shift = Modifiers::COMMAND | Modifiers::SHIFT;
        assert_eq!(format(ctrl_shift, Key::E), "Ctrl+Shift+E");
        assert_eq!(format(Modifiers::ALT, Key::Enter), "Alt+Enter");
        assert_eq!(format(Modifiers::COMMAND, Key::PageDown), "Ctrl+PgDn");
        assert_eq!(format(Modifiers::NONE, Key::F5), "F5");
        for text in [
            "Ctrl+Shift+E",
            "Alt+Enter",
            "Ctrl+PgDn",
            "F5",
            "Ctrl++",
            "Alt+Shift+A",
            "Shift+F12",
        ] {
            let (m, k) = parse(text).expect(text);
            assert_eq!(format(m, k), text);
        }
        assert_eq!(parse("Ctrl++").map(|(_, k)| k), Some(Key::Plus));
        assert!(same("ctrl+shift+e", "Ctrl+Shift+E"));
        assert!(!same("Ctrl+E", "Ctrl+Shift+E"));
        assert!(
            parse("Hyper+E").is_none() && parse("").is_none() && parse("Ctrl+Nothing").is_none()
        );
        // Every key, Esc and Space alone too.
        assert!(allowed(Modifiers::NONE, Key::Escape) && allowed(Modifiers::NONE, Key::Space));
        assert!(allowed(Modifiers::COMMAND, Key::Enter) && allowed(Modifiers::NONE, Key::F9));
        assert!(
            !allowed(Modifiers::ALT, Key::AltLeft)
                && !allowed(Modifiers::COMMAND, Key::ControlRight)
        );
    }

    #[test]
    fn keys_egui_does_not_report_and_taps_are_shortcuts_too() {
        for text in [
            "Pause",
            "Ctrl+PrintScreen",
            "Numpad7",
            "Alt+NumpadAdd",
            "VolumeUp",
            "MediaPlayPause",
            "LShift",
            "Ctrl+RAlt",
            "LWin",
            "Shift+ScrollLock",
        ] {
            let (m, c) = parse_code(text).expect(text);
            assert_eq!(format_code(m, c), text);
        }
        // The keypad's 1 is not the 1 above the letters.
        assert!(parse("Numpad1").is_none());
        assert_eq!(
            parse_code("Numpad1").map(|x| x.1),
            Some(Code::Extra(Extra::Numpad(1)))
        );
        assert!(!same("Numpad1", "1") && same("numpad1", "Numpad1"));
        assert!(same("lshift", "LShift") && !same("LShift", "RShift"));
        assert_eq!(Extra::Numpad(4).twin(), Some(Key::Num4));
        assert_eq!(Extra::NumpadAdd.twin(), Some(Key::Plus));
        assert_eq!(Extra::Pause.twin(), None);
        assert_eq!(
            Tap::LCtrl.without_own(Modifiers::COMMAND | Modifiers::SHIFT),
            Modifiers::SHIFT
        );
        // Every extra key's name reads back.
        for (e, _) in Extra::ALL {
            assert_eq!(Extra::from_name(&e.name()), Some(e));
        }
    }

    /// Whether `shortcut` may stay with the browser in the browser version.
    fn browser_keeps(shortcut: &str) -> bool {
        BROWSER_KEYS.iter().any(|(k, _)| same(k, shortcut))
    }

    /// #1409: the keys browsers keep map to the browser version's keys,
    /// and those keys back to the command's own shortcut.
    #[test]
    fn browser_kept_keys_map_to_alternatives() {
        assert_eq!(browser_alternative("Ctrl+N"), Some("Alt+Shift+N"));
        assert_eq!(browser_alternative("ctrl+w"), Some("Ctrl+F5"));
        assert_eq!(browser_alternative("Ctrl+PgUp"), Some("Alt+PgUp"));
        assert_eq!(browser_alternative("Ctrl+PgDn"), Some("Alt+PgDn"));
        assert_eq!(browser_alternative("Ctrl+F4"), Some("Alt+Shift+W"));
        assert_eq!(browser_alternative("Ctrl+3"), Some("Alt+Shift+3"));
        // Exit needs none (close the tab); keys the page keeps need none.
        assert!(browser_keeps("Alt+F4") && browser_alternative("Alt+F4").is_none());
        for k in [
            "Ctrl+E",
            "Ctrl+R",
            "F5",
            "Ctrl+Home",
            "Shift+PgUp",
            "Ctrl+Shift+N",
        ] {
            assert!(!browser_keeps(k) && browser_alternative(k).is_none(), "{k}");
        }
        // Each alternative is given once and keeps none of the taken keys.
        let alts: Vec<&str> = BROWSER_KEYS.iter().filter_map(|(_, a)| *a).collect();
        for (i, a) in alts.iter().enumerate() {
            assert!(parse(a).is_some(), "{a}");
            assert!(!browser_keeps(a), "{a}");
            assert!(!alts[..i].iter().any(|b| same(a, b)), "{a}");
        }
        // Pressed in the browser, the alternative stands for the key the
        // browser keeps; the digits by their physical key (Shift+1 is `!`
        // on many layouts).
        assert_eq!(
            browser_key_for(Key::PageUp, None, Modifiers::ALT),
            Some((Modifiers::COMMAND, Key::PageUp))
        );
        assert_eq!(
            browser_key_for(Key::N, Some(Key::N), Modifiers::ALT | Modifiers::SHIFT),
            Some((Modifiers::COMMAND, Key::N))
        );
        assert_eq!(
            browser_key_for(Key::F5, None, Modifiers::COMMAND),
            Some((Modifiers::COMMAND, Key::W))
        );
        assert_eq!(
            browser_key_for(
                Key::Exclamationmark,
                Some(Key::Num1),
                Modifiers::ALT | Modifiers::SHIFT
            ),
            Some((Modifiers::COMMAND, Key::Num1))
        );
        assert_eq!(browser_key_for(Key::PageUp, None, Modifiers::COMMAND), None);
        assert_eq!(browser_key_for(Key::E, None, Modifiers::ALT), None);
        // The menus show the alternative in the browser version only.
        set_web_keys_for_test(false);
        assert_eq!(shown("Ctrl+PgUp".into()), "Ctrl+PgUp");
        set_web_keys_for_test(true);
        assert_eq!(shown("Ctrl+PgUp".into()), "Alt+PgUp");
        assert_eq!(shown("Alt+F4".into()), "Alt+F4");
        assert_eq!(shown("Ctrl+E".into()), "Ctrl+E");
        assert_eq!(shown_in("New (Ctrl+N)".into()), "New (Alt+Shift+N)");
        assert_eq!(
            shown_in("Edit Text (Ctrl+Shift+T)".into()),
            "Edit Text (Alt+Shift+T)"
        );
        assert_eq!(shown_in("Text (Ctrl+T)".into()), "Text (Alt+Shift+X)");
        assert_eq!(shown_in("Export (Ctrl+E)".into()), "Export (Ctrl+E)");
        set_web_keys_for_test(false);
        assert_eq!(shown_in("New (Ctrl+N)".into()), "New (Ctrl+N)");
        assert_eq!(
            browser_keys_list(),
            "Ctrl+N: Alt+Shift+N, Ctrl+T: Alt+Shift+X, Ctrl+Shift+T: Alt+Shift+T, \
             Ctrl+W: Ctrl+F5, Ctrl+F4: Alt+Shift+W, Ctrl+PgUp: Alt+PgUp, \
             Ctrl+PgDn: Alt+PgDn, Ctrl+Q: Alt+Shift+C, Ctrl+Shift+Q: Alt+Shift+Q, \
             Ctrl+0..9: Alt+Shift+0..9"
        );
    }
}

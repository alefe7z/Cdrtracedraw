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
}

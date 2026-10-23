//! The keys shortcuts can use beyond what `App::keyboard` reads: the keys
//! egui does not report (Pause, Print Screen, Scroll Lock, the keypad as
//! its own keys, media and browser keys: [`Extra`]), read from Windows
//! each frame, and Shift, Ctrl, Alt or the Windows key tapped alone
//! ([`Tap`]). It also takes the keys of a command waiting for its
//! shortcut (Options > Customization > Commands), any key at all, Esc
//! included, before the dialog's own Esc handling sees them.
//!
//! `ui::show` calls [`frame`] once a frame, before `App::keyboard`; a
//! shortcut found here runs its menu command (`App::pending_command`) or
//! picks its tool, and the keys it used are taken out of the input.
//! See docs/behavior/shortcuts.md, "Any key".

use crate::app::App;
use crate::shortcuts::{self, Code, Extra, Tap};
use egui::{Context, Event, Key, Modifiers};
use std::cell::{Cell, RefCell};

thread_local! {
    /// The extra keys held down at the last look, by virtual-key code.
    static HELD: RefCell<[bool; 256]> = const { RefCell::new([false; 256]) };
    /// A modifier pressed with no other key since: its tap on release.
    static PENDING_TAP: Cell<Option<Tap>> = const { Cell::new(None) };
}

/// One frame: the key presses this module reads, then the waiting
/// command's capture or the user's shortcuts.
pub fn frame(app: &mut App, ctx: &Context) {
    let mods = ctx.input(|i| i.modifiers);
    let mut presses: Vec<(Modifiers, Code)> = poll(ctx)
        .into_iter()
        .map(|e| (mods, Code::Extra(e)))
        .collect();
    let events = ctx.input(|i| i.events.clone());
    presses.extend(taps(&events).into_iter().map(|(m, t)| (m, Code::Tap(t))));

    if app.shortcut_editor.capture.is_some() {
        capture(app, ctx, &presses, &events);
        return;
    }
    if presses.is_empty() || blocked(app, ctx) {
        return;
    }
    for (m, code) in presses {
        let text = shortcuts::format_code(m, code);
        if run(app, &text) {
            if let Code::Extra(e) = code {
                // The keypad key's twin (its digit) is not also a press.
                if let Some(twin) = e.twin() {
                    take_key(ctx, twin);
                }
            }
            return;
        }
    }
}

/// Run the command or pick the tool the user gave `text`; false when it
/// is nobody's shortcut.
fn run(app: &mut App, text: &str) -> bool {
    let Some(id) = app
        .settings
        .shortcuts
        .iter()
        .find(|(_, v)| !v.is_empty() && shortcuts::same(v, text))
        .map(|(id, _)| id.clone())
    else {
        return false;
    };
    match crate::tools::Tool::ALL.iter().find(|t| t.id() == id) {
        Some(t) => app.set_tool(*t),
        None => app.pending_command = Some(id),
    }
    true
}

/// When the keyboard belongs to something else, as in `App::keyboard`:
/// a macro's form, a dialog, a field, text being typed, an open menu.
fn blocked(app: &App, ctx: &Context) -> bool {
    app.macros.modal()
        || !matches!(app.dialog, crate::ui::dialogs::Dialog::None)
        || ctx.egui_wants_keyboard_input()
        || app.text_edit.is_some()
        || app.table_edit.is_some()
        || egui::Popup::is_any_open(ctx)
}

/// A command waiting for its keys (only while Options shows the
/// Commands page) takes the first key pressed this frame: an extra key or
/// a tap, else any key egui reports with the modifiers held, Esc, Tab,
/// Enter and Space alone included. The keys are taken out of the input,
/// so Esc does not also close the options.
fn capture(app: &mut App, ctx: &Context, presses: &[(Modifiers, Code)], events: &[Event]) {
    let on_page = matches!(app.dialog, crate::ui::dialogs::Dialog::Options)
        && app.options_page == crate::ui::options::OptionsPage::Commands;
    if !on_page {
        app.shortcut_editor.capture = None;
        return;
    }
    let plain = events.iter().find_map(|e| match e {
        Event::Key {
            key,
            pressed: true,
            repeat: false,
            modifiers,
            ..
        } if shortcuts::allowed(*modifiers, *key) => Some((*modifiers, Code::Key(*key))),
        _ => None,
    });
    let Some((m, code)) = presses.first().copied().or(plain) else {
        return;
    };
    let Some(target) = app.shortcut_editor.capture.take() else {
        return;
    };
    crate::ui::options::set_menu_shortcut(app, &target, &shortcuts::format_code(m, code));
    ctx.input_mut(|i| {
        i.events
            .retain(|e| !matches!(e, Event::Key { pressed: true, .. } | Event::Text(_)))
    });
}

/// Take every press of `key` (and the text it typed) out of this frame's
/// input.
fn take_key(ctx: &Context, key: Key) {
    ctx.input_mut(|i| {
        i.events.retain(|e| match e {
            Event::Key { key: k, .. } => *k != key,
            Event::Text(_) => false,
            _ => true,
        })
    });
}

/// The modifier taps among `events`: a modifier released with no other
/// key or button pressed since it went down. The modifiers held with it
/// (less its own) come along: holding Ctrl and tapping Shift is
/// Ctrl+LShift.
pub fn taps(events: &[Event]) -> Vec<(Modifiers, Tap)> {
    let mut out = Vec::new();
    for e in events {
        match e {
            Event::Key {
                key,
                pressed,
                repeat,
                modifiers,
                ..
            } => match (Tap::of(*key), *pressed) {
                (Some(t), true) if !repeat => PENDING_TAP.set(Some(t)),
                (Some(_), true) => {}
                (Some(t), false) => {
                    if PENDING_TAP.get() == Some(t) {
                        out.push((t.without_own(*modifiers), t));
                    }
                    PENDING_TAP.set(None);
                }
                (None, true) => PENDING_TAP.set(None),
                (None, false) => {}
            },
            Event::PointerButton { pressed: true, .. } | Event::MouseWheel { .. } => {
                PENDING_TAP.set(None)
            }
            _ => {}
        }
    }
    out
}

/// The extra keys that went down since the last look, while the window
/// has the focus. Empty outside Windows (there egui's keys and the taps
/// are what can be shortcuts).
fn poll(ctx: &Context) -> Vec<Extra> {
    let focused = ctx.input(|i| i.viewport().focused.unwrap_or(true));
    let mut out = Vec::new();
    HELD.with_borrow_mut(|held| {
        for (extra, vk) in Extra::ALL {
            let (down, since) = system::state(vk);
            let was = held[vk as usize];
            held[vk as usize] = down;
            // A tap between two looks only sets the "since" bit.
            if focused && !was && (down || since) {
                out.push(extra);
            }
        }
    });
    out
}

#[cfg(target_os = "windows")]
mod system {
    #[link(name = "user32")]
    extern "system" {
        fn GetAsyncKeyState(vkey: i32) -> i16;
    }

    /// Whether the key is down, and whether it was pressed since the
    /// last call.
    pub fn state(vk: u8) -> (bool, bool) {
        // SAFETY: GetAsyncKeyState takes any virtual-key code and only
        // reads the keyboard state.
        let s = unsafe { GetAsyncKeyState(vk as i32) } as u16;
        (s & 0x8000 != 0, s & 1 != 0)
    }
}

#[cfg(not(target_os = "windows"))]
mod system {
    pub fn state(_vk: u8) -> (bool, bool) {
        (false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(key: Key, pressed: bool, modifiers: Modifiers) -> Event {
        Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers,
        }
    }

    #[test]
    fn a_modifier_alone_is_a_tap_but_not_when_it_modifies_a_key() {
        PENDING_TAP.set(None);
        let t = taps(&[
            key(Key::ShiftLeft, true, Modifiers::SHIFT),
            key(Key::ShiftLeft, false, Modifiers::NONE),
        ]);
        assert_eq!(t, vec![(Modifiers::NONE, Tap::LShift)]);
        // Ctrl+E: Ctrl is not tapped.
        let t = taps(&[
            key(Key::ControlLeft, true, Modifiers::COMMAND),
            key(Key::E, true, Modifiers::COMMAND),
            key(Key::E, false, Modifiers::COMMAND),
            key(Key::ControlLeft, false, Modifiers::NONE),
        ]);
        assert!(t.is_empty());
        // Holding Ctrl and tapping the right Alt.
        let t = taps(&[
            key(Key::ControlLeft, true, Modifiers::COMMAND),
            key(Key::AltRight, true, Modifiers::COMMAND | Modifiers::ALT),
            key(Key::AltRight, false, Modifiers::COMMAND),
        ]);
        assert_eq!(t, vec![(Modifiers::COMMAND, Tap::RAlt)]);
        assert_eq!(
            shortcuts::format_code(t[0].0, Code::Tap(t[0].1)),
            "Ctrl+RAlt"
        );
        // A click between press and release: no tap.
        PENDING_TAP.set(None);
        let t = taps(&[
            key(Key::AltLeft, true, Modifiers::ALT),
            Event::PointerButton {
                pos: egui::Pos2::ZERO,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::ALT,
            },
            key(Key::AltLeft, false, Modifiers::NONE),
        ]);
        assert!(t.is_empty());
    }

    /// A shortcut of the user's on a tap runs its command; one on a tool
    /// picks the tool.
    #[test]
    fn taps_run_commands_and_pick_tools() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        app.show_welcome = false;
        app.settings
            .shortcuts
            .push(("menu.file.export".into(), "RCtrl".into()));
        PENDING_TAP.set(None);
        let input = egui::RawInput {
            events: vec![
                key(Key::ControlRight, true, Modifiers::COMMAND),
                key(Key::ControlRight, false, Modifiers::NONE),
            ],
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| frame(&mut app, ui.ctx()));
        out.textures_delta.clear();
        assert_eq!(app.pending_command.as_deref(), Some("menu.file.export"));
        app.pending_command = None;
        let tool = crate::tools::Tool::ALL[3];
        app.settings
            .shortcuts
            .push((tool.id().into(), "LAlt".into()));
        let input = egui::RawInput {
            events: vec![
                key(Key::AltLeft, true, Modifiers::ALT),
                key(Key::AltLeft, false, Modifiers::NONE),
            ],
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| frame(&mut app, ui.ctx()));
        out.textures_delta.clear();
        assert_eq!(app.tool, tool);
        assert!(app.pending_command.is_none());
    }
}

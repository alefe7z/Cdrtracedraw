//! The system's printers for the Print dialog's Printer list: the
//! printers CUPS knows (Linux and macOS), listed by `lpstat` and printed
//! to with `lp`, which takes the print PDF as it is. Where there is no
//! CUPS (Windows) the list is empty and the print goes
//! through the system PDF viewer, as before. See
//! docs/behavior/printing.md.

use std::path::Path;

/// A printer the system has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Printer {
    /// The name `lp -d` takes.
    pub name: String,
    /// The system's default destination.
    pub default: bool,
    /// What the system says of it ("idle", "Offline"); empty when unknown.
    pub status: String,
    /// Where it is, as the system was told.
    pub location: String,
    /// The system's description or comment.
    pub comment: String,
}

/// The printers `lpstat -l -p -d` lists ("printer NAME is idle..."
/// lines, their "Description:" and "Location:" lines and "system default
/// destination: NAME"), the default one first.
pub fn parse_lpstat(output: &str) -> Vec<Printer> {
    let mut out: Vec<Printer> = Vec::new();
    let mut default: Option<String> = None;
    for line in output.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("printer ") {
            let name = rest.split_whitespace().next().unwrap_or_default();
            if !name.is_empty() && !out.iter().any(|p| p.name == name) {
                let after = rest[name.len()..].trim_start();
                let status = after
                    .strip_prefix("is ")
                    .unwrap_or(after)
                    .split(['.', ' '])
                    .next()
                    .unwrap_or_default()
                    .to_string();
                out.push(Printer {
                    name: name.to_string(),
                    default: false,
                    status,
                    location: String::new(),
                    comment: String::new(),
                });
            }
        } else if let Some(rest) = line.strip_prefix("Description:") {
            if let Some(p) = out.last_mut() {
                p.comment = rest.trim().to_string();
            }
        } else if let Some(rest) = line.strip_prefix("Location:") {
            if let Some(p) = out.last_mut() {
                p.location = rest.trim().to_string();
            }
        } else if let Some(rest) = line.strip_prefix("system default destination:") {
            let name = rest.trim();
            if !name.is_empty() {
                default = Some(name.to_string());
            }
        }
    }
    if let Some(name) = default {
        if let Some(i) = out.iter().position(|p| p.name == name) {
            out[i].default = true;
            let p = out.remove(i);
            out.insert(0, p);
        }
    }
    out
}

/// The printers the system has, the default one first; empty where
/// none is set up or there is no CUPS.
#[cfg(not(target_os = "windows"))]
pub fn system_printers() -> Vec<Printer> {
    let out = std::process::Command::new("lpstat")
        .args(["-l", "-p", "-d"])
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .output();
    match out {
        Ok(o) => parse_lpstat(&String::from_utf8_lossy(&o.stdout)),
        Err(e) => {
            log::info!("no printer list: lpstat: {e}");
            Vec::new()
        }
    }
}

/// The printers Windows has (`Win32_Printer` through PowerShell), the
/// default one first.
#[cfg(target_os = "windows")]
pub fn system_printers() -> Vec<Printer> {
    let script = "Get-CimInstance Win32_Printer | Select-Object Name,Default,\
                  PrinterStatus,WorkOffline,Location,Comment | ConvertTo-Csv -NoTypeInformation";
    match hidden("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
    {
        Ok(o) => parse_windows_printers(&String::from_utf8_lossy(&o.stdout)),
        Err(e) => {
            log::info!("no printer list: powershell: {e}");
            Vec::new()
        }
    }
}

/// A command that opens no console window.
#[cfg(target_os = "windows")]
fn hidden(program: &str) -> std::process::Command {
    use std::os::windows::process::CommandExt;
    let mut c = std::process::Command::new(program);
    c.creation_flags(0x0800_0000);
    c
}

/// The CSV PowerShell writes for the printers: a header, then one quoted
/// row per printer (Name, Default, PrinterStatus, WorkOffline, Location,
/// Comment). The default one comes first.
#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
pub fn parse_windows_printers(csv: &str) -> Vec<Printer> {
    let mut lines = csv.lines().filter(|l| !l.trim().is_empty());
    let Some(head) = lines.next() else {
        return Vec::new();
    };
    let head = csv_fields(head);
    let col = |row: &[String], name: &str| -> String {
        head.iter()
            .position(|h| h.eq_ignore_ascii_case(name))
            .and_then(|i| row.get(i).cloned())
            .unwrap_or_default()
    };
    let mut out: Vec<Printer> = Vec::new();
    for line in lines {
        let row = csv_fields(line);
        let name = col(&row, "Name");
        if name.is_empty() {
            continue;
        }
        let offline = col(&row, "WorkOffline").eq_ignore_ascii_case("true");
        // Win32_Printer.PrinterStatus: 3 idle, 4 printing, 5 warming up,
        // 6 stopped, 7 offline.
        let status = if offline {
            "Offline".to_string()
        } else {
            match col(&row, "PrinterStatus").as_str() {
                "3" => "Ready",
                "4" => "Printing",
                "5" => "Warming up",
                "6" => "Stopped",
                "7" => "Offline",
                _ => "",
            }
            .to_string()
        };
        let p = Printer {
            name,
            default: col(&row, "Default").eq_ignore_ascii_case("true"),
            status,
            location: col(&row, "Location"),
            comment: col(&row, "Comment"),
        };
        if p.default {
            out.insert(0, p);
        } else {
            out.push(p);
        }
    }
    out
}

/// The fields of one CSV line, quotes removed (`""` is a quote).
#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
fn csv_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = line.trim_end_matches('\r').chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted && chars.peek() == Some(&'"') => {
                cur.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// The gear by the Printer list: the system's own settings of `printer`
/// (the driver's preferences on Windows).
pub fn open_preferences(printer: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let r = hidden("rundll32")
        .args(["printui.dll,PrintUIEntry", "/e", "/n", printer])
        .spawn();
    #[cfg(target_os = "macos")]
    let r = {
        let _ = printer;
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.Print-Scan-Settings.extension")
            .spawn()
    };
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let r = {
        let _ = printer;
        std::process::Command::new("system-config-printer").spawn()
    };
    r.map(|_| ()).map_err(|e| e.to_string())
}

/// The printer the dialog starts on: the one remembered when the system
/// still has it, else the system's default (the first listed), else none
/// (the print goes through the PDF viewer).
pub fn starting_printer(remembered: Option<&str>, printers: &[Printer]) -> Option<String> {
    if let Some(r) = remembered {
        if printers.iter().any(|p| p.name == r) {
            return Some(r.to_string());
        }
    }
    printers.first().map(|p| p.name.clone())
}

/// The `lp` command that prints `pdf` on `printer`: `copies` copies,
/// collated as whole sets or sheet by sheet, under `title` in the queue.
pub fn lp_args(printer: &str, pdf: &Path, copies: u32, collate: bool, title: &str) -> Vec<String> {
    let mut args = vec!["-d".to_string(), printer.to_string()];
    if copies > 1 {
        args.push("-n".into());
        args.push(copies.min(999).to_string());
        args.push("-o".into());
        args.push(format!(
            "collate={}",
            if collate { "true" } else { "false" }
        ));
    }
    let title = title.trim();
    if !title.is_empty() {
        args.push("-t".into());
        args.push(title.to_string());
    }
    args.push("--".into());
    args.push(pdf.display().to_string());
    args
}

/// Send `pdf` to `printer` through `lp`. Ok with what `lp` said (the
/// request id), Err with its complaint.
#[cfg(not(target_os = "windows"))]
pub fn print_pdf(
    printer: &str,
    pdf: &Path,
    copies: u32,
    collate: bool,
    title: &str,
) -> Result<String, String> {
    let out = std::process::Command::new("lp")
        .args(lp_args(printer, pdf, copies, collate, title))
        .env("LC_ALL", "C")
        .output()
        .map_err(|e| format!("lp: {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        Err(if err.is_empty() {
            format!("lp: {}", out.status)
        } else {
            err
        })
    }
}

/// Send `pdf` to `printer` through the PDF program's "Print to" verb;
/// without one that can, the PDF opens in the viewer to print from there.
#[cfg(target_os = "windows")]
pub fn print_pdf(
    printer: &str,
    pdf: &Path,
    _copies: u32,
    _collate: bool,
    _title: &str,
) -> Result<String, String> {
    let quote = |s: &str| s.replace('\'', "''");
    let script = format!(
        "Start-Process -FilePath '{}' -Verb PrintTo -ArgumentList '\"{}\"' -WindowStyle Hidden",
        quote(&pdf.display().to_string()),
        quote(printer)
    );
    let ok = hidden("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        Ok(String::new())
    } else {
        crate::export::open_with_system(pdf).map(|_| String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The Printer list comes from `lpstat`: every printer, the default
    /// first, whatever the rest of the line says (#300).
    #[test]
    fn the_printer_list_is_read_from_lpstat() {
        let out =
            "printer EPSON-L3250-Series is idle.  enabled since Tue 21 Oct 2026 10:00:00 AM\n\
                   printer Office_HP disabled since Mon 20 Oct 2026 09:00:00 AM -\n\
                   \tPaused\n\
                   printer PDF is idle.  enabled since Tue 21 Oct 2026 10:00:00 AM\n\
                   system default destination: Office_HP\n";
        let p = parse_lpstat(out);
        let names: Vec<&str> = p.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["Office_HP", "EPSON-L3250-Series", "PDF"]);
        assert!(p[0].default && !p[1].default && !p[2].default);
        // No printers, or no default: the list as given.
        assert!(parse_lpstat("lpstat: No destinations added.\n").is_empty());
        assert!(parse_lpstat("no system default destination\n").is_empty());
        let p =
            parse_lpstat("printer A is idle.\nprinter B is idle.\nno system default destination\n");
        assert_eq!(p.len(), 2);
        assert!(!p[0].default);
    }

    #[test]
    fn windows_printers_come_from_the_csv_with_status_and_location() {
        let csv = "\"Name\",\"Default\",\"PrinterStatus\",\"WorkOffline\",\"Location\",\"Comment\"\r\n\
                   \"Microsoft Print to PDF\",\"False\",\"3\",\"False\",\"\",\"\"\r\n\
                   \"EPSON L3250, Series\",\"True\",\"3\",\"True\",\"Sala 2\",\"Tinta \"\"nova\"\"\"\r\n";
        let p = parse_windows_printers(csv);
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].name, "EPSON L3250, Series");
        assert!(p[0].default);
        assert_eq!(p[0].status, "Offline");
        assert_eq!(p[0].location, "Sala 2");
        assert_eq!(p[0].comment, "Tinta \"nova\"");
        assert_eq!(p[1].status, "Ready");
        assert!(parse_windows_printers("").is_empty());
    }

    #[test]
    fn lpstat_gives_the_status_description_and_location() {
        let p = parse_lpstat(
            "printer Office is idle.  enabled since x\n\tDescription: HP de cor\n\tLocation: Sala 1\n",
        );
        assert_eq!(p[0].status, "idle");
        assert_eq!(p[0].comment, "HP de cor");
        assert_eq!(p[0].location, "Sala 1");
    }

    #[test]
    fn the_dialog_starts_on_the_remembered_printer_else_the_default() {
        let printers =
            parse_lpstat("printer A is idle.\nprinter B is idle.\nsystem default destination: B\n");
        assert_eq!(starting_printer(Some("A"), &printers).as_deref(), Some("A"));
        assert_eq!(
            starting_printer(Some("Gone"), &printers).as_deref(),
            Some("B")
        );
        assert_eq!(starting_printer(None, &printers).as_deref(), Some("B"));
        assert_eq!(starting_printer(Some("A"), &[]), None);
    }

    /// `lp` gets the printer, the copies with their collation, the title
    /// and the file; one copy needs no count.
    #[test]
    fn lp_is_told_the_printer_copies_and_title() {
        let pdf = Path::new("/tmp/tracedraw-print/Poster.pdf");
        assert_eq!(
            lp_args("EPSON-L3250-Series", pdf, 1, true, "Poster"),
            [
                "-d",
                "EPSON-L3250-Series",
                "-t",
                "Poster",
                "--",
                "/tmp/tracedraw-print/Poster.pdf"
            ]
        );
        assert_eq!(
            lp_args("A", pdf, 3, false, ""),
            [
                "-d",
                "A",
                "-n",
                "3",
                "-o",
                "collate=false",
                "--",
                "/tmp/tracedraw-print/Poster.pdf"
            ]
        );
        assert_eq!(
            lp_args("A", pdf, 2, true, " ")[4..6],
            ["-o", "collate=true"]
        );
    }
}

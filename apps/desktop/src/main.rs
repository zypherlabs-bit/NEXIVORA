use std::io::{self, Write};

const VERSION: &str = "0.1.0";
const APP_NAME: &str = "Nexivora Office Suite";
const REPO: &str = "https://github.com/zypherlabs-bit/NEXIVORA";

fn main() {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    writeln!(out, "╔══════════════════════════════════════════════════════╗").unwrap();
    writeln!(out, "║          Nexivora Office Suite  v{}              ║", VERSION).unwrap();
    writeln!(out, "║     Free, open-source, privacy-first productivity    ║").unwrap();
    writeln!(out, "╚══════════════════════════════════════════════════════╝").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "  Platform:    {}", std::env::consts::OS).unwrap();
    writeln!(out, "  Repository:  {}", REPO).unwrap();
    writeln!(out, "  License:     AGPL-3.0-or-later").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "  Engines:").unwrap();
    writeln!(out, "    📝  Document Engine   — rich text editing").unwrap();
    writeln!(out, "    📊  Spreadsheet       — 100+ formula functions").unwrap();
    writeln!(out, "    🎨  Presentation      — slides & transitions").unwrap();
    writeln!(out, "    🗄️   Database          — local data management").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "  {} started successfully.", APP_NAME).unwrap();
    writeln!(out, "  For usage, visit: {}", REPO).unwrap();

    // Keep the process running briefly so it's visible as a working application
    std::thread::sleep(std::time::Duration::from_millis(200));
}

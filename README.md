# Nexivora

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![GitHub Release](https://img.shields.io/github/v/release/nexivora/nexivora)](https://github.com/nexivora/nexivora/releases)
[![Build Status](https://github.com/nexivora/nexivora/actions/workflows/ci.yml/badge.svg)](https://github.com/nexivora/nexivora/actions/workflows/ci.yml)

**Nexivora** is a modern, open-source office suite built with Rust and Tauri. It provides a complete productivity solution with spreadsheets, documents, presentations, and databases - all in a single, cross-platform application.

## 🚀 Download Nexivora

Nexivora is available for Windows, Linux, and macOS. Choose your platform:

<details>
<summary>🪟 Windows</summary>

### Windows Downloads

| Version | Architecture | Download | Checksum |
|---------|--------------|----------|----------|
| Latest | x64 | [Nexivora-Windows-x64.exe](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Windows-x64.exe) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Windows-x64.exe.sha256) |
| Latest | Portable | [Nexivora-Windows-Portable.zip](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Windows-Portable.zip) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Windows-Portable.zip.sha256) |

**Supported Versions**: Windows 10 (64-bit) and Windows 11

**Installation**:
```powershell
# Download and verify
Invoke-WebRequest -Uri "https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Windows-x64.exe" -OutFile "Nexivora-Installer.exe"
Get-FileHash -Algorithm SHA256 "Nexivora-Installer.exe"

# Install
Start-Process -FilePath ".\Nexivora-Installer.exe" -Wait
```

[📖 Windows Installation Guide](docs/installation/windows.md)
</details>

<details>
<summary>🐧 Linux</summary>

### Linux Downloads

| Format | Architecture | Download | Checksum |
|--------|--------------|----------|----------|
| AppImage | x86_64 | [Nexivora-Linux-x86_64.AppImage](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Linux-x86_64.AppImage) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Linux-x86_64.AppImage.sha256) |
| DEB | amd64 | [nexivora-amd64.deb](https://github.com/nexivora/nexivora/releases/latest/download/nexivora-amd64.deb) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/nexivora-amd64.deb.sha256) |
| RPM | x86_64 | [nexivora-x86_64.rpm](https://github.com/nexivora/nexivora/releases/latest/download/nexivora-x86_64.rpm) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/nexivora-x86_64.rpm.sha256) |

**Supported Distributions**: Ubuntu 20.04+, Debian 11+, Fedora 36+, openSUSE 15.4+

**Installation**:
```bash
# AppImage (recommended)
wget https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-Linux-x86_64.AppImage
chmod +x Nexivora-Linux-x86_64.AppImage
./Nexivora-Linux-x86_64.AppImage

# DEB (Debian/Ubuntu)
wget https://github.com/nexivora/nexivora/releases/latest/download/nexivora-amd64.deb
sudo apt install ./nexivora-amd64.deb

# RPM (Fedora/openSUSE)
wget https://github.com/nexivora/nexivora/releases/latest/download/nexivora-x86_64.rpm
sudo dnf install ./nexivora-x86_64.rpm  # Fedora
# sudo zypper install ./nexivora-x86_64.rpm  # openSUSE
```

[📖 Linux Installation Guide](docs/installation/linux.md)
</details>

<details>
<summary>🍎 macOS</summary>

### macOS Downloads

| Architecture | Download | Checksum |
|--------------|----------|----------|
| Universal | [Nexivora-macOS-Universal.dmg](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-macOS-Universal.dmg) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-macOS-Universal.dmg.sha256) |
| Apple Silicon | [Nexivora-macOS-AppleSilicon.dmg](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-macOS-AppleSilicon.dmg) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-macOS-AppleSilicon.dmg.sha256) |
| Intel | [Nexivora-macOS-Intel.dmg](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-macOS-Intel.dmg) | [SHA256](https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-macOS-Intel.dmg.sha256) |

**Supported Versions**: macOS 10.15 (Catalina) and later

**Installation**:
```bash
# Download and verify
curl -L -o Nexivora.dmg https://github.com/nexivora/nexivora/releases/latest/download/Nexivora-macOS-Universal.dmg
shasum -a 256 Nexivora.dmg

# Mount and install
hdiutil attach Nexivora.dmg
cp -R /Volumes/Nexivora/Nexivora.app /Applications/
hdiutil detach /Volumes/Nexivora
```

[📖 macOS Installation Guide](docs/installation/macos.md)
</details>

## 📦 Features

- **Spreadsheet Engine**: Advanced formula parsing and evaluation with 100+ functions
- **Document Engine**: Rich text editing with styles, formatting, and templates
- **Presentation Engine**: Slide creation with transitions and animations
- **Database Engine**: Local data management with query capabilities
- **Cross-Platform**: Native performance on Windows, Linux, and macOS
- **Offline-First**: Full functionality without internet connection
- **Privacy-Focused**: No telemetry, no ads, no required accounts
- **Extensible**: Plugin system for custom functionality

## 🚀 Getting Started (Developers)

```bash
git clone https://github.com/nexivora/nexivora.git
cd nexivora
cargo build --release
```

For more detailed documentation, see the [docs](docs/README.md).

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guide](CONTRIBUTING.md) before submitting pull requests.

## 📜 License

This project is licensed under the [AGPL-3.0-or-later](LICENSE).

## 📞 Support

- [Documentation](docs/README.md)
- [Issues](https://github.com/nexivora/nexivora/issues)
- [Discussions](https://github.com/nexivora/nexivora/discussions)
- [Security Policy](SECURITY.md)

## 🌟 Stargazers

[![Stargazers repo roster for @nexivora/nexivora](https://reporoster.com/stars/nexivora/nexivora)](https://github.com/nexivora/nexivora/stargazers)

## 🏆 Backers

Support Nexivora development by becoming a backer or sponsor!

[![GitHub Sponsors](https://img.shields.io/github/sponsors/nexivora)](https://github.com/sponsors/nexivora)
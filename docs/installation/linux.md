# Linux Installation

## System Requirements

### Minimum Requirements
- **Operating System**: Any modern Linux distribution (64-bit)
- **Processor**: 1 GHz or faster with 2 or more cores
- **RAM**: 2 GB (4 GB recommended)
- **Storage**: 500 MB available space
- **Dependencies**: GTK 3.0+, libwebkit2gtk-4.0, and standard system libraries

### Recommended Requirements
- **Operating System**: Ubuntu 22.04 LTS, Fedora 38+, or equivalent
- **Processor**: 2 GHz or faster with 4 or more cores
- **RAM**: 8 GB or more
- **Storage**: 1 GB available space (SSD recommended)

### Supported Distributions

**Officially Supported**:
- Ubuntu 20.04 LTS and later
- Debian 11 and later
- Fedora 36 and later
- openSUSE Leap 15.4 and later
- Arch Linux (and derivatives like Manjaro)

**Community Supported**:
- Other distributions may work but are not officially tested

## Installation Methods

### AppImage (Recommended)

1. **Download** the AppImage from [GitHub Releases](https://github.com/zypherlabs-bit/NEXIVORA/releases)
2. **Make executable**: `chmod +x Nexivora-*.AppImage`
3. **Run**: `./Nexivora-*.AppImage`

**Optional**: Integrate with your system:
```bash
# Create a desktop entry
sudo cp Nexivora-*.AppImage /opt/nexivora/
sudo ln -s /opt/nexivora/Nexivora-*.AppImage /usr/local/bin/nexivora

# Create desktop shortcut
cat > ~/.local/share/applications/nexivora.desktop << EOF
[Desktop Entry]
Name=Nexivora
Exec=/opt/nexivora/Nexivora-*.AppImage
Icon=/opt/nexivora/nexivora.png
Type=Application
Categories=Office;
Terminal=false
EOF
```

### DEB Package (Debian/Ubuntu)

1. **Download** the DEB package from [GitHub Releases](https://github.com/zypherlabs-bit/NEXIVORA/releases)
2. **Install dependencies**: `sudo apt update && sudo apt install -y libgtk-3-0 libwebkit2gtk-4.0-37`
3. **Install**: `sudo dpkg -i nexivora-*.deb`
4. **Fix dependencies**: `sudo apt --fix-broken install`

### RPM Package (Fedora/openSUSE)

1. **Download** the RPM package from [GitHub Releases](https://github.com/zypherlabs-bit/NEXIVORA/releases)
2. **Install**: `sudo dnf install ./nexivora-*.rpm` (Fedora) or `sudo zypper install ./nexivora-*.rpm` (openSUSE)

### Flatpak (When Available)

```bash
flatpak install flathub org.nexivora.Nexivora
flatpak run org.nexivora.Nexivora
```

## Verification

### Checksum Verification

After downloading, verify the integrity:

```bash
# For AppImage
sha256sum Nexivora-*.AppImage

# For DEB
sha256sum nexivora-*.deb

# For RPM
sha256sum nexivora-*.rpm
```

Compare the output with the SHA-256 checksum listed in the release notes.

## Troubleshooting

### AppImage Issues

**Problem**: AppImage doesn't run
**Solution**:
- Ensure it's executable: `chmod +x Nexivora-*.AppImage`
- Install FUSE: `sudo apt install libfuse2` (Ubuntu/Debian)
- Try: `./Nexivora-*.AppImage --no-sandbox`

**Problem**: "No such file or directory" error
**Solution**: Your system might be missing 32-bit libraries:
```bash
sudo dpkg --add-architecture i386
sudo apt update
sudo apt install libgtk-3-0:i386 libwebkit2gtk-4.0-37:i386
```

### DEB/RPM Issues

**Problem**: Dependency errors
**Solution**: Install missing dependencies manually or use:
```bash
# For DEB
sudo apt --fix-broken install

# For RPM
sudo dnf install -y $(rpm -qpR nexivora-*.rpm | grep -v "rpmlib(")
```

### Library Issues

**Problem**: Missing GTK or WebKitGTK
**Solution**:
```bash
# Ubuntu/Debian
sudo apt install libgtk-3-0 libwebkit2gtk-4.0-37

# Fedora
sudo dnf install gtk3 webkit2gtk3

# openSUSE
sudo zypper install gtk3 libwebkit2gtk-4_0-37

# Arch Linux
sudo pacman -S gtk3 webkit2gtk
```

## Upgrading

### From Previous Versions

1. **Backup** your documents (see below)
2. **Download** the latest version
3. **Install** using your preferred method
4. **Verify** your documents open correctly

### Backup Important Data

Before upgrading, we recommend backing up:
- `~/.config/nexivora`
- `~/.local/share/nexivora`
- Any custom templates or configurations

## Uninstallation

### AppImage
Simply delete the AppImage file and any desktop entries you created.

### DEB Package
```bash
sudo apt remove nexivora
```

### RPM Package
```bash
sudo dnf remove nexivora  # Fedora
sudo zypper remove nexivora  # openSUSE
```

### Manual Cleanup
After uninstalling, you may want to remove leftover files:
- `~/.config/nexivora`
- `~/.local/share/nexivora`
- `~/.cache/nexivora`

## Common Errors

### "Failed to load module: canberra-gtk-module"
Install the missing module:
```bash
sudo apt install libcanberra-gtk-module  # Ubuntu/Debian
sudo dnf install libcanberra-gtk3  # Fedora
```

### "Symbol lookup error"
Your system might have incompatible library versions. Try:
```bash
ldd ./Nexivora-*.AppImage
```
To see which dependencies might be missing or incompatible.

### "Gtk-WARNING: cannot open display"
Ensure you have X11 or Wayland running and proper display permissions.

## Support

For additional help:
- Check our [FAQ](https://github.com/zypherlabs-bit/NEXIVORA/wiki/FAQ)
- Open an [Issue](https://github.com/zypherlabs-bit/NEXIVORA/issues)
- Join our [Community Discussions](https://github.com/zypherlabs-bit/NEXIVORA/discussions)
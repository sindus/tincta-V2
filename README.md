# SimpleEdit

[![CI](https://github.com/simpleeditdev/simpleedit/actions/workflows/ci.yml/badge.svg)](https://github.com/simpleeditdev/simpleedit/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A fast, stable, cross-platform text editor for **macOS** and **Linux**. Built in Rust with [iced](https://github.com/iced-rs/iced).

**[Website & Downloads](https://simpleeditdev.github.io/simpleedit)**

---

## Features

- **Syntax highlighting** for 60+ languages (TextMate grammars)
- **File sidebar** — open and switch between multiple files
- **Search & Replace** with regex support
- **Dark / Light theme**
- **Internationalisation** — English and French
- Configurable: font size, tab width, word wrap, auto-indent, bracket/quote completion
- Line editing: duplicate, move, comment/uncomment, indent/dedent
- Code formatting via external tools (prettier, rustfmt, …)
- Native binaries — macOS (Apple Silicon) and Linux (x86_64)

---

## Installation

### One-liner (Linux & macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/simpleeditdev/simpleedit/main/install.sh | bash
```

Detects your OS automatically, downloads the latest release, and installs it.

---

### macOS — Homebrew

```bash
brew tap simpleeditdev/simpleedit
brew install simpleedit
```

**Uninstall:**
```bash
brew uninstall simpleedit
```

---

### Ubuntu / Debian — .deb

Download and install the latest `.deb` package:

```bash
curl -fsSL https://api.github.com/repos/simpleeditdev/simpleedit/releases/latest \
  | grep '"browser_download_url"' | grep '\.deb' \
  | cut -d '"' -f 4 | xargs wget -q -O simpleedit.deb
sudo dpkg -i simpleedit.deb && rm simpleedit.deb
```

Or grab the file directly from the [Releases page](https://github.com/simpleeditdev/simpleedit/releases/latest).

**Uninstall:**
```bash
sudo apt remove simpleedit
# or
sudo dpkg -r simpleedit
```

---

### Linux — tar.gz (any distro)

```bash
VERSION=$(curl -fsSL https://api.github.com/repos/simpleeditdev/simpleedit/releases/latest | grep '"tag_name"' | head -1 | sed 's/.*"\(.*\)".*/\1/')
curl -fsSL "https://github.com/simpleeditdev/simpleedit/releases/download/${VERSION}/simpleedit-${VERSION}-x86_64-linux.tar.gz" | tar xz
sudo mv simpleedit /usr/local/bin/
```

**Uninstall:**
```bash
sudo rm /usr/local/bin/simpleedit
```

---

### Build from source

```bash
# Prerequisites (Ubuntu/Debian)
sudo apt-get install libgtk-3-dev libxkbcommon-dev

# Build & run
cargo build --release
./target/release/simpleedit
```

---

## Usage

```bash
simpleedit                  # open with last session
simpleedit path/to/file     # open a specific file
```

---

## Development

```bash
cargo run           # run in dev mode
cargo test          # run tests
cargo clippy        # lint
cargo fmt           # format source
```

---

## License

MIT — see [LICENSE](LICENSE).

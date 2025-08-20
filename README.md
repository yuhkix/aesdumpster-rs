<div align="center">

# 🦀 AESDumpster-rs

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Windows](https://img.shields.io/badge/platform-windows-lightgrey.svg)](https://www.microsoft.com/windows)

A high-performance Rust utility for scanning Windows executables to locate AES keys through pattern matching and entropy analysis.

[Features](#-key-features) •
[Quick Start](#-quickstart) •
[Usage](#-usage-and-output) •
[How It Works](#-how-it-works) •
[Building](#-building-details)

![Ferris](https://i.ibb.co/QVThVkd/Ferris.png)

</div>

## 🚀 Key Features

- ✨ Advanced signature scanning with wildcard support (`C7/?? byte patterns`)
- 🔑 Intelligent extraction of 32-byte hex keys from matched patterns
- 📊 Shannon entropy analysis for candidate ranking
- 🎨 Rich console output with color-coded results
- 🛡️ Built-in false positive filtering

## 🔧 Quick Start

### Prerequisites

- Rust (stable channel)
- MSVC toolchain (via Visual Studio Build Tools or Visual Studio)

### Installation

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone https://github.com/yuhkix/aesdumpster-rs
   cd aesdumpster-rs
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

### Running the Tool

```powershell
# Single file analysis
.\target\release\aesdumpster.exe C:\path\to\Game-Win64-Shipping.exe

# Multiple file analysis
.\target\release\aesdumpster.exe C:\path\to\First.exe C:\path\to\Second.exe
```

## 📘 Usage and Output

### Output Format

Each analyzed file produces color-coded results based on entropy scoring:

| Entropy Range | Color        | Likelihood |
|--------------|--------------|------------|
| ≥3.7         | Bright Green | Highest    |
| 3.5-3.7      | Bright Yellow| High       |
| 3.4-3.5      | Dark Yellow  | Medium     |
| 3.3-3.4      | Bright Red   | Low        |

Example output:
```
Key: 0x0123ABCD... | Key Entropy: 3.72
```

## 🔍 How It Works

### Core Components

1. **Signature Scanner**
   - Pattern matching engine with wildcard support
   - Optimized for Unreal Engine code patterns

2. **Key Assembly**
   - Concatenates 8 DWORDs into 32-byte keys
   - Intelligent offset handling

3. **Entropy Analysis**
   - Shannon entropy calculation
   - Adaptive threshold system

## 📁 Project Structure

```
aesdumpster-rs/
├── src/
│   ├── main.rs           # Core execution logic
│   ├── other_tools.rs    # File & console utilities
│   └── key_dumpster.rs   # Scanner & analysis engine
```

## 🛠️ Building Details

### Dependencies

- `windows` - Windows API bindings for console manipulation

### Build Commands

```powershell
# Debug build
cargo build

# Release build
cargo build --release
```

## 🙏 Credits

- Original [AESDumpster](https://github.com/GHFear/AESDumpster) by GHFear @ IllusorySoftware
- Rust implementation focusing on memory safety and performance

## 📜 Disclaimer

This tool is intended for legitimate research, debugging, and forensics purposes. Users must ensure they have appropriate rights to analyze target binaries. The authors and contributors accept no responsibility for misuse.

---
<div align="center">
Made with ❤️ using Rust
</div>
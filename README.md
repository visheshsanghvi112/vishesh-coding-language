<p align="center">
    <img alt="Vish" src="https://raw.githubusercontent.com/vedic-lang/vedic/main/assets/vedic-lang.png" width="300vw"/>
    <br><br>
    <b>🕉️ VISH LANGUAGE - Vedic on Steroids 🕉️</b>
    <br>
    <i>A Sanskrit Programming Language by Vishesh Sanghvi</i>
</p>

<p align="center">
    <a href="#installation"><img src="https://img.shields.io/badge/install-one--liner-green" alt="Install"></a>
    <a href="#features"><img src="https://img.shields.io/badge/functions-28+-blue" alt="Functions"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-orange" alt="License"></a>
    <a href="#"><img src="https://img.shields.io/badge/extension-.vish-purple" alt="Extension"></a>
</p>

---

## ⚡ Installation

### Linux/macOS (One-liner)
```bash
curl -fsSL https://raw.githubusercontent.com/visheshsanghvi112/vishesh-coding-language/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/visheshsanghvi112/vishesh-coding-language/main/install.ps1 | iex
```

### Manual Download
Download from [Releases](https://github.com/visheshsanghvi112/vishesh-coding-language/releases)

---

## 🚀 Quick Start

```bash
# Create a Vish file
echo 'वद("नमस्ते विश्व!");' > hello.vish

# Run it
vish hello.vish
```

---

## ✨ Features

### 📐 Mathematics (11 Functions)
| Sanskrit | Function | Example |
|----------|----------|---------|
| `मूल()` | sqrt | `मूल(16)` → 4 |
| `घात()` | pow | `घात(2, 8)` → 256 |
| `ज्या()` | sin | `ज्या(पै()/2)` → 1 |
| `कोज्या()` | cos | `कोज्या(0)` → 1 |
| `स्पर्शज्या()` | tan | Tangent |
| `पै()` | π | 3.14159... |
| `यादृच्छिक()` | random | 0.0-1.0 |
| `पूर्णाङ्क()` | floor | `पूर्णाङ्क(3.7)` → 3 |
| `उन्नयन()` | ceil | `उन्नयन(3.2)` → 4 |
| `निरपेक्ष()` | abs | `निरपेक्ष(-5)` → 5 |
| `लघुगणक()` | ln | Natural log |

### 📝 String Manipulation (10 Functions)
`कर्तन` (trim) · `दीर्घाक्षर` (uppercase) · `ह्रस्वाक्षर` (lowercase) · `समाविष्ट` (contains) · `प्रतिस्थापन` (replace) · `अक्षरम्` (charAt) · `विभाजन` (split) · `उल्टा` (reverse) · `आरम्भ` (startsWith) · `अन्त` (endsWith)

### 📁 File I/O
```vish
लेखन("output.txt", "नमस्ते!");  # Write
मान data = वाचन("input.txt");   # Read
```

### 🔐 Cryptography
```vish
मान hash = गूढलिपि("secret");
```

### 🎨 SVG Graphics
```vish
मान art = Canvas();
art.Srijan();
art.Chakra(250, 250, 100, "gold");
art.Samapan();  # → mandala.svg
```

---

## 📖 Language Reference

```vish
# Variables
मान x = 42;
मान नाम = "विशेष";

# Output
वद("Hello, World!");

# Conditionals
यदि (x > 10) {
    वद("बड़ा");
} अथ {
    वद("छोटा");
}

# Loops
पर्यन्त (i < 10) {
    वद(i);
    i = i + 1;
}

# Functions
सूत्र योग(a, b) {
    फल a + b;
}
```

---

## 📂 Project Structure

```
vishesh-coding-language/
├── cli/               # Command-line interface
├── core/              # Core interpreter
│   └── moolsutra/     # 28 native functions
├── examples/          # Demo projects
│   ├── demos/         # Feature showcases
│   ├── graphics/      # SVG generation
│   └── games/         # Game simulations
├── polyfill/          # Python interpreter
├── install.sh         # Linux/macOS installer
└── install.ps1        # Windows installer
```

---

## 👤 Author

**Vishesh Sanghvi**
- GitHub: [@visheshsanghvi112](https://github.com/visheshsanghvi112)

### Credits
Original Vedic Language by [Pt. Prashant Tripathi](https://github.com/ptprashanttripathi)

---

## 📝 License

MIT License

---

<p align="center">
    <b>🕉️ असतो मा सद्गमय। तमसो मा ज्योतिर्गमय॥ 🕉️</b><br>
    <i>"Lead me from unreal to real, from darkness to light."</i>
</p>

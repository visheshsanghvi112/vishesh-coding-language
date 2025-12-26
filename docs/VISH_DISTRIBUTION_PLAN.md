# 🚀 VISH LANGUAGE - Distribution & Launch Plan
## "Vedic on Steroids" - Created by Vishesh

---

## 🎯 THE VISION

**Goal:** Make Vish Language downloadable worldwide, so anyone can:
1. Install `vish` with a single command
2. Run `.vish` files seamlessly
3. Access 28+ Sanskrit-named functions
4. Experience "Vedic on Steroids"

---

## 📋 DISTRIBUTION STRATEGY

### Phase 1: GitHub Release (Week 1) ✅ READY
**Objective:** Make it downloadable from GitHub

#### Steps:
1. **Fork the Vedic repo** to your own GitHub account
2. **Rename the fork** to `vish-lang` or `vishesh-lang`
3. **Push all changes** we made (new modules, examples, README)
4. **Create a GitHub Release** with pre-built binaries

#### Pre-built Binaries Needed:
| Platform | File | Architecture |
|----------|------|--------------|
| Windows | `vish-windows-x64.exe` | 64-bit |
| macOS | `vish-macos-x64` | Intel |
| macOS | `vish-macos-arm64` | Apple Silicon |
| Linux | `vish-linux-x64` | 64-bit |

#### Build Commands (on each platform):
```bash
cargo build --release
# Binary at: target/release/vedic (rename to vish)
```

---

### Phase 2: One-Line Install Script (Week 2)

#### For Linux/macOS:
Create `install.sh`:
```bash
#!/bin/bash
curl -fsSL https://github.com/YOUR_USERNAME/vish-lang/releases/latest/download/vish-$(uname -s)-$(uname -m) -o /usr/local/bin/vish
chmod +x /usr/local/bin/vish
echo "✅ Vish Language installed! Run: vish --help"
```

Users install with:
```bash
curl -fsSL https://vishesh.dev/install.sh | bash
```

#### For Windows:
Create `install.ps1`:
```powershell
Invoke-WebRequest -Uri "https://github.com/YOUR_USERNAME/vish-lang/releases/latest/download/vish-windows-x64.exe" -OutFile "$env:USERPROFILE\vish.exe"
$env:PATH += ";$env:USERPROFILE"
Write-Host "✅ Vish Language installed! Run: vish --help"
```

Users install with:
```powershell
irm https://vishesh.dev/install.ps1 | iex
```

---

### Phase 3: Package Managers (Week 3-4)

#### npm (Cross-platform via Node.js wrapper)
```bash
npm install -g vish-lang
```

Create a Node.js wrapper that downloads the binary for the current platform.

#### Homebrew (macOS/Linux)
```bash
brew tap vishesh/vish
brew install vish
```

Create a Homebrew formula that builds from source.

#### Chocolatey (Windows)
```bash
choco install vish
```

#### Scoop (Windows)
```bash
scoop bucket add vish https://github.com/vishesh/scoop-vish
scoop install vish
```

---

### Phase 4: Website & Documentation (Week 4-5)

#### Create `vishesh.dev` or `vish-lang.github.io`

**Homepage:**
- Hero section with live code editor
- One-click download buttons
- Feature showcase

**Pages:**
1. `/download` - Platform-specific installers
2. `/docs` - Full language documentation
3. `/playground` - Online WASM-based IDE
4. `/examples` - Gallery of demo projects

---

### Phase 5: Online Playground (Week 5-6)

**Use the existing WASM module!**

The repo already has `vedic/wasm/` which compiles to WebAssembly.

Steps:
1. Build WASM: `wasm-pack build --target web`
2. Create a web page with:
   - Monaco Editor (code input)
   - Run button
   - Output console
3. Host on GitHub Pages

Users can try Vish without installing anything!

---

## 🔧 TECHNICAL REQUIREMENTS

### 1. Rebrand the Binary
In `cli/Cargo.toml`, change:
```toml
[package]
name = "vish"
version = "2.0.0"
description = "Vish Language - Vedic on Steroids"
authors = ["Vishesh"]
```

### 2. Update CLI Name
In `cli/src/main.rs`, the binary is already called based on Cargo package name.

### 3. File Extension Registration
For full OS integration, create:
- **Windows:** Registry entries for `.vish` association
- **macOS:** `Info.plist` with UTI declarations
- **Linux:** `.desktop` file and MIME type

### 4. VS Code Extension
Create a syntax highlighter:
```json
{
  "name": "vish-lang",
  "fileTypes": [".vish", ".ved"],
  "scopeName": "source.vish"
}
```

---

## 📊 DISTRIBUTION CHANNELS SUMMARY

| Channel | Effort | Reach | Priority |
|---------|--------|-------|----------|
| GitHub Releases | Low | Developers | 🔴 HIGH |
| Install Script | Medium | Power Users | 🔴 HIGH |
| npm | Medium | JS Developers | 🟡 MEDIUM |
| Homebrew | Medium | macOS/Linux | 🟡 MEDIUM |
| Chocolatey | Medium | Windows | 🟡 MEDIUM |
| Website | High | Everyone | 🟡 MEDIUM |
| WASM Playground | High | Curious Users | 🟢 LOW |
| VS Code Extension | Low | Developers | 🟢 LOW |

---

## 📅 LAUNCH TIMELINE

```
Week 1: GitHub Release
        ├── Fork repo
        ├── Push changes
        └── Create first release with binaries

Week 2: Install Scripts
        ├── Create install.sh and install.ps1
        └── Test on all platforms

Week 3: Package Managers
        ├── Submit to npm
        └── Create Homebrew formula

Week 4: Documentation
        ├── Create vish-lang.github.io
        └── Write comprehensive docs

Week 5: Community
        ├── Launch on Reddit (r/programming, r/rust)
        ├── Post on Hacker News
        └── Tweet announcement

Week 6: Polish
        ├── Create VS Code extension
        └── Launch WASM playground
```

---

## 🎯 IMMEDIATE NEXT STEPS

### Step 1: Create GitHub Account/Repo
```bash
# On GitHub:
# 1. Create new repo: vishesh/vish-lang
# 2. OR fork vedic-lang/vedic to vishesh/vish-lang

git remote add origin https://github.com/vishesh/vish-lang.git
git push -u origin main
```

### Step 2: Build Binaries
```bash
# On Windows
cargo build --release
copy target\release\vedic.exe vish-windows-x64.exe

# On macOS
cargo build --release
cp target/release/vedic vish-macos-x64

# On Linux
cargo build --release
cp target/release/vedic vish-linux-x64
```

### Step 3: Create GitHub Release
1. Go to repo → Releases → "Draft new release"
2. Tag: `v2.0.0`
3. Title: "Vish Language v2.0 - Vedic on Steroids"
4. Upload all binaries
5. Publish!

---

## 💡 NAMING OPTIONS

| Option | Pros | Cons |
|--------|------|------|
| `vish` | Short, your name, memorable | Might conflict with "vish" (poison in Hindi) |
| `vishesh` | Full name, unique | Longer to type |
| `vish-lang` | Clear it's a language | Two words |
| `vlang` | Very short | Conflicts with V language |

**Recommendation:** Use `vish` as the binary name, `Vish Language` as the full name.

---

## 🏆 SUCCESS METRICS

| Metric | Week 1 | Month 1 | Month 6 |
|--------|--------|---------|---------|
| GitHub Stars | 50 | 200 | 1000 |
| Downloads | 100 | 500 | 5000 |
| Contributors | 1 | 5 | 20 |
| npm Weekly Downloads | 0 | 50 | 500 |

---

## ❓ QUESTIONS FOR YOU

1. **GitHub username?** (I'll use it to update docs)
2. **Custom domain?** (e.g., vishesh.dev, vish-lang.com)
3. **Do you have access to a Mac/Linux for builds?** (Or use GitHub Actions)
4. **Budget for hosting?** (GitHub Pages is free, custom domain ~$12/year)

---

**Once you answer these, I can start implementing the distribution pipeline!**

---

*This plan transforms Vish from a local project to a globally downloadable programming language.*

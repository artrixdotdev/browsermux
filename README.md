# 🌐 Browsermux
**Open multiple browsers at once — with a single click.**

<!-- [![Crates.io](https://img.shields.io/crates/v/browsermux)](https://crates.io/crates/browsermux) -->
<!-- [![AUR](https://img.shields.io/aur/version/browsermux)](https://aur.archlinux.org/packages/browsermux) -->

---

## ✨ Features
- 🔍 **Regex powered rules** — match any URL and decide where it should open.
- 🌍 **Browser agnostic** — works with any browser (or application).
- ⚡ **One‑click launch** — spin up multiple browsers instantly, no extra steps.

---

## 🛠️ Roadmap
- 🚀 Set **Browsermux as the default system browser**, so your rules apply everywhere.
- 📝 Install shell completions by default.
- 💻 Run browsers through `bash` or the default shell.


---

## 📦 Installation

### 🐧 Unix-based (macOS, Linux)
Arch Linux users get first-class support:
```bash
yay -S browsermux
```

For all other Unix-based systems, simply run the installer from the [latest release](https://github.com/artrixdotdev/browsermux/releases/latest):

```bash
curl -s https://github.com/artrixdotdev/browsermux/releases/latest/download/browsermux-installer.sh | bash
```


### 🪟 Windows
Head over to the [latest release](https://github.com/artrixdotdev/browsermux/releases/latest) and download the Windows installer (`.msi`).

Or, if you can install it via the powershell script:

```powershell
iwr -useb https://github.com/artrixdotdev/browsermux/releases/latest/download/browsermux-installer.ps1 | iex
```



---

### 🐚 BSD & Others
Available on [crates.io](https://crates.io/crates/browsermux):
```bash
cargo install browsermux
```

---

## ⚙️ Configuration
Browsermux is configured via `~/.config/browsermux/config.toml`.

Here's an example configuration:
```toml
"$schema" = "https://github.com/artrixdotdev/browsermux/releases/latest/download/schema.json"

# The default browser to use when no rules match
default = "zen" # Always a key in the [browsers] table

# The executables of the browsers you want to use
[browsers]
zen = "/usr/bin/zen-browser"
chrome = "/usr/bin/google-chrome"

[[rules]]
regex = 'https:\/\/youtube\.com[^\s]*'
browser = "chrome" # Always a key in the [browsers] table
```

#

## 🚀 Why Browsermux?
Whether you're a developer juggling Chrome, Firefox, and a privacy‑hardened browser,
or someone who just likes keeping work and personal browsing **totally separate** —
**Browsermux makes managing multiple browsers effortless.**


Would you like me to also add example **configuration snippets** (like `.browsermuxrc` with regex rules) so people immediately see how to use it after installing? That could make the README feel even more welcoming.

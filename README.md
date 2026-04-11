# 🌐 Anveshan Browser

> *"Most browsers hide the web. Anveshan reveals it."*
> — Arjun Puri

**Anveshan** (Sanskrit: अन्वेषण) — *to search, to explore, to surf.*

An open-source browser built from scratch in Rust. No telemetry. No bloat. Just the web.

---

## 🧠 The Vision

Hi, I'm Arjun. I built Anveshan out of frustration.

Chrome, Firefox, Safari — they all share the same problem. That top bar. The tabs, the URL bar, the toolbar — sitting there whether you need it or not, occupying valuable screen space, looking like something welded together from Soviet-era scraps. You didn't ask for it. It just exists.

I changed that.

- **Hold Ctrl + Scroll** → a tab strip fades in from the left. Release, and it disappears.
- **Middle mouse click** → a floating search island appears at the center of the screen. Type, navigate, done. It vanishes.
- **Everything else** stays hidden until you need it.

The screen is yours. The browser gets out of the way.

Beyond the UI, Anveshan is built on three hard principles:

- **Privacy** — no telemetry, no phoning home, no data collection. Ever.
- **Minimalism** — if it doesn't need to exist, it doesn't.
- **Speed** — light enough to be compared to a feather.

---

## 🧩 Architecture

Anveshan is **not** a Chromium browser. It uses only the raw rendering layers:

```
[ Google Chrome      ]  ← telemetry, sync, branding       ✗ excluded
[ Chromium Browser   ]  ← tabs, navigation, process model  ✗ excluded
─────────────────────────────────────────────────────────────────────
[ Content / Blink    ]  ← HTML + CSS rendering              ✓ used
[ V8                 ]  ← JavaScript engine                 ✓ used
[ BoringSSL + QUIC   ]  ← networking                        ✓ used
─────────────────────────────────────────────────────────────────────
[ Anveshan Shell     ]  ← our custom UI, written in Rust    ✓ ours
```

The Google layers don't exist in Anveshan. We take the renderer and build everything else ourselves.

---

## ⚙️ How It Works

The shell is written entirely in **Rust** using:

| Component | Library |
|---|---|
| Window + Input | `winit` |
| 2D UI Rendering | `tiny-skia` |
| Font Rendering | `fontdue` |
| Web Engine (WIP) | CEF (Blink + V8) |

---

## ⚡ Features

- 🔎 **Zero-chrome UI** — no visible tabs or URL bar by default
- ⚡ **Blazing fast** — Rust from top to bottom
- 🔒 **Private by design** — no telemetry, no trackers, no Google services
- 🧩 **Built from scratch** — not a Chromium fork, not Electron
- 🎨 **Beautiful** — UI that people admire, not tolerate

---

## 🛠️ Build & Run

**Requirements:**
- Rust (stable) — [rustup.rs](https://rustup.rs)
- Visual Studio 2022 Build Tools (Windows)
- CMake 3.21+

```bash
git clone https://github.com/ArjunPuri2011/anveshan-browser
cd anveshan-browser/Browser/Shell
cargo run --release
```

**Controls:**
| Action | Result |
|---|---|
| `Ctrl + Scroll` | Tab strip appears / disappears |
| `Middle Click` | Search island opens |
| `ESC` | Close island |
| `Ctrl + T` | New tab |
| `Ctrl + W` | Close tab |
| Bottom-left `⚙` | Settings panel |

---

## 🎯 Roadmap

- [x] Custom Rust UI shell
- [x] Animated tab strip (Ctrl+Scroll)
- [x] Floating search island (Middle click)
- [x] Settings panel
- [ ] CEF/Blink web engine integration
- [ ] Basic HTML rendering
- [ ] Built-in tracker blocker
- [ ] History and bookmarks (settings only, no toolbar)
- [ ] Widevine + PlayReady DRM support
- [ ] Custom rendering engine (long term)
- [ ] Onion routing mode (privacy-first, faster than TOR)
- [ ] Enterprise / corporate secure version

---

## 🤝 Contributing

This is a learning-driven project. Every line of code is intentional.

Feel free to fork, experiment, open issues, and suggest improvements. If you believe software should be understood and not just used — you're in the right place.

---

## 👤 Author

**Arjun Puri**
Passionate about systems programming, building from scratch, and making software that actually respects the person using it.

---

## 🧭 Philosophy

> Software should be understood, not just used.

Most browsers are black boxes. Anveshan is an attempt to break that — to build something from the ground up, understand every layer of it, and make it beautiful in the process.

Complexity into clarity. Clarity into beauty.

---

## 📄 License

Mozilla Public License 2.0 — the same license Firefox uses. Fitting.

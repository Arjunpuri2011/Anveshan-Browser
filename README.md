# 🌐 Anveshan Browser

> Explore the web.  
> Don’t just use it.

---

## 🧠 Vision

Hi, I’m Arjun.

Anveshan is built from my experience with modern browsers like  
:contentReference[oaicite:2]{index=2}, :contentReference[oaicite:3]{index=3}, and :contentReference[oaicite:4]{index=4}.

They are powerful.  
But they carry assumptions.

Tabs. Bars. Layers of UI.  
Always visible. Always there.

Anveshan questions that.

Why should navigation occupy space when it’s not needed?  
Why should the interface exist when you're not using it?

So instead:

- Tabs appear only when invoked  
- Search exists only when needed  
- The web takes the full stage  

Minimal. Intent-driven. Invisible.

---

## ✨ Philosophy

Most browsers add layers.

Anveshan removes them.

It is not built to compete.  
It is built to understand — and rethink.

---

## 🔐 Focus

Anveshan focuses on three things:

- **Privacy** — no telemetry, no tracking  
- **Lightness** — minimal resource consumption  
- **Clarity** — clean, distraction-free experience  

Modern browsers often trade simplicity for features.

Anveshan does the opposite.

---

## 🧩 Architecture

Anveshan is **not a Chromium-based browser**.

For reference, modern browsers are typically structured like this:
[ Browser Layer ]
└── UI, Sync, Telemetry, Services

[ Core Layer ]
└── Tabs, Navigation, Network

[ Rendering Engine ]
└── HTML / CSS (e.g., Blink)

[ JavaScript Engine ]
└── Execution (e.g., V8)


Anveshan avoids embedding a full browser stack.

Instead, it aims to build core browser behavior independently,  
while **exploring selective integration of low-level components** in the future.

---

## ⚙️ How It Works

Built in **Rust**, Anveshan focuses on:

- Low-level control over memory and performance  
- Direct handling of browser logic  
- Experimental UI/UX models  
- Minimal abstraction  

This is a **learning-first project** — focused on understanding the web from the ground up.

---

## ⚡ Features

- 🔎 Minimal, distraction-free UI  
- ⚡ Lightweight and fast  
- 🧩 Built from scratch in Rust  
- 🛠️ Developer-focused architecture  

---

## 🛠️ Build & Run

```bash
git clone https://github.com/ArjunPuri2011/anveshan-browser
cd anveshan-browser/Browser/Shell
cargo build
cargo run

```
🗺️ Roadmap
-[] Core browser architecture
-[] Basic HTML rendering
-[] UI refinement
-[] Experimental rendering engine
-[] Secure networking layer
-[] Privacy-focused browsing mode


🤝 Contributing
This is a learning-driven project.
Fork it.
Break it.
Improve it.

👤 Author
Arjun Puri
Systems programming enthusiast
Building from scratch, one layer at a time

🧭 Philosophy
Software should be understood — not just used.
Anveshan is about turning complexity into clarity.

💭 Why Rust?
Because control matters.
Rust offers performance, safety, and precision —
without hiding what’s happening underneath.

🧩 Final Thought
Most browsers hide the web.
Anveshan reveals it.
— Arjun Puri

## NOTE:

Anveshan is dual-licensed under GPLv2 and MPL 2.0.

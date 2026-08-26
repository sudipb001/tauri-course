# Tauri v2 for Windows: Build Lightweight Desktop Apps with React, TypeScript & Rust

This repository contains all the code written throughout the course **"Tauri v2 for Windows: Build Lightweight Desktop Apps with React, TypeScript & Rust."**

The course teaches React and TypeScript developers how to build real Windows desktop applications using **Tauri v2** and just enough **Rust** to unlock native desktop features. By the end of the course, you will have built a complete Windows application called **FileFlow**, packaged it into an installer, and installed it on your own computer.

## What You'll Build

**FileFlow** — a Windows desktop app that:

- Selects files with a native file picker
- Accepts files through drag and drop
- Reads and transforms CSV data
- Processes large files in the background without freezing the UI
- Shows live progress while processing
- Exports results to CSV or JSON
- Saves user preferences locally
- Runs from the Windows system tray
- Shows native Windows notifications

## Prerequisites

Before starting, you should have:

- Intermediate JavaScript or TypeScript knowledge
- Basic React knowledge
- Basic HTML and CSS knowledge
- Basic command-line knowledge
- A Windows computer for development

No previous Rust knowledge is required — every Rust concept used in the course is explained the first time it's introduced.

## Tools Used

- Node.js (LTS) and npm
- Rust, installed via rustup, and Cargo
- Microsoft C++ Build Tools
- WebView2 (usually already installed on Windows 10/11)
- Visual Studio Code

Setup instructions for all of these are covered in **Lecture 5**.

## Repository Structure

All course code lives inside the `tauri-course/` folder. Each lecture that includes hands-on code has its own subfolder, numbered to match the lecture:

```
tauri-course/
├── lecture-10/
├── lecture-11/
├── lecture-12/
├── lecture-13/
├── lecture-14/
├── lecture-15/
├── lecture-16/
├── ...
```

- Lectures that use a single standalone Rust file (compiled with `rustc`) contain a `main.rs` file directly inside their folder.
- Lectures that need external crates (like `serde` or `tokio`) contain a full Cargo project, with a `Cargo.toml` file and a `src/main.rs` file.
- Starting in Module 3, the course moves into a real Tauri project, and the folder structure follows the standard Tauri layout (`src` for the React frontend, `src-tauri` for the Rust backend).
- When a lecture reuses a file from a previous lecture's folder, the file is copied forward (for example, `cp ../lecture-13/main.rs .`) rather than shared across folders. Each lecture folder is a complete, standalone record of what was built in that lecture.

## Course Outline

1. **Module 1 — Getting Started with Tauri v2**
   Understanding Tauri, comparing it to Electron, setting up your Windows environment, and creating your first Tauri application.

2. **Module 2 — Rust for Tauri Developers**
   The practical Rust you need: variables, types, functions, structs, `Option` and `Result`, collections, Serde/JSON, ownership and borrowing, and async basics.

3. **Module 3 — React and Rust Communication**
   Tauri IPC, commands, `invoke()`, passing parameters, returning structured data, events, and application state.

4. **Module 4 — Windows Native Features**
   Native file dialogs, filesystem access, drag and drop, notifications, system tray, global shortcuts, and window management.

5. **Module 5 — Security, Permissions, Storage and Sidecars**
   Tauri v2 capabilities, permissions, filesystem scopes, secure IPC, local storage, SQLite, and sidecars.

6. **Module 6 — Capstone Project: FileFlow**
   Building the full FileFlow application from start to finish.

7. **Module 7 — Packaging and Distribution**
   Building production Windows installers, code signing, and automatic updates.

8. **Module 8 — Bonus: AI Desktop Assistant**
   An optional module on building an AI-powered desktop assistant with Tauri.

## How to Use This Repository

Each lecture's folder is meant to be followed in order, since later lectures build on the Rust and Tauri concepts introduced earlier. To follow along:

1. Open the relevant lecture folder in VS Code.
2. Read through the lecture material for the commands and code to type.
3. Run the app or script using the commands provided in that lecture (for example, `rustc main.rs` and `.\main.exe`, or `cargo run`, or `npm run tauri dev`).

## License

This repository is for personal, educational use as part of the course. Feel free to reference and reuse the code for your own learning and projects.

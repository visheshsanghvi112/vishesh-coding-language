# Plan for Project: Vedic-Lang Next Generation (Vedic-NG)

This document outlines the strategic roadmap to elevate the **Vedic** programming language from a conceptual prototype to a robust, system-ready language ("Next Level").

---

## 🏗️ Phase 1: Core Robustness ("The Dharma Update")

**Goal**: Fix critical flaws that prevent basic usability and reliability.
**Timeline**: Immediate.

### 1.1 Lexical & Syntax Stability
*   **Current State**: `lexer.rs` fails to handle string escapes (`\n`, `\"`). This makes parsing complex text (like JSON or escaped SQL) impossible.
*   **Action**:
    *   Rewrite `lexer.rs` string parsing logic to support standard escape sequences (`\\`, `\"`, `\n`, `\t`, `\r`, `\0`).
    *   Add support for hex/unicode escapes (`\u{0905}`).

### 1.2 Error Handling & Recovery
*   **Current State**: Error handling relies heavily on `panic!` or hard exits (`process::exit`).
*   **Action**:
    *   Refactor `parser.rs` to use `Result` types properly and recover from syntax errors (Synchronization) instead of crashing.
    *   Implement decent stack traces (`dosa_nirupaka.rs`) that show the call stack, not just the current line.

---

## 🛠️ Phase 2: Essential Capabilities ("The Karma Update")

**Goal**: Add features required for "Real World" applications (Bank, Files, Networking).
**Timeline**: Short-term.

### 2.1 Standard Library Expansion (Sanchika & Jaala)
*   **File I/O (`std_file`)**:
    *   Implement strict functions `लेखन` (Write) and `वाचन` (Read) in `core/moolsutra`.
    *   *Constraint*: Start with text-only I/O.
*   **Math Library (`std_ganit`)**:
    *   Add `sin`, `cos`, `tan`, `sqrt`, `pow`, `abs`. Access Rust’s `f64` methods directly.
*   **System Interaction (`std_yantra`)**:
    *   Add `exec` (Execute shell commands), `env` (Get Environment variables).

### 2.2 Memory & Data Structures
*   **Current State**: We have `Suchi` (List). It's a pointer to a `vector`.
*   **Action**:
    *   Add `Kosha` (Map/Dictionary). Without HashMaps, building complex data models is painful.
    *   Implement efficient Garbage Collection hooks (Mark & Sweep is implemented in `smrti_prabandhan.rs` but needs verification for cycle detection).

---

## 🚀 Phase 3: Architecture & Security ("The Moksha Update")

**Goal**: Make it fast and secure (The "Security" aspect you asked for).
**Timeline**: Long-term.

### 3.1 Strict Security Mode ("Satya Mode")
*   **Concept**: A compiler flag that enforces "Pure Functions".
*   **Action**:
    *   If `@satya` annotation is used on a `Sutra`, the parser rejects any code that modifies global variables or performs I/O.
    *   This guarantees that the function is mathematically pure and side-effect free—essential for Financial/Contract code.

### 3.2 Compilation Target (Bytecode)
*   **Current State**: AST Intepreter (Tree Walk). Slow.
*   **Action**:
    *   Write a Compiler (`lekhaka.rs`) that emits a linear Bytecode (`VedicCode`).
    *   Write a VM (`yantra.rs`) to execute this bytecode.
    *   *Result*: 10x-50x speed improvement.

---

## 🛡️ "Vedic Yuddha" Proof of Concept

To validate these changes, we will build **Vedic Yuddha 2.0**.
*   It will load warrior stats from a **File** (Impossible currently).
*   It will save the battle log to a **File**.
*   It will use **Secure Randomness** (via Foreign Function Interface to Rust).

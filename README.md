# Bare-Metal LLM Chat OS

A minimal operating system written in Rust that runs directly on hardware with the sole purpose of providing a chat interface powered by a built-in LLM (Language Learning Model).

## Overview

This is an experimental project for fun, created with the assistance of Claude AI and GitHub Copilot. The OS runs an LLM for chat interactions without requiring a full operating system. The OS handles:

- Memory management for LLM operations
- Keyboard input for user interaction
- VGA text-mode display for the chat interface
- A simplified LLM implementation

## Components

- **VGA Buffer**: Handles text display in VGA text mode
- **Memory Management**: Page tables and heap allocation for the LLM
- **Keyboard Driver**: Processes user input
- **LLM Implementation**: A language model for generating responses
- **Chat Interface**: Manages the conversation between user and LLM

## Running the OS

To run this OS, you need:

1. Rust installed with the `nightly` toolchain
2. QEMU for x86_64 emulation

Build and run with:

```
cargo bootimage
cargo run
```

If you want to copy text from the QEMU terminal, use:

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin -display curses
```

## Features

- Minimalist bare-metal environment
- Direct hardware interaction
- Memory-efficient LLM implementation
- Simple text-based chat interface
- Chat history tracking

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the development roadmap and current progress.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
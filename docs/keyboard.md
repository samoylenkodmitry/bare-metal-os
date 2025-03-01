# Keyboard Handler (keyboard.rs)

## Overview
The keyboard module manages PS/2 keyboard input, providing a low-level interface for reading keystrokes and handling keyboard events.

## Key Components
- `KeyboardReader`: Manages input buffering and line reading
- `KEYBOARD`: Global static Mutex for keyboard state
- `LAST_SCANCODE`: Tracks previous input for key repeat handling

## Features
- US 104-key layout support
- Scancode Set 1 decoding
- Key repeat prevention
- Backspace handling
- Line buffering
- Non-blocking input

## Technical Details
- Uses port I/O (0x60 and 0x64)
- Implements PS/2 controller interaction
- 256-character input buffer
- Thread-safe using spin locks
- Configurable key repeat handling

## External Libraries Used
1. `pc-keyboard` (0.5.0)
   - Provides keyboard layout handling and scancode translation
   - Handles key event processing and keyboard state
   - A bare metal implementation would require:
     * Manual PS/2 controller initialization
     * Raw scancode processing and translation
     * Custom keyboard layout implementation
     * Key state tracking and modifier key handling

2. `x86_64` (0.14.2)
   - Provides safe abstractions for port I/O operations
   - A bare metal implementation would require:
     * Direct port I/O using assembly instructions (in/out)
     * Manual handling of hardware timing and synchronization

3. `spin` and `lazy_static`
   - Used for thread-safe state management
   - Same benefits as described in VGA buffer documentation

These libraries hide complex details that would otherwise require:
- Writing raw assembly for port I/O
- Implementing complete PS/2 keyboard protocol
- Building scancode translation tables
- Managing hardware timing requirements

## Improvement Possibilities
1. Add support for multiple keyboard layouts
2. Implement key modifiers (Shift, Ctrl, Alt)
3. Add function key support
4. Implement keyboard LEDs control
5. Add PS/2 mouse support
6. Support USB keyboards
7. Add key mapping customization
8. Implement keyboard shortcuts
9. Add input event queuing
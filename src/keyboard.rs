use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, layouts, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use crate::{println, vga_buffer};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    
    // Track the last scancode to prevent continuous key repeat
    static ref LAST_SCANCODE: Mutex<Option<u8>> = Mutex::new(None);
}

// Status port for keyboard controller
const KEYBOARD_STATUS_PORT: u16 = 0x64;
// Data port for keyboard controller
const KEYBOARD_DATA_PORT: u16 = 0x60;
// Status bits
const KEYBOARD_OUTPUT_FULL: u8 = 1;

pub fn init() {
    // Reset the keyboard state
    *LAST_SCANCODE.lock() = None;
    println!("Keyboard initialized.");
}

// Check if a key has been pressed
fn is_key_available() -> bool {
    let mut status_port = Port::new(KEYBOARD_STATUS_PORT);
    let status: u8 = unsafe { status_port.read() };
    status & KEYBOARD_OUTPUT_FULL == KEYBOARD_OUTPUT_FULL
}

// Read a character from the keyboard with proper key repeat handling
pub fn read_char() -> Option<char> {
    // Only proceed if there's keyboard input available
    if !is_key_available() {
        return None;
    }
    
    let mut data_port = Port::new(KEYBOARD_DATA_PORT);
    let scancode: u8 = unsafe { data_port.read() };
    
    // Check if this is a repeated scancode (key is held down)
    let mut last_scancode = LAST_SCANCODE.lock();
    
    if let Some(last) = *last_scancode {
        if last == scancode {
            // Same key as before, ignore to prevent repeats
            return None;
        }
    }
    
    // Update the last scancode
    *last_scancode = Some(scancode);
    
    // Process the scancode
    let mut keyboard = KEYBOARD.lock();
    
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => return Some(character),
                DecodedKey::RawKey(_) => return None,
            }
        }
    }
    
    None
}

pub struct KeyboardReader {
    buffer: [char; 256],
    position: usize,
}

impl KeyboardReader {
    pub fn new() -> Self {
        Self {
            buffer: ['\0'; 256],
            position: 0,
        }
    }
    
    pub fn read_line(&mut self) -> &str {
        self.position = 0;
        
        loop {
            // Add a small delay to prevent CPU overuse
            for _ in 0..100000 {
                // This empty loop acts as a simple delay
            }
            
            if let Some(c) = read_char() {
                match c {
                    '\n' => break,
                    '\u{8}' => { // Backspace
                        if self.position > 0 {
                            self.position -= 1;
                            // Use our custom backspace function
                            vga_buffer::backspace();
                        }
                    },
                    c => {
                        if self.position < self.buffer.len() - 1 {
                            self.buffer[self.position] = c;
                            self.position += 1;
                            crate::print!("{}", c);
                        }
                    }
                }
            }
            
            // Reset the last scancode when no key is being pressed
            if !is_key_available() {
                *LAST_SCANCODE.lock() = None;
            }
        }
        
        // Make a string slice from the buffer up to the current position
        unsafe { core::str::from_utf8_unchecked(&core::slice::from_raw_parts(
            self.buffer.as_ptr() as *const u8, 
            self.position
        )) }
    }
}
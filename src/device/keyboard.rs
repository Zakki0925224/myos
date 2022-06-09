use crate::util::logger::log_debug;

// https://wiki.osdev.org/PS2_Keyboard
// scan code set 1
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ScanCode
{
    pub key_code: KeyCode,
    pub pressed: [u8; 6],
    pub released: [u8; 6],
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyCode
{
    Esc,            // p: 0x01, r: 0x81
    F1,             // p: 0x3b, r: 0xbb
    F2,             // p: 0x3c, r: 0xbc
    F3,             // p: 0x3d, r: 0xbd
    F4,             // p: 0x3e, r: 0xbe
    F5,             // p: 0x3f, r: 0xbf
    F6,             // p: 0x40, r: 0xc0
    F7,             // p: 0x41, r: 0xc1
    F8,             // p: 0x42, r: 0xc2
    F9,             // p: 0x43, r: 0xc3
    F10,            // p: 0x44, r: 0xc4
    F11,            // p: 0x57, r: 0xd7
    F12,            // p: 0x58, r: 0xd8
    PrintScreen,    // p: 0xe0, 0x2a, 0xe0, 0x37, r: 0xe0, 0xb7, 0xe0, 0xaa
    ScrollLock,     // p: 0x46, r: 0xc6
    Pause,          // p: 0xe1, 0x1d, 0x45, 0x9d, 0xc5, r: none
    Insert,         // p: 0xe0, 0x52, r: 0xe0, 0xd2
    Home,           // p: 0xe0, 0x47, r: 0xe0, 0xc7
    PageUp,         // p: 0xe0, 0x49, r: 0xe0, 0xc9
    Delete,         // p: 0xe0, 0x53, r: 0xe0, 0xd3
    End,            // p: 0xe0, 0x4f, r: 0xe0, 0xcf
    PageDown,       // p: 0xe0, 0x51, r: 0xe0, 0xd1
    CursorRight,    // p: 0xe0, 0x4d, r: 0xe0, 0xcd
    CursorLeft,     // p: 0xe0, 0x4b, r: 0xe0, 0xcb
    CursorDown,     // p: 0xe0, 0x50, r: 0xe0, 0xd0
    CursorUp,       // p: 0xe0, 0x48, r: 0xe0, 0xc8
    NumLock,        // p: 0x45, r: 0xc5
    KpDivide,       // p: 0xe0, 0x35, r: 0xe0, 0xb5
    KpMultiply,     // p: 0x37, r: 0xb7
    KpSubtract,     // p: 0x4a, r: 0xca
    KpAdd,          // p: 0x4e, r: 0xce
    KpEnter,        // p: 0xe0, 0x1c, r: 0xe0, 0x9c
    Kp1,            // p: 0x4f, r: 0xcf
    Kp2,            // p: 0x50, r: 0xd0
    Kp3,            // p: 0x51, r: 0xd1
    Kp4,            // p: 0x4b, r: 0xcb
    Kp5,            // p: 0x4c, r: 0xcc
    Kp6,            // p: 0x4d, r: 0xcd
    Kp7,            // p: 0x47, r: 0xc7
    Kp8,            // p: 0x48, r: 0xc8
    Kp9,            // p: 0x49, r: 0xc9
    Kp0,            // p: 0x52, r: 0xd2
    KpPeriod,       // p: 0x53, r: 0xd3
    LCtrl,          // p: 0x1d, r: 0x9d
    LGui,           // p: 0xe0, 0x5b, r: 0xe0, 0xdb
    LAlt,           // p: 0x38, r: 0xb8
    Space,          // p: 0x39, r: 0xb9
    RGui,           // p: 0xe0, 0x5c, r: 0xe0, 0xdc
    RAlt,           // p: 0xe0, 0x38, r: 0xe0, 0xb8
    Apps,           // p: 0xe0, 0x6d, r: 0xe0, 0xdd
    RCtrl,          // p: 0xe0, 0x1d, r: 0xe0, 0x9d
    LShift,         // p: 0x2a, r: 0xaa
    CapsLock,       // p: 0x3a, r: 0xba
    Tab,            // p: 0x0f, r: 0x8f
    Backspace,      // p: 0x0e, r: 0x8e
    Enter,          // p: 0x1c, r: 0x9c
    RShift,         // p: 0x36, r: 0xb6
    Num1,           // p: 0x02, r: 0x82
    Num2,           // p: 0x03, r: 0x83
    Num3,           // p: 0x04, r: 0x84
    Num4,           // p: 0x05, r: 0x85
    Num5,           // p: 0x06, r: 0x86
    Num6,           // p: 0x07, r: 0x87
    Num7,           // p: 0x08, r: 0x88
    Num8,           // p: 0x09, r: 0x89
    Num9,           // p: 0x0a, r: 0x8a
    Num0,           // p: 0x0b, r: 0x8b
    A,              // p: 0x1e, r: 0x9e
    B,              // p: 0x30, r: 0xb0
    C,              // p: 0x2e, r: 0xae
    D,              // p: 0x20, r: 0xa0
    E,              // p: 0x12, r: 0x92
    F,              // p: 0x21, r: 0xa1
    G,              // p: 0x22, r: 0xa2
    H,              // p: 0x23, r: 0xa3
    I,              // p: 0x17, r: 0x97
    J,              // p: 0x24, r: 0xa4
    K,              // p: 0x25, r: 0xa5
    L,              // p: 0x26, r: 0xa6
    M,              // p: 0x32, r: 0xb2
    N,              // p: 0x31, r: 0xb1
    O,              // p: 0x18, r: 0x98
    P,              // p: 0x19, r: 0x99
    Q,              // p: 0x10, r: 0x90
    R,              // p: 0x13, r: 0x93
    S,              // p: 0x1f, r: 0x9f
    T,              // p: 0x14, r: 0x94
    U,              // p: 0x16, r: 0x96
    V,              // p: 0x2f, r: 0xaf
    W,              // p: 0x11, r: 0x91
    X,              // p: 0x2d, r: 0xad
    Y,              // p: 0x15, r: 0x95
    Z,              // p: 0x2c, r: 0xac
    Backtick,     // ` p: 0x29, r: 0xa9
    Subtract,     // - p: 0x0c, r: 0x8c
    Equal,        // = p: 0x0d, r: 0x8d
    BracketLeft,  // [ p: 0x1a, r: 0x9a
    BracketRight, // ] p: 0x1b, r: 0x9b
    Backslash,    // \ p: 0x2b, r: 0xab
    Semicolon,    // ; p: 0x27, r: 0xa7
    Quote,        // ' p: 0x28, r: 0xa8
    Comma,        // , p: 0x33, r: 0xb3
    Period,       // . p: 0x34, r: 0xb4
    Slash,        // / p: 0x35, r: 0xb5
    Unknown
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct AnsiUs104KeyMap
{
    map: [ScanCode; 104],
}

impl AnsiUs104KeyMap
{
    pub fn new() -> AnsiUs104KeyMap
    {
        return AnsiUs104KeyMap
        {
            map:
            [
                ScanCode { key_code: KeyCode::Esc, pressed: [0x01, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x81, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F1, pressed: [0x3b, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xbb, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F2, pressed: [0x3c, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xbc, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F3, pressed: [0x3d, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xbd, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F4, pressed: [0x3e, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xbe, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F5, pressed: [0x3f, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xbf, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F6, pressed: [0x40, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F7, pressed: [0x41, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc1, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F8, pressed: [0x42, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc2, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F9, pressed: [0x43, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc3, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F10, pressed: [0x44, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc4, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F11, pressed: [0x57, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xd7, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F12, pressed: [0x58, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xd8, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::PrintScreen, pressed: [0xe0, 0x1d, 0x46, 0x9d, 0xc5, 0x00], released: [0xe0, 0xb7, 0xe0, 0xaa, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::ScrollLock, pressed: [0x46, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc6, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Pause, pressed: [0xe1, 0x1d, 0x45, 0x9d, 0xc5, 0x00], released: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Insert, pressed: [0xe0, 0x52, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xd2, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Home, pressed: [0xe0, 0x47, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xc7, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::PageUp, pressed: [0xe0, 0x49, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xc9, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Delete, pressed: [0xe0, 0x53, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xd3, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::End, pressed: [0xe0, 0x4f, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xcf, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::PageDown, pressed: [0xe0, 0x51, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xd1, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::CursorRight, pressed: [0xe0, 0x4d, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xcd, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::CursorLeft, pressed: [0xe0, 0x4b, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xcb, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::CursorDown, pressed: [0xe0, 0x50, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xd0, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::CursorUp, pressed: [0xe0, 0x48, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xc8, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::NumLock, pressed: [0x45, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc5, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::KpDivide, pressed: [0xe0, 0x35, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xb5, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::KpMultiply, pressed: [0x37, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb7, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::KpSubtract, pressed: [0x4a, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xca, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::KpAdd, pressed: [0x4e, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xce, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::KpEnter, pressed: [0xe0, 0x1c, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0x9c, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp1, pressed: [0x4f, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xcf, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp2, pressed: [0x50, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xd0, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp3, pressed: [0x51, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xd1, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp4, pressed: [0x4b, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xcb, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp5, pressed: [0x4c, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xcc, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp6, pressed: [0x4d, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xcd, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp7, pressed: [0x47, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc7, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp8, pressed: [0x48, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc8, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp9, pressed: [0x49, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xc9, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Kp0, pressed: [0x52, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xd2, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::KpPeriod, pressed: [0x53, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xd3, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::LCtrl, pressed: [0x1d, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x9d, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::LGui, pressed: [0xe0, 0x5b, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xdb, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::LAlt, pressed: [0x38, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb8, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Space, pressed: [0x39, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb9, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::RGui, pressed: [0xe0, 0x5c, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xdc, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::RAlt, pressed: [0xe0, 0x38, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xb8, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Apps, pressed: [0xe0, 0x6d, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0xdd, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::RCtrl, pressed: [0xe0, 0x1d, 0x00, 0x00, 0x00, 0x00], released: [0xe0, 0x9d, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::LShift, pressed: [0x2a, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xaa, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::CapsLock, pressed: [0x3a, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xba, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Tab, pressed: [0x0f, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x8f, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Backspace, pressed: [0x0e, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x8e, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Enter, pressed: [0x1c, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x9c, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::RShift, pressed: [0x36, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb6, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num1, pressed: [0x02, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x82, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num2, pressed: [0x03, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x83, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num3, pressed: [0x04, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x84, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num4, pressed: [0x05, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x85, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num5, pressed: [0x06, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x86, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num6, pressed: [0x07, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x87, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num7, pressed: [0x08, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x88, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num8, pressed: [0x09, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x89, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num9, pressed: [0x0a, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x8a, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Num0, pressed: [0x0b, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x8b, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::A, pressed: [0x1e, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x9e, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::B, pressed: [0x30, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb0, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::C, pressed: [0x2e, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xae, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::D, pressed: [0x20, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa0, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::E, pressed: [0x12, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x92, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::F, pressed: [0x21, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa1, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::G, pressed: [0x22, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa2, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::H, pressed: [0x23, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa3, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::I, pressed: [0x17, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x97, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::J, pressed: [0x24, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa4, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::K, pressed: [0x25, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa5, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::L, pressed: [0x26, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa6, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::M, pressed: [0x32, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb2, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::N, pressed: [0x31, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb1, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::O, pressed: [0x18, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x98, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::P, pressed: [0x19, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x99, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Q, pressed: [0x10, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x90, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::R, pressed: [0x13, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x93, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::S, pressed: [0x1f, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x9f, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::T, pressed: [0x14, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x94, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::U, pressed: [0x16, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x96, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::V, pressed: [0x2f, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xaf, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::W, pressed: [0x11, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x91, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::X, pressed: [0x2d, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xad, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Y, pressed: [0x15, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x95, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Z, pressed: [0x2c, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xac, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Backtick, pressed: [0x29, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa9, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Subtract, pressed: [0x0c, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x8c, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Equal, pressed: [0x0d, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x8d, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::BracketLeft, pressed: [0x1a, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x9a, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::BracketRight, pressed: [0x1b, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0x9b, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Backslash, pressed: [0x2b, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xab, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Semicolon, pressed: [0x27, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa7, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Quote, pressed: [0x28, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xa8, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Comma, pressed: [0x33, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb3, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Period, pressed: [0x34, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb4, 0x00, 0x00, 0x00, 0x00, 0x00] },
                ScanCode { key_code: KeyCode::Slash, pressed: [0x35, 0x00, 0x00, 0x00, 0x00, 0x00], released: [0xb5, 0x00, 0x00, 0x00, 0x00, 0x00] }

            ]
        };
    }

    pub fn get_key_map(&self) -> [ScanCode; 104]
    {
        return self.map;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyState
{
    Pressed,
    Released,
    Unknown
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyLayout
{
    AnsiUs104
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct KeyEvent
{
    pub code: KeyCode,
    pub state: KeyState,
    pub layout: KeyLayout
}

impl KeyEvent
{
    pub fn new(code: KeyCode, state: KeyState, layout: KeyLayout) -> KeyEvent
    {
        return KeyEvent { code, state, layout };
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ModifierKeysState
{
    pub on_shift: bool,
    pub on_ctrl: bool,
    pub on_gui: bool,
    pub on_alt: bool,
    // pub on_numlock: bool
}

pub struct Keyboard
{
    pub layout: KeyLayout,
    pub key_map: [ScanCode; 104],
    key_buf: [u8; 6],
    key_buf_cnt: usize,
    modifier_keys_state: ModifierKeysState
}

impl Keyboard
{
    pub fn new(layout: KeyLayout) -> Keyboard
    {
        if layout != KeyLayout::AnsiUs104
        {
            panic!("Unsupported keyboard layout");
        }

        return Keyboard
        {
            layout,
            key_map: AnsiUs104KeyMap::new().get_key_map(),
            key_buf: [0; 6],
            key_buf_cnt: 0,
            modifier_keys_state: ModifierKeysState { on_shift: false, on_ctrl: false, on_gui: false, on_alt: false }
        };
    }

    fn clear_key_buf(&mut self)
    {
        self.key_buf_cnt = 0;
        self.key_buf = [0; 6];
    }

    pub fn input(&mut self, data: u8) -> Option<(KeyEvent, ModifierKeysState)>
    {
        if self.key_buf_cnt > 5
        {
            self.clear_key_buf();
        }

        self.key_buf[self.key_buf_cnt] = data;

        if self.layout == KeyLayout::AnsiUs104
        {
            if self.key_buf_cnt == 0 &&
               (data == 0xe0 || data == 0xe1)
            {
                self.key_buf_cnt += 1;
                return None;;
            }

            if self.key_buf_cnt == 1 &&
               (data == 0x2a || data == 0xb7 || (self.key_buf[0] == 0xe1 && data == 0x1d))
            {
                self.key_buf_cnt += 1;
                return None;
            }

            if self.key_buf_cnt == 2 &&
               (data == 0xe0 || data == 0x45)
            {
                self.key_buf_cnt += 1;
                return None;
            }
        }

        let e = self.get_key_event();
        self.set_modifier_key(e);
        self.clear_key_buf();

        return Some((e, self.modifier_keys_state));
    }

    fn set_modifier_key(&mut self, event: KeyEvent)
    {
        if event.state == KeyState::Pressed
        {
            match event.code
            {
                KeyCode::LShift => self.modifier_keys_state.on_shift = true,
                KeyCode::RShift => self.modifier_keys_state.on_shift = true,
                KeyCode::LCtrl => self.modifier_keys_state.on_ctrl = true,
                KeyCode::RCtrl => self.modifier_keys_state.on_ctrl = true,
                KeyCode::LGui => self.modifier_keys_state.on_gui = true,
                KeyCode::RGui => self.modifier_keys_state.on_gui = true,
                KeyCode::LAlt => self.modifier_keys_state.on_alt = true,
                KeyCode::RAlt => self.modifier_keys_state.on_alt = true,
                _ => return
            }
        }
        else if event.state == KeyState::Released
        {
            match event.code
            {
                KeyCode::LShift => self.modifier_keys_state.on_shift = false,
                KeyCode::RShift => self.modifier_keys_state.on_shift = false,
                KeyCode::LCtrl => self.modifier_keys_state.on_ctrl = false,
                KeyCode::RCtrl => self.modifier_keys_state.on_ctrl = false,
                KeyCode::LGui => self.modifier_keys_state.on_gui = false,
                KeyCode::RGui => self.modifier_keys_state.on_gui = false,
                KeyCode::LAlt => self.modifier_keys_state.on_alt = false,
                KeyCode::RAlt => self.modifier_keys_state.on_alt = false,
                _ => return
            }
        }
    }

    fn get_key_event(&mut self) -> KeyEvent
    {
        let mut key_code = KeyCode::Unknown;
        let mut key_state = KeyState::Unknown;

        for key in self.key_map
        {
            if key.pressed == self.key_buf
            {
                key_code = key.key_code;
                key_state = KeyState::Pressed;
            }
            else if key.released == self.key_buf
            {
                key_code = key.key_code;
                key_state = KeyState::Released;
            }
        }

        return KeyEvent::new(key_code, key_state, self.layout);
    }
}
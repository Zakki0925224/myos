// https://wiki.osdev.org/PS2_Keyboard
// scan code set 1
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
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
    CursorDown,     // p: 0xe0, 0x51, r: 0xe0, 0xd1
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
    RCtrl,          // p: 0xe0, 0x1d
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
    Subtract,     // - p: 0x0c, r: 0xcc
    Equal,        // = p: 0x0d, r: 0xcd
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
    pub raw: u8,
    pub code: KeyCode,
    pub state: KeyState,
    pub layout: KeyLayout
}

impl KeyEvent
{
    pub fn new(raw: u8, code: KeyCode, state: KeyState, layout: KeyLayout) -> KeyEvent
    {
        return KeyEvent { raw, code, state, layout };
    }
}

pub fn get_key(scan_code: u8) -> KeyEvent
{
    let layout = KeyLayout::AnsiUs104;
    let mut state = KeyState::Unknown;
    let mut code = KeyCode::Unknown;

    if scan_code >= 0x01 && scan_code <= 0x58
    {
        state = KeyState::Pressed;

        code = match scan_code
        {
            0x01 => KeyCode::Esc,
            0x02 => KeyCode::Num1,
            0x03 => KeyCode::Num2,
            0x04 => KeyCode::Num3,
            0x05 => KeyCode::Num4,
            0x06 => KeyCode::Num5,
            0x07 => KeyCode::Num6,
            0x08 => KeyCode::Num7,
            0x09 => KeyCode::Num8,
            0x0a => KeyCode::Num9,
            0x0b => KeyCode::Num0,
            0x0c => KeyCode::Subtract,
            0x0d => KeyCode::Equal,
            0x0e => KeyCode::Backspace,
            0x0f => KeyCode::Tab,
            0x10 => KeyCode::Q,
            0x11 => KeyCode::W,
            0x12 => KeyCode::E,
            0x13 => KeyCode::R,
            0x14 => KeyCode::T,
            0x15 => KeyCode::Y,
            0x16 => KeyCode::U,
            0x17 => KeyCode::I,
            0x18 => KeyCode::O,
            0x19 => KeyCode::P,
            0x1a => KeyCode::BracketLeft,
            0x1b => KeyCode::BracketRight,
            0x1c => KeyCode::Enter,
            0x1d => KeyCode::LCtrl,
            0x1e => KeyCode::A,
            0x1f => KeyCode::S,
            0x20 => KeyCode::D,
            0x21 => KeyCode::F,
            0x22 => KeyCode::G,
            0x23 => KeyCode::H,
            0x24 => KeyCode::J,
            0x25 => KeyCode::K,
            0x26 => KeyCode::L,
            0x27 => KeyCode::Semicolon,
            0x28 => KeyCode::Quote,
            0x29 => KeyCode::Backtick,
            0x2a => KeyCode::LShift,
            0x2b => KeyCode::Backslash,
            0x2c => KeyCode::Z,
            0x2d => KeyCode::X,
            0x2e => KeyCode::C,
            0x2f => KeyCode::V,
            0x30 => KeyCode::B,
            0x31 => KeyCode::N,
            0x32 => KeyCode::M,
            0x33 => KeyCode::Comma,
            0x34 => KeyCode::Period,
            0x35 => KeyCode::Slash,
            0x36 => KeyCode::RShift,
            0x37 => KeyCode::KpMultiply,
            0x38 => KeyCode::LAlt,
            0x39 => KeyCode::Space,
            0x3a => KeyCode::CapsLock,
            0x3b => KeyCode::F1,
            0x3c => KeyCode::F2,
            0x3d => KeyCode::F3,
            0x3e => KeyCode::F4,
            0x3f => KeyCode::F5,
            0x40 => KeyCode::F6,
            0x41 => KeyCode::F7,
            0x42 => KeyCode::F8,
            0x43 => KeyCode::F9,
            0x44 => KeyCode::F10,
            0x45 => KeyCode::NumLock,
            0x46 => KeyCode::ScrollLock,
            0x47 => KeyCode::Kp7,
            0x48 => KeyCode::Kp8,
            0x49 => KeyCode::Kp9,
            0x4a => KeyCode::KpSubtract,
            0x4b => KeyCode::Kp4,
            0x4c => KeyCode::Kp5,
            0x4d => KeyCode::Kp6,
            0x4e => KeyCode::KpAdd,
            0x4f => KeyCode::Kp1,
            0x50 => KeyCode::Kp2,
            0x51 => KeyCode::Kp3,
            0x52 => KeyCode::Kp0,
            0x53 => KeyCode::KpPeriod,
            0x57 => KeyCode::F11,
            0x58 => KeyCode::F12,
            _ => KeyCode::Unknown
        };
    }
    else if scan_code >= 0x81 && scan_code <= 0xd8
    {
        state = KeyState::Released;

        code = match scan_code
        {
            0x81 => KeyCode::Esc,
            0x82 => KeyCode::Num1,
            0x83 => KeyCode::Num2,
            0x84 => KeyCode::Num3,
            0x85 => KeyCode::Num4,
            0x86 => KeyCode::Num5,
            0x87 => KeyCode::Num6,
            0x88 => KeyCode::Num7,
            0x89 => KeyCode::Num8,
            0x8a => KeyCode::Num9,
            0x8b => KeyCode::Num0,
            0x8c => KeyCode::Subtract,
            0x8d => KeyCode::Equal,
            0x8e => KeyCode::Backspace,
            0x8f => KeyCode::Tab,
            0x90 => KeyCode::Q,
            0x91 => KeyCode::W,
            0x92 => KeyCode::E,
            0x93 => KeyCode::R,
            0x94 => KeyCode::T,
            0x95 => KeyCode::Y,
            0x96 => KeyCode::U,
            0x97 => KeyCode::I,
            0x98 => KeyCode::O,
            0x99 => KeyCode::P,
            0x9a => KeyCode::BracketLeft,
            0x9b => KeyCode::BracketRight,
            0x9c => KeyCode::Enter,
            0x9d => KeyCode::LCtrl,
            0x9e => KeyCode::A,
            0x9f => KeyCode::S,
            0xa0 => KeyCode::D,
            0xa1 => KeyCode::F,
            0xa2 => KeyCode::G,
            0xa3 => KeyCode::H,
            0xa4 => KeyCode::J,
            0xa5 => KeyCode::K,
            0xa6 => KeyCode::L,
            0xa7 => KeyCode::Semicolon,
            0xa8 => KeyCode::Quote,
            0xa9 => KeyCode::Backtick,
            0xaa => KeyCode::LShift,
            0xab => KeyCode::Backslash,
            0xac => KeyCode::Z,
            0xad => KeyCode::X,
            0xae => KeyCode::C,
            0xaf => KeyCode::V,
            0xb0 => KeyCode::B,
            0xb1 => KeyCode::N,
            0xb2 => KeyCode::M,
            0xb3 => KeyCode::Comma,
            0xb4 => KeyCode::Period,
            0xb5 => KeyCode::Slash,
            0xb6 => KeyCode::RShift,
            0xb7 => KeyCode::KpMultiply,
            0xb8 => KeyCode::LAlt,
            0xb9 => KeyCode::Space,
            0xba => KeyCode::CapsLock,
            0xbb => KeyCode::F1,
            0xbc => KeyCode::F2,
            0xbd => KeyCode::F3,
            0xbe => KeyCode::F4,
            0xbf => KeyCode::F5,
            0xc0 => KeyCode::F6,
            0xc1 => KeyCode::F7,
            0xc2 => KeyCode::F8,
            0xc3 => KeyCode::F9,
            0xc4 => KeyCode::F10,
            0xc5 => KeyCode::NumLock,
            0xc6 => KeyCode::ScrollLock,
            0xc7 => KeyCode::Kp7,
            0xc8 => KeyCode::Kp8,
            0xc9 => KeyCode::Kp9,
            0xca => KeyCode::KpSubtract,
            0xcb => KeyCode::Kp4,
            0xcc => KeyCode::Kp5,
            0xcd => KeyCode::Kp6,
            0xce => KeyCode::KpAdd,
            0xcf => KeyCode::Kp1,
            0xd0 => KeyCode::Kp2,
            0xd1 => KeyCode::Kp3,
            0xd2 => KeyCode::Kp0,
            0xd3 => KeyCode::KpPeriod,
            0xd7 => KeyCode::F11,
            0xd8 => KeyCode::F12,
            _ => KeyCode::Unknown
        }
    }

    return KeyEvent::new(scan_code, code, state, layout);
}
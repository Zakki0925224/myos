use crate::device::keyboard::{KeyEvent, ModifierKeysState, KeyState, KeyLayout, KeyCode};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum AsciiCode
{
    Null,
    StartOfHeading,
    StartOfText,
    EndOfText,
    EndOfTransmission,
    Enquiry,
    Acknowledge,
    Bell,
    Backspace,
    HorizontalTab,
    NewLine,
    VerticalTab,
    NewPage,
    CarriageReturn,
    ShiftOut,
    ShiftIn,
    DataLinkEscape,
    DeviceControl1,
    DeviceControl2,
    DeviceControl3,
    DeviceControl4,
    NegativeAcknowledge,
    SynchronousIdle,
    EndOfTransBlock,
    Cancel,
    EndOfMedium,
    Substitute,
    Escape,
    FileSeparator,
    GroupSeparator,
    RecordSeparator,
    UnitSeparator,
    Space,
    Exclamation,    // !
    Quotation,      // "
    Hash,           // #
    Doll,           // $
    Percent,        // %
    Ampersand,      // &
    Apostrophe,     // '
    LParenthesis,   // (
    RParenthesis,   // )
    Asterisk,       // *
    Plus,           // +
    Comma,          // ,
    Hyphen,         // -
    FullStop,       // .
    Solidius,       // /
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Colon,          // :
    Semiclon,       // ;
    LessThan,       // <
    Equal,          // =
    GreaterThan,    // >
    Question,       // ?
    At,             // @
    LargeA,
    LargeB,
    LargeC,
    LargeD,
    LargeE,
    LargeF,
    LargeG,
    LargeH,
    LargeI,
    LargeJ,
    LargeK,
    LargeL,
    LargeM,
    LargeN,
    LargeO,
    LargeP,
    LargeQ,
    LargeR,
    LargeS,
    LargeT,
    LargeU,
    LargeV,
    LargeW,
    LargeX,
    LargeY,
    LargeZ,
    LSquareBracket,     // [
    ReverseSolidus,     // \
    RSquareBracket,     // ]
    CircumflexAccent,   // ^
    LowLine,            // _
    GraveAccent,        // `
    SmallA,
    SmallB,
    SmallC,
    SmallD,
    SmallE,
    SmallF,
    SmallG,
    SmallH,
    SmallI,
    SmallJ,
    SmallK,
    SmallL,
    SmallM,
    SmallN,
    SmallO,
    SmallP,
    SmallQ,
    SmallR,
    SmallS,
    SmallT,
    SmallU,
    SmallV,
    SmallW,
    SmallX,
    SmallY,
    SmallZ,
    LCurlyBracket,      // {
    VerticalLine,       // |
    RCurltBracket,      // }
    Tilde,              // ~
    Delete
}

pub fn key_event_to_ascii_code(event: KeyEvent, modifier_keys_state: ModifierKeysState) -> Option<AsciiCode>
{
    if event.state != KeyState::Pressed || event.layout != KeyLayout::AnsiUs104
    {
        return None;
    }
    match event.code
    {
        KeyCode::Space => return Some(AsciiCode::Space),
        KeyCode::Tab => return Some(AsciiCode::HorizontalTab),
        KeyCode::Enter => return Some(AsciiCode::NewLine),
        _ => ()
    }

    if !modifier_keys_state.on_shift
    {
        match event.code
        {
            KeyCode::Num0 => return Some(AsciiCode::Num0),
            KeyCode::Num1 => return Some(AsciiCode::Num1),
            KeyCode::Num2 => return Some(AsciiCode::Num2),
            KeyCode::Num3 => return Some(AsciiCode::Num3),
            KeyCode::Num4 => return Some(AsciiCode::Num4),
            KeyCode::Num5 => return Some(AsciiCode::Num5),
            KeyCode::Num6 => return Some(AsciiCode::Num6),
            KeyCode::Num7 => return Some(AsciiCode::Num7),
            KeyCode::Num8 => return Some(AsciiCode::Num8),
            KeyCode::Num9 => return Some(AsciiCode::Num9),
            KeyCode::A => return Some(AsciiCode::SmallA),
            KeyCode::B => return Some(AsciiCode::SmallB),
            KeyCode::C => return Some(AsciiCode::SmallC),
            KeyCode::D => return Some(AsciiCode::SmallD),
            KeyCode::E => return Some(AsciiCode::SmallE),
            KeyCode::F => return Some(AsciiCode::SmallF),
            KeyCode::G => return Some(AsciiCode::SmallG),
            KeyCode::H => return Some(AsciiCode::SmallH),
            KeyCode::I => return Some(AsciiCode::SmallI),
            KeyCode::J => return Some(AsciiCode::SmallJ),
            KeyCode::K => return Some(AsciiCode::SmallK),
            KeyCode::L => return Some(AsciiCode::SmallL),
            KeyCode::M => return Some(AsciiCode::SmallM),
            KeyCode::N => return Some(AsciiCode::SmallN),
            KeyCode::O => return Some(AsciiCode::SmallO),
            KeyCode::P => return Some(AsciiCode::SmallP),
            KeyCode::Q => return Some(AsciiCode::SmallQ),
            KeyCode::R => return Some(AsciiCode::SmallR),
            KeyCode::S => return Some(AsciiCode::SmallS),
            KeyCode::T => return Some(AsciiCode::SmallT),
            KeyCode::U => return Some(AsciiCode::SmallU),
            KeyCode::V => return Some(AsciiCode::SmallV),
            KeyCode::W => return Some(AsciiCode::SmallW),
            KeyCode::X => return Some(AsciiCode::SmallX),
            KeyCode::Y => return Some(AsciiCode::SmallY),
            KeyCode::Z => return Some(AsciiCode::SmallZ),
            _ => ()
        }
    }
    else
    {
        match event.code
        {
            KeyCode::A => return Some(AsciiCode::LargeA),
            KeyCode::B => return Some(AsciiCode::LargeB),
            KeyCode::C => return Some(AsciiCode::LargeC),
            KeyCode::D => return Some(AsciiCode::LargeD),
            KeyCode::E => return Some(AsciiCode::LargeE),
            KeyCode::F => return Some(AsciiCode::LargeF),
            KeyCode::G => return Some(AsciiCode::LargeG),
            KeyCode::H => return Some(AsciiCode::LargeH),
            KeyCode::I => return Some(AsciiCode::LargeI),
            KeyCode::J => return Some(AsciiCode::LargeJ),
            KeyCode::K => return Some(AsciiCode::LargeK),
            KeyCode::L => return Some(AsciiCode::LargeL),
            KeyCode::M => return Some(AsciiCode::LargeM),
            KeyCode::N => return Some(AsciiCode::LargeN),
            KeyCode::O => return Some(AsciiCode::LargeO),
            KeyCode::P => return Some(AsciiCode::LargeP),
            KeyCode::Q => return Some(AsciiCode::LargeQ),
            KeyCode::R => return Some(AsciiCode::LargeR),
            KeyCode::S => return Some(AsciiCode::LargeS),
            KeyCode::T => return Some(AsciiCode::LargeT),
            KeyCode::U => return Some(AsciiCode::LargeU),
            KeyCode::V => return Some(AsciiCode::LargeV),
            KeyCode::W => return Some(AsciiCode::LargeW),
            KeyCode::X => return Some(AsciiCode::LargeX),
            KeyCode::Y => return Some(AsciiCode::LargeY),
            KeyCode::Z => return Some(AsciiCode::LargeZ),
            _ => ()
        }
    }

    return None;
}
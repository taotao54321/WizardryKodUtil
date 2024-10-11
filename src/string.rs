use std::fmt::Write as _;

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// 原作の文字列。
#[derive(Clone, Default, Eq, PartialEq)]
pub struct GameString(Vec<GameChar>);

impl GameString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bytes(buf: &[u8]) -> anyhow::Result<Self> {
        let inner = buf
            .iter()
            .copied()
            .map(GameChar::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(inner))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Debug for GameString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;

        for &ch in &self.0 {
            std::fmt::Display::fmt(&ch, f)?;
        }

        f.write_char('"')?;

        Ok(())
    }
}

impl std::fmt::Display for GameString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &ch in &self.0 {
            ch.fmt(f)?;
        }

        Ok(())
    }
}

/// 原作の文字。
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[allow(non_camel_case_types)]
pub enum GameChar {
    // ASCII
    Space = 0x20,
    Exclamation = 0x21,
    Quotation = 0x22,
    Hash = 0x23,
    Dollar = 0x24,
    Percent = 0x25,
    Ampersand = 0x26,
    Apostrophe = 0x27,
    ParenOpen = 0x28,
    ParenClose = 0x29,
    Asterisk = 0x2A,
    Plus = 0x2B,
    Comma = 0x2C,
    Minus = 0x2D,
    Dot = 0x2E,
    Slash = 0x2F,
    _0 = 0x30,
    _1 = 0x31,
    _2 = 0x32,
    _3 = 0x33,
    _4 = 0x34,
    _5 = 0x35,
    _6 = 0x36,
    _7 = 0x37,
    _8 = 0x38,
    _9 = 0x39,
    Colon = 0x3A,
    Semicolon = 0x3B,
    LessThan = 0x3C,
    Equal = 0x3D,
    GreaterThan = 0x3E,
    Question = 0x3F,
    /// 実際には copyright マーク。
    At = 0x40,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    BracketOpen = 0x5B,
    /// 実際には yen マーク。
    BackSlash = 0x5C,
    BracketClose = 0x5D,
    Caret = 0x5E,
    UnderScore = 0x5F,
    Grave = 0x60,
    a = 0x61,
    b = 0x62,
    c = 0x63,
    d = 0x64,
    e = 0x65,
    f = 0x66,
    g = 0x67,
    h = 0x68,
    i = 0x69,
    j = 0x6A,
    k = 0x6B,
    l = 0x6C,
    m = 0x6D,
    n = 0x6E,
    o = 0x6F,
    p = 0x70,
    q = 0x71,
    r = 0x72,
    s = 0x73,
    t = 0x74,
    u = 0x75,
    v = 0x76,
    w = 0x77,
    x = 0x78,
    y = 0x79,
    z = 0x7A,
    CurlyBraceOpen = 0x7B,
    VerticalBar = 0x7C,
    CurlyBraceClose = 0x7D,
    Tilde = 0x7E,

    Of = 0xA0,

    Potion = 0xA2,
    Scroll = 0xA3,
}

impl GameChar {
    /// 原作の文字を Unicode 文字に変換する。
    ///
    /// 一部の文字は直接対応する Unicode 文字を持たないので、適当にそれっぽく置き換える。
    pub fn to_char(self) -> char {
        match self {
            Self::Space => ' ',
            Self::Exclamation => '!',
            Self::Quotation => '"',
            Self::Hash => '#',
            Self::Dollar => '$',
            Self::Percent => '%',
            Self::Ampersand => '&',
            Self::Apostrophe => '\'',
            Self::ParenOpen => '(',
            Self::ParenClose => ')',
            Self::Asterisk => '*',
            Self::Plus => '+',
            Self::Comma => ',',
            Self::Minus => '-',
            Self::Dot => '.',
            Self::Slash => '/',
            Self::_0 => '0',
            Self::_1 => '1',
            Self::_2 => '2',
            Self::_3 => '3',
            Self::_4 => '4',
            Self::_5 => '5',
            Self::_6 => '6',
            Self::_7 => '7',
            Self::_8 => '8',
            Self::_9 => '9',
            Self::Colon => ':',
            Self::Semicolon => ';',
            Self::LessThan => '<',
            Self::Equal => '=',
            Self::GreaterThan => '>',
            Self::Question => '?',
            Self::At => '@',
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            Self::H => 'H',
            Self::I => 'I',
            Self::J => 'J',
            Self::K => 'K',
            Self::L => 'L',
            Self::M => 'M',
            Self::N => 'N',
            Self::O => 'O',
            Self::P => 'P',
            Self::Q => 'Q',
            Self::R => 'R',
            Self::S => 'S',
            Self::T => 'T',
            Self::U => 'U',
            Self::V => 'V',
            Self::W => 'W',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
            Self::BracketOpen => '[',
            Self::BackSlash => '\\',
            Self::BracketClose => ']',
            Self::Caret => '^',
            Self::UnderScore => '_',
            Self::Grave => '`',
            Self::a => 'a',
            Self::b => 'b',
            Self::c => 'c',
            Self::d => 'd',
            Self::e => 'e',
            Self::f => 'f',
            Self::g => 'g',
            Self::h => 'h',
            Self::i => 'i',
            Self::j => 'j',
            Self::k => 'k',
            Self::l => 'l',
            Self::m => 'm',
            Self::n => 'n',
            Self::o => 'o',
            Self::p => 'p',
            Self::q => 'q',
            Self::r => 'r',
            Self::s => 's',
            Self::t => 't',
            Self::u => 'u',
            Self::v => 'v',
            Self::w => 'w',
            Self::x => 'x',
            Self::y => 'y',
            Self::z => 'z',
            Self::CurlyBraceOpen => '{',
            Self::VerticalBar => '|',
            Self::CurlyBraceClose => '}',
            Self::Tilde => '~',
            Self::Of => '之',
            Self::Potion => '薬',
            Self::Scroll => '巻',
        }
    }
}

impl std::fmt::Debug for GameChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.to_char())
    }
}

impl std::fmt::Display for GameChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

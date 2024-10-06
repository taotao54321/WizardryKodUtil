use std::fmt::Write as _;

use flagset::{flags, FlagSet};

flags! {
    /// 職業。
    #[repr(u8)]
    pub enum Class: u8 {
        Fighter = 1 << 0,
        Mage = 1 << 1,
        Cleric = 1 << 2,
        Thief = 1 << 3,
        Wizard = 1 << 4,
        Samurai = 1 << 5,
        Lord = 1 << 6,
        Ninja = 1 << 7,
    }
}

impl Class {
    /// 職業ID (`0..=7`) から職業を作る。
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::Fighter),
            1 => Some(Self::Mage),
            2 => Some(Self::Cleric),
            3 => Some(Self::Thief),
            4 => Some(Self::Wizard),
            5 => Some(Self::Samurai),
            6 => Some(Self::Lord),
            7 => Some(Self::Ninja),
            _ => None,
        }
    }

    /// 職業ID (`0..=7`) を返す。
    pub fn to_id(self) -> u8 {
        match self {
            Self::Fighter => 0,
            Self::Mage => 1,
            Self::Cleric => 2,
            Self::Thief => 3,
            Self::Wizard => 4,
            Self::Samurai => 5,
            Self::Lord => 6,
            Self::Ninja => 7,
        }
    }

    /// 正式名称を返す。
    pub fn name(self) -> &'static str {
        match self {
            Self::Fighter => "Fighter",
            Self::Mage => "Mage",
            Self::Cleric => "Cleric",
            Self::Thief => "Thief",
            Self::Wizard => "Wizard",
            Self::Samurai => "Samurai",
            Self::Lord => "Lord",
            Self::Ninja => "Ninja",
        }
    }

    /// 名称の頭文字を返す。
    pub fn name_initial(self) -> &'static str {
        match self {
            Self::Fighter => "F",
            Self::Mage => "M",
            Self::Cleric => "C",
            Self::Thief => "T",
            Self::Wizard => "W",
            Self::Samurai => "S",
            Self::Lord => "L",
            Self::Ninja => "N",
        }
    }

    /// 全ての職業を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [
            Self::Fighter,
            Self::Mage,
            Self::Cleric,
            Self::Thief,
            Self::Wizard,
            Self::Samurai,
            Self::Lord,
            Self::Ninja,
        ]
        .into_iter()
    }
}

/// 職業マスク。
pub type Classes = FlagSet<Class>;

/// 職業マスクをフォーマットし、正式名称を join した文字列にする。
#[derive(Debug)]
pub struct ClassesDisplay<'sep> {
    classes: Classes,
    sep: &'sep str,
}

impl<'sep> ClassesDisplay<'sep> {
    pub fn new(classes: Classes, sep: &'sep str) -> Self {
        Self { classes, sep }
    }
}

impl std::fmt::Display for ClassesDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for class in Class::iter() {
            if self.classes.contains(class) {
                if !first {
                    f.write_str(self.sep)?;
                }
                f.write_str(class.name())?;
                first = false;
            }
        }

        Ok(())
    }
}

/// 職業マスクをフォーマットし、頭文字を繋げた文字列にする。
/// 含まれない職業の部分は指定した文字でパディングする。
#[derive(Debug)]
pub struct ClassesDisplayInitialPad {
    classes: Classes,
    pad: char,
}

impl ClassesDisplayInitialPad {
    pub fn new(classes: Classes, pad: char) -> Self {
        Self { classes, pad }
    }
}

impl std::fmt::Display for ClassesDisplayInitialPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for class in Class::iter() {
            if self.classes.contains(class) {
                f.write_str(class.name_initial())?;
            } else {
                f.write_char(self.pad)?;
            }
        }

        Ok(())
    }
}

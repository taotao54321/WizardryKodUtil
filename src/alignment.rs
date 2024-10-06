use flagset::{flags, FlagSet};

flags! {
    /// 性格。
    #[repr(u8)]
    pub enum Alignment: u8 {
        Good = 1 << 0,
        Neutral = 1 << 1,
        Evil = 1 << 2,
    }
}

impl Alignment {
    /// 性格ID (`0..=2`) から性格を作る。
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::Good),
            1 => Some(Self::Neutral),
            2 => Some(Self::Evil),
            _ => None,
        }
    }

    /// 性格ID (`0..=2`) を返す。
    pub fn to_id(self) -> u8 {
        match self {
            Self::Good => 0,
            Self::Neutral => 1,
            Self::Evil => 2,
        }
    }

    /// 正式名称を返す。
    pub fn name(self) -> &'static str {
        match self {
            Self::Good => "Good",
            Self::Neutral => "Neutral",
            Self::Evil => "Evil",
        }
    }

    /// 名称の頭文字を返す。
    pub fn name_initial(self) -> &'static str {
        match self {
            Self::Good => "G",
            Self::Neutral => "N",
            Self::Evil => "E",
        }
    }

    /// 日本語での正式名称を返す。
    pub fn name_ja(self) -> &'static str {
        match self {
            Self::Good => "善",
            Self::Neutral => "中立",
            Self::Evil => "悪",
        }
    }

    /// 全ての性格を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [Self::Good, Self::Neutral, Self::Evil].into_iter()
    }
}

/// 性格マスク。
pub type Alignments = FlagSet<Alignment>;

/// 性格マスクをフォーマットし、正式名称を join した文字列にする。
#[derive(Debug)]
pub struct AlignementsDisplay<'sep> {
    alignments: Alignments,
    sep: &'sep str,
}

impl<'sep> AlignementsDisplay<'sep> {
    pub fn new(alignments: Alignments, sep: &'sep str) -> Self {
        Self { alignments, sep }
    }
}

impl std::fmt::Display for AlignementsDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for alignment in Alignment::iter() {
            if self.alignments.contains(alignment) {
                if !first {
                    f.write_str(self.sep)?;
                }
                f.write_str(alignment.name())?;
                first = false;
            }
        }

        Ok(())
    }
}

/// 性格マスクをフォーマットし、頭文字を join した文字列にする。
#[derive(Debug)]
pub struct AlignementsDisplayInitial<'sep> {
    alignments: Alignments,
    sep: &'sep str,
}

impl<'sep> AlignementsDisplayInitial<'sep> {
    pub fn new(alignments: Alignments, sep: &'sep str) -> Self {
        Self { alignments, sep }
    }
}

impl std::fmt::Display for AlignementsDisplayInitial<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for alignment in Alignment::iter() {
            if self.alignments.contains(alignment) {
                if !first {
                    f.write_str(self.sep)?;
                }
                f.write_str(alignment.name_initial())?;
                first = false;
            }
        }

        Ok(())
    }
}

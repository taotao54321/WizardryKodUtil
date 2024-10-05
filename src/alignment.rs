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

    /// 全ての性格を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [Self::Good, Self::Neutral, Self::Evil].into_iter()
    }
}

/// 性格マスク。
pub type Alignments = FlagSet<Alignment>;

/// 性格マスクをフォーマットし、正式名称のカンマ区切り文字列にする。
#[derive(Debug)]
pub struct AlignementsDisplay(Alignments);

impl AlignementsDisplay {
    pub fn new(alignments: Alignments) -> Self {
        Self(alignments)
    }
}

impl std::fmt::Display for AlignementsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for alignment in Alignment::iter() {
            if self.0.contains(alignment) {
                if !first {
                    f.write_str(",")?;
                }
                f.write_str(alignment.name())?;
                first = false;
            }
        }

        Ok(())
    }
}

/// 性格マスクをフォーマットし、頭文字を繋げた文字列にする。
#[derive(Debug)]
pub struct AlignementsDisplayInitial(Alignments);

impl AlignementsDisplayInitial {
    pub fn new(alignments: Alignments) -> Self {
        Self(alignments)
    }
}

impl std::fmt::Display for AlignementsDisplayInitial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for alignment in Alignment::iter() {
            if self.0.contains(alignment) {
                f.write_str(alignment.name_initial())?;
            }
        }

        Ok(())
    }
}

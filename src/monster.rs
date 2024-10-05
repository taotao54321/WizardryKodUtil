use flagset::{flags, FlagSet};

use crate::dice::define_dice_expr;
use crate::element::Elements;
use crate::string::GameString;

/// モンスター。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monster {
    pub name_known_singular: GameString,
    pub name_known_plural: GameString,
    pub name_unknown_singular: GameString,
    pub name_unknown_plural: GameString,

    pub kinds: MonsterKinds,
    pub spawn_dice_expr: MonsterSpawnDiceExpr,
    pub hp_dice_expr: MonsterHpDiceExpr,
    pub ac: i8,
    pub drain_xl: u8,
    pub healing: i8,
    pub drop_table_id_wandering: u8,
    pub drop_table_id_guardian: u8,
    pub follower_monster_id: u8,
    pub follower_probability: u8,
    pub mage_spell_lv: u8,
    pub cleric_spell_lv: u8,
    pub breath_elements: Elements,
    pub spell_resistance: u8,
    pub element_resistance: Elements,
    pub abilitys: MonsterAbilitys,
    pub xp: u64,
    pub melee_dice_exprs: Vec<MonsterMeleeDiceExpr>,
}

flags! {
    /// モンスター種別。
    #[repr(u16)]
    pub enum MonsterKind: u16 {
        /// 戦士。
        Fighter = 1 << 0,
        /// 魔術師。
        Mage = 1 << 1,
        /// 僧侶。
        Cleric = 1 << 2,
        /// 盗賊。
        Thief = 1 << 3,
        /// 小人。
        Tiny = 1 << 4,
        /// 巨人。
        Giant = 1 << 5,
        /// 神話。
        Myth = 1 << 6,
        /// 竜。
        Dragon = 1 << 7,
        /// 動物。
        Animal = 1 << 8,
        /// (未使用)
        Unused9 = 1 << 9,
        /// 不死。
        Undead = 1 << 10,
        /// 悪魔。
        Demon = 1 << 11,
        /// 昆虫。
        Insect = 1 << 12,
        /// 魔法生物。
        Enchanted = 1 << 13,
        /// 獣人。
        Lycanthrope = 1 << 14,
        /// (未使用)
        Unused15 = 1 << 15,
    }
}

impl MonsterKind {
    /// 正式名称を返す。
    pub fn name(self) -> &'static str {
        match self {
            Self::Fighter => "戦士",
            Self::Mage => "魔術師",
            Self::Cleric => "僧侶",
            Self::Thief => "盗賊",
            Self::Tiny => "小人",
            Self::Giant => "巨人",
            Self::Myth => "神話",
            Self::Dragon => "竜",
            Self::Animal => "動物",
            Self::Unused9 => "(未使用9)",
            Self::Undead => "不死",
            Self::Demon => "悪魔",
            Self::Insect => "昆虫",
            Self::Enchanted => "魔法生物",
            Self::Lycanthrope => "獣人",
            Self::Unused15 => "(未使用15)",
        }
    }

    /// 略称を返す。
    pub fn name_abbrev(self) -> &'static str {
        match self {
            Self::Fighter => "戦",
            Self::Mage => "魔",
            Self::Cleric => "僧",
            Self::Thief => "盗",
            Self::Tiny => "小",
            Self::Giant => "巨",
            Self::Myth => "神",
            Self::Dragon => "竜",
            Self::Animal => "動",
            Self::Unused9 => "謎",
            Self::Undead => "不",
            Self::Demon => "悪",
            Self::Insect => "昆",
            Self::Enchanted => "傀", // 「傀儡」に由来。「魔」は重複するので。
            Self::Lycanthrope => "獣",
            Self::Unused15 => "謎",
        }
    }

    /// 全てのモンスター種別を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [
            Self::Fighter,
            Self::Mage,
            Self::Cleric,
            Self::Thief,
            Self::Tiny,
            Self::Giant,
            Self::Myth,
            Self::Dragon,
            Self::Animal,
            Self::Unused9,
            Self::Undead,
            Self::Demon,
            Self::Insect,
            Self::Enchanted,
            Self::Lycanthrope,
            Self::Unused15,
        ]
        .into_iter()
    }
}

/// モンスター種別マスク。
pub type MonsterKinds = FlagSet<MonsterKind>;

/// モンスター種別マスクをフォーマットし、正式名称のカンマ区切り文字列にする。
#[derive(Debug)]
pub struct MonsterKindDisplay(MonsterKinds);

impl MonsterKindDisplay {
    pub fn new(kinds: MonsterKinds) -> Self {
        Self(kinds)
    }
}

impl std::fmt::Display for MonsterKindDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for kind in MonsterKind::iter() {
            if self.0.contains(kind) {
                if !first {
                    f.write_str(",")?;
                }
                f.write_str(kind.name())?;
                first = false;
            }
        }

        Ok(())
    }
}

/// モンスター種別マスクをフォーマットし、略称を繋げた文字列にする。
#[derive(Debug)]
pub struct MonsterKindDisplayAbbrev(MonsterKinds);

impl MonsterKindDisplayAbbrev {
    pub fn new(kinds: MonsterKinds) -> Self {
        Self(kinds)
    }
}

impl std::fmt::Display for MonsterKindDisplayAbbrev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for kind in MonsterKind::iter() {
            if self.0.contains(kind) {
                f.write_str(kind.name_abbrev())?;
            }
        }

        Ok(())
    }
}

flags! {
    /// モンスター特殊能力。
    #[repr(u8)]
    pub enum MonsterAbility: u8 {
        /// 石化打撃。
        Petrify = 1 << 0,
        /// 毒打撃。
        Poison = 1 << 1,
        /// 麻痺打撃。
        Paralyze = 1 << 2,
        /// 首切り打撃。
        Critical = 1 << 3,
        /// 睡眠弱点。
        Sleepy = 1 << 4,
        /// 逃走する。
        Flee = 1 << 5,
        /// 仲間を呼ぶ。
        Call = 1 << 6,
        /// (未使用)
        Unused7 = 1 << 7,
    }
}

impl MonsterAbility {
    /// 正式名称を返す。
    pub fn name(self) -> &'static str {
        match self {
            Self::Petrify => "石化打撃",
            Self::Poison => "毒打撃",
            Self::Paralyze => "麻痺打撃",
            Self::Critical => "首切り打撃",
            Self::Sleepy => "睡眠弱点",
            Self::Flee => "逃走する",
            Self::Call => "仲間を呼ぶ",
            Self::Unused7 => "(未使用7)",
        }
    }

    /// 略称を返す。
    pub fn name_abbrev(self) -> &'static str {
        match self {
            Self::Petrify => "石",
            Self::Poison => "毒",
            Self::Paralyze => "麻",
            Self::Critical => "首",
            Self::Sleepy => "眠",
            Self::Flee => "逃",
            Self::Call => "呼",
            Self::Unused7 => "謎",
        }
    }

    /// 全てのモンスター特殊能力を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [
            Self::Petrify,
            Self::Poison,
            Self::Paralyze,
            Self::Critical,
            Self::Sleepy,
            Self::Flee,
            Self::Call,
            Self::Unused7,
        ]
        .into_iter()
    }
}

/// モンスター特殊能力マスク。
pub type MonsterAbilitys = FlagSet<MonsterAbility>;

/// モンスター特殊能力マスクをフォーマットし、正式名称のカンマ区切り文字列にする。
#[derive(Debug)]
pub struct MonsterAbilitysDisplay(MonsterAbilitys);

impl MonsterAbilitysDisplay {
    pub fn new(abilitys: MonsterAbilitys) -> Self {
        Self(abilitys)
    }
}

impl std::fmt::Display for MonsterAbilitysDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for ability in MonsterAbility::iter() {
            if self.0.contains(ability) {
                if !first {
                    f.write_str(",")?;
                }
                f.write_str(ability.name())?;
                first = false;
            }
        }

        Ok(())
    }
}

/// モンスター特殊能力マスクをフォーマットし、略称を繋げた文字列にする。
#[derive(Debug)]
pub struct MonsterAbilitysDisplayAbbrev(MonsterAbilitys);

impl MonsterAbilitysDisplayAbbrev {
    pub fn new(abilitys: MonsterAbilitys) -> Self {
        Self(abilitys)
    }
}

impl std::fmt::Display for MonsterAbilitysDisplayAbbrev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ability in MonsterAbility::iter() {
            if self.0.contains(ability) {
                f.write_str(ability.name_abbrev())?;
            }
        }

        Ok(())
    }
}

// モンスターの 1 グループあたりの出現数ダイス式。
define_dice_expr!(MonsterSpawnDiceExpr);

impl MonsterSpawnDiceExpr {
    pub fn bias_decoded(self) -> i16 {
        decode_spawn_hp_bias(self.bias)
    }
}

impl std::fmt::Display for MonsterSpawnDiceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self.count;
        let face = self.face;
        let bias_decoded = self.bias_decoded();

        if bias_decoded == 0 {
            write!(f, "{count}d{face}")
        } else {
            write!(f, "{count}d{face}{bias_decoded:+}")
        }
    }
}

// モンスターのHPダイス式。
define_dice_expr!(MonsterHpDiceExpr);

impl MonsterHpDiceExpr {
    pub fn bias_decoded(self) -> i16 {
        decode_spawn_hp_bias(self.bias)
    }
}

impl std::fmt::Display for MonsterHpDiceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self.count;
        let face = self.face;
        let bias_decoded = self.bias_decoded();

        if bias_decoded == 0 {
            write!(f, "{count}d{face}")
        } else {
            write!(f, "{count}d{face}{bias_decoded:+}")
        }
    }
}

// モンスターの打撃 1 回のダメージダイス式。
define_dice_expr!(MonsterMeleeDiceExpr);

/// 出現数ダイス式およびHPダイス式の追加値を `i16` 値に変換する。
///
/// `0..=149` が正、`150..=255` が負 (2 の補数) とみなされる。
fn decode_spawn_hp_bias(bias: u8) -> i16 {
    if bias <= 149 {
        i16::from(bias)
    } else {
        i16::from(bias as i8)
    }
}

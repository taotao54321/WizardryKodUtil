use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::alignment::Alignment;
use crate::class::Classes;
use crate::dice::define_dice_expr;
use crate::element::Elements;
use crate::monster::MonsterKinds;
use crate::string::GameString;

/// アイテム。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub name_known: GameString,
    pub name_unknown: GameString,

    pub kind: ItemKind,
    /// 性格限定装備。
    pub alignment: Option<Alignment>,
    pub cursed: bool,
    pub special_power_id: u8,
    pub break_probability: u8,
    pub break_item_id: u8,
    pub price: u64,
    pub use_spell_id: u8,
    pub usable_in_camp: bool,
    pub usable_in_battle: bool,
    pub equip_classes: Classes,
    pub healing: i8,
    /// 撃退対象のモンスター種別マスク。
    pub repel_monster_kinds: MonsterKinds,
    pub element_resistance: Elements,
    /// NOTE: この値が装備者のACから「減算」される。
    pub ac: i8,
    pub melee_accuracy: i8,
    pub melee_dice_expr: ItemMeleeDiceExpr,
    pub extra_melee_count: u8,
    pub critical: bool,
    /// 倍打対象のモンスター種別マスク。
    pub slay_monster_kinds: MonsterKinds,
}

/// アイテム種別。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum ItemKind {
    Weapon = 0,
    Armor = 1,
    Shield = 2,
    Helm = 3,
    Gloves = 4,
    Accessory = 5,
    Tool = 6,
}

impl ItemKind {
    /// 正式名称を返す。
    pub fn name(self) -> &'static str {
        match self {
            Self::Weapon => "武器",
            Self::Armor => "鎧",
            Self::Shield => "盾",
            Self::Helm => "兜",
            Self::Gloves => "小手",
            Self::Accessory => "装飾品",
            Self::Tool => "道具",
        }
    }

    /// 全てのアイテム種別を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [
            Self::Weapon,
            Self::Armor,
            Self::Shield,
            Self::Helm,
            Self::Gloves,
            Self::Accessory,
            Self::Tool,
        ]
        .into_iter()
    }
}

// アイテムの打撃ダメージダイス式。
define_dice_expr!(ItemMeleeDiceExpr);

impl ItemMeleeDiceExpr {
    pub fn bias_decoded(self) -> i8 {
        self.bias as i8
    }
}

impl std::fmt::Display for ItemMeleeDiceExpr {
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

use crate::alignment::Alignment;
use crate::bcd::PackedBcdBe;
use crate::class::Classes;
use crate::element::Elements;
use crate::item::{Item, ItemKind, ItemMeleeDiceExpr};
use crate::monster::MonsterKinds;
use crate::rom::Rom;
use crate::string::GameString;
use crate::util::{SliceExt as _, U8SliceExt as _};

pub const ITEM_COUNT: usize = 138;

/// 全アイテムをIDの昇順で抽出する。
pub fn extract_items(rom: &Rom) -> Vec<Item> {
    (0..ITEM_COUNT).map(|id| extract_item(rom, id)).collect()
}

/// 指定したIDのアイテムを抽出する。
pub fn extract_item(rom: &Rom, id: usize) -> Item {
    const CHUNK_LEN: usize = 31;

    assert!(id < ITEM_COUNT);

    let bank = rom.prg_bank(8);

    let buf = &bank[CHUNK_LEN * id..][..CHUNK_LEN];

    let (name_known, buf) = split_first_name(bank, buf);
    let (name_unknown, buf) = split_first_name(bank, buf);

    let (kind, buf) = buf.split_first_u8().unwrap();
    let kind = ItemKind::try_from(kind).unwrap();
    let (alignment, buf) = buf.split_first_u8().unwrap();
    let alignment = (alignment != 0xFF).then(|| Alignment::from_id(alignment).unwrap());
    let (cursed, buf) = buf.split_first_u8().unwrap();
    let cursed = match cursed {
        0 => false,
        0xFF => true,
        _ => panic!("invalid `cursed` value: 0x{cursed:02X}"),
    };
    let (special_power_id, buf) = buf.split_first_u8().unwrap();
    let (break_probability, buf) = buf.split_first_u8().unwrap();
    let (break_item_id, buf) = buf.split_first_u8().unwrap();
    let (price, buf) = split_first_bcd::<6>(buf);
    let price = price.to_u64();
    let (use_spell_id, usable_in_camp, usable_in_battle, buf) = {
        let (b, buf) = buf.split_first_u8().unwrap();
        (b & 0x3F, (b & (1 << 6)) != 0, (b & (1 << 7)) != 0, buf)
    };
    let (equip_classes, buf) = buf.split_first_u8().unwrap();
    let equip_classes = Classes::new(equip_classes).unwrap();
    let (healing, buf) = buf.split_first_i8().unwrap();
    let (repel_monster_kinds, buf) = buf.split_first_u16le().unwrap();
    let repel_monster_kinds = MonsterKinds::new(repel_monster_kinds).unwrap();
    let (element_resistance, buf) = buf.split_first_u8().unwrap();
    let element_resistance = Elements::new(element_resistance).unwrap();
    let (ac, buf) = buf.split_first_i8().unwrap();
    let (melee_accuracy, buf) = buf.split_first_i8().unwrap();
    let (melee_dice_expr, buf) = split_first_melee_dice_expr(buf);
    let (extra_melee_count, buf) = buf.split_first_u8().unwrap();
    let (critical, buf) = buf.split_first_u8().unwrap();
    let critical = match critical {
        0 => false,
        0xFF => true,
        _ => panic!("invalid `critical` value: 0x{critical:02X}"),
    };
    let (slay_monster_kinds, _) = buf.split_first_u16le().unwrap();
    let slay_monster_kinds = MonsterKinds::new(slay_monster_kinds).unwrap();

    Item {
        name_known,
        name_unknown,

        kind,
        alignment,
        cursed,
        special_power_id,
        break_probability,
        break_item_id,
        price,
        use_spell_id,
        usable_in_camp,
        usable_in_battle,
        equip_classes,
        healing,
        repel_monster_kinds,
        element_resistance,
        ac,
        melee_accuracy,
        melee_dice_expr,
        extra_melee_count,
        critical,
        slay_monster_kinds,
    }
}

fn split_first_name<'a>(bank: &'a [u8], buf: &'a [u8]) -> (GameString, &'a [u8]) {
    let (ptr, buf) = buf.split_first_u16le().unwrap();
    let name = read_name(&bank[usize::from(ptr - 0x8000)..]);

    (name, buf)
}

fn read_name(buf: &[u8]) -> GameString {
    let (name, _) = buf.split_once_(|&b| b == 0).unwrap();

    GameString::from_bytes(name).unwrap()
}

fn split_first_bcd<const LEN: usize>(buf: &[u8]) -> (PackedBcdBe<LEN>, &[u8]) {
    let (&bcd, buf) = buf.split_first_chunk::<LEN>().unwrap();
    let bcd = PackedBcdBe::new(bcd).unwrap();

    (bcd, buf)
}

fn split_first_melee_dice_expr(buf: &[u8]) -> (ItemMeleeDiceExpr, &[u8]) {
    // (面数, 個数, 追加値) の順であることに注意。
    let (face, buf) = buf.split_first_u8().unwrap();
    let (count, buf) = buf.split_first_u8().unwrap();
    let (bias, buf) = buf.split_first_u8().unwrap();

    (ItemMeleeDiceExpr::new(count, face, bias), buf)
}

/// 指定したIDのアイテムの正体名を返す。
pub fn item_true_name(id: usize) -> &'static str {
    todo!();
}

use byteorder::{ByteOrder as _, LE};

use crate::item::{Item, ItemKind};
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

    let name_known = split_first_name(bank, buf);
    let name_unknown = split_first_name(bank, buf);

    let (kind, buf) = buf.split_first_u8().unwrap();
    let kind = ItemKind::try_from(kind).unwrap();

    todo!();
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

/// 指定したIDのアイテムの正体名を返す。
pub fn item_true_name(id: usize) -> &'static str {
    todo!();
}

use std::path::PathBuf;

use clap::Parser;
use derive_builder::Builder;
use itertools::Itertools as _;

use wizardry_kod_util::*;

/// 原作の ROM からモンスターデータを抽出する。
#[derive(Debug, Parser)]
struct Cli {
    /// 原作の iNES ROM ファイル。
    path_ines: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(cli.path_ines)?;

    let monsters = extract::extract_monsters(&rom);

    output_markdown(monsters);

    Ok(())
}

fn output_markdown(monsters: Vec<Monster>) {
    output_markdown_header();

    for (id, monster) in monsters.into_iter().enumerate() {
        output_markdown_row(id, monster);
    }
}

fn output_markdown_header() {
    const COLUMNS: &[(HoriAlign, &str)] = &[
        (HoriAlign::Right, "ID"),
        (HoriAlign::Left, "確定名"),
        (HoriAlign::Left, "不確定名"),
        (HoriAlign::Left, "種別"),
        (HoriAlign::Left, "HP"),
        (HoriAlign::Right, "AC"),
        (HoriAlign::Right, "無効化"),
        (HoriAlign::Left, "打撃"),
        (HoriAlign::Right, "経験値"),
        (HoriAlign::Left, "ドロップ"),
        (HoriAlign::Left, "出現数"),
        (HoriAlign::Left, "後続"),
        (HoriAlign::Left, "備考"),
    ];

    println!("| {} |", COLUMNS.iter().map(|col| col.1).join(" | "));
    println!(
        "| {} |",
        COLUMNS.iter().map(|col| col.0.as_markdown()).join(" | ")
    );
}

fn output_markdown_row(id: usize, monster: Monster) {
    let Monster {
        name_known_singular: _,
        name_known_plural,
        name_unknown_singular,
        name_unknown_plural,
        kinds,
        spawn_dice_expr,
        hp_dice_expr,
        ac,
        drain_xl,
        healing,
        drop_table_id_wandering,
        drop_table_id_guardian,
        follower_monster_id,
        follower_probability,
        mage_spell_lv,
        cleric_spell_lv,
        breath_elements,
        spell_resistance,
        element_resistance,
        abilitys,
        xp,
        melee_dice_exprs,
    } = monster;

    let true_name = extract::monster_true_name(id);

    let mut row = MarkdownRowBuilder::default();

    row.id(format!("{id}"));

    row.name_known(format!("{true_name}<br>{name_known_plural}"));
    row.name_unknown(format!("{name_unknown_singular}<br>{name_unknown_plural}"));

    row.kinds(format!("{}", MonsterKindDisplay::new(kinds)));
    row.hp(format!("{hp_dice_expr}"));
    row.ac(format!("{ac}"));
    row.spell_resistance(format!("{spell_resistance}/256"));
    row.melee(melee_dice_exprs.iter().join("<br>"));
    row.xp(format!("{xp}"));
    row.drop_(format!(
        "徘徊: {drop_table_id_wandering}<br>玄室: {drop_table_id_guardian}"
    ));
    row.spawn(format!("{spawn_dice_expr}"));
    row.follower(format!(
        "{} ({follower_probability} %)",
        extract::monster_true_name(usize::from(follower_monster_id))
    ));

    {
        let mut notes = Vec::<String>::new();
        notes.extend(note_spells(mage_spell_lv, cleric_spell_lv));
        notes.extend(note_breath(breath_elements));
        notes.extend(note_special(drain_xl, healing, abilitys));
        notes.extend(note_element_resistance(element_resistance));
        notes.extend(note_vulnerability(abilitys));
        row.notes(notes.into_iter().join("<br>"));
    }

    row.build().unwrap().print();
}

fn note_spells(mage: u8, cleric: u8) -> Option<String> {
    let mut ss = Vec::<String>::new();

    if mage > 0 {
        ss.push(format!("魔{mage}"));
    }
    if cleric > 0 {
        ss.push(format!("僧{cleric}"));
    }

    (!ss.is_empty()).then(|| format!("呪文: {}", ss.into_iter().join(" ")))
}

fn note_breath(elements: Elements) -> Option<String> {
    (!elements.is_empty()).then(|| format!("息: {}", ElementsDisplayAbbrev::new(elements)))
}

fn note_special(drain_xl: u8, healing: i8, abilitys: MonsterAbilitys) -> Option<String> {
    let mut ss = Vec::<String>::new();

    // 打撃の追加効果は原作での処理順に並べている。
    if abilitys.contains(MonsterAbility::Poison) {
        ss.push("毒".to_owned());
    }
    if abilitys.contains(MonsterAbility::Paralyze) {
        ss.push("麻".to_owned());
    }
    if abilitys.contains(MonsterAbility::Petrify) {
        ss.push("石".to_owned());
    }
    if drain_xl > 0 {
        ss.push(format!("吸{drain_xl}"));
    }
    if abilitys.contains(MonsterAbility::Critical) {
        ss.push("首".to_owned());
    }

    if abilitys.contains(MonsterAbility::Call) {
        ss.push("呼".to_owned());
    }
    if abilitys.contains(MonsterAbility::Flee) {
        ss.push("逃".to_owned());
    }

    if healing != 0 {
        ss.push(format!("回{healing}"));
    }

    (!ss.is_empty()).then(|| format!("特殊: {}", ss.into_iter().join(" ")))
}

fn note_element_resistance(elements: Elements) -> Option<String> {
    (!elements.is_empty()).then(|| format!("抵抗: {}", ElementsDisplayAbbrev::new(elements)))
}

fn note_vulnerability(abilitys: MonsterAbilitys) -> Option<String> {
    abilitys
        .contains(MonsterAbility::Sleepy)
        .then(|| "弱点: 眠".to_owned())
}

#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
struct MarkdownRow {
    id: String,

    name_known: String,
    name_unknown: String,

    kinds: String,
    hp: String,
    ac: String,
    spell_resistance: String,
    melee: String,
    xp: String,
    drop_: String,
    spawn: String,
    follower: String,

    notes: String,
}

impl MarkdownRow {
    fn print(&self) {
        let Self {
            id,
            name_known,
            name_unknown,
            kinds,
            hp,
            ac,
            spell_resistance,
            melee,
            xp,
            drop_,
            spawn,
            follower,
            notes,
        } = self;

        let fields = [
            id,
            name_known,
            name_unknown,
            kinds,
            hp,
            ac,
            spell_resistance,
            melee,
            xp,
            drop_,
            spawn,
            follower,
            notes,
        ];

        println!("| {} |", fields.iter().join(" | "));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HoriAlign {
    Left,
    #[allow(dead_code)]
    Center,
    Right,
}

impl HoriAlign {
    fn as_markdown(self) -> &'static str {
        match self {
            Self::Left => "--",
            Self::Center => ":-:",
            Self::Right => "--:",
        }
    }
}

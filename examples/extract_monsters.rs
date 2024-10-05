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
        (HoriAlign::Right, "経験値"),
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
    row.xp(format!("{xp}"));

    row.build().unwrap().print();
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
    xp: String,
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
            xp,
        } = self;

        let fields = [id, name_known, name_unknown, kinds, hp, ac, xp];

        println!("| {} |", fields.iter().join(" | "));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HoriAlign {
    Left,
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

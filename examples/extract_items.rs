use std::path::PathBuf;

use clap::Parser;
use derive_builder::Builder;
use itertools::Itertools as _;

use wizardry_kod_util::*;

/// 原作の ROM からアイテムデータを抽出する。
#[derive(Debug, Parser)]
struct Cli {
    /// 原作の ROM ファイル。
    path_ines: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(cli.path_ines)?;

    let items = extract::extract_items(&rom);

    output_markdown(items);

    Ok(())
}

fn output_markdown(items: Vec<Item>) {
    output_markdown_header();

    for (id, item) in items.into_iter().enumerate() {
        output_markdown_row(id, item);
    }
}

fn output_markdown_header() {
    const COLUMNS: &[(HoriAlign, &str)] = &[
        (HoriAlign::Right, "ID"),
        (HoriAlign::Left, "確定名"),
        (HoriAlign::Left, "不確定名"),
        (HoriAlign::Left, "種別"),
        (HoriAlign::Center, "装備条件"),
        (HoriAlign::Right, "AC"),
        (HoriAlign::Left, "ダメージ"),
        (HoriAlign::Right, "買値"),
        (HoriAlign::Left, "備考"),
    ];

    println!("| {} |", COLUMNS.iter().map(|col| col.1).join(" | "));
    println!(
        "| {} |",
        COLUMNS.iter().map(|col| col.0.as_markdown()).join(" | ")
    );
}

fn output_markdown_row(id: usize, item: Item) {
    let Item {
        name_known: _,
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
    } = item;

    let true_name = extract::item_true_name(id);

    let mut row = MarkdownRowBuilder::default();

    row.id(format!("{id}"));

    row.name_known(true_name);
    row.name_unknown(format!("{name_unknown}"));

    row.kind(kind.name());
    row.equip_classes(format!(
        "{}",
        ClassesDisplayInitialPad::new(equip_classes, '-')
    ));
    // TODO: 呪いありの場合のAC表示
    row.ac(fmt_ac(ac));
    row.melee_dice_expr(format!("{melee_dice_expr}"));
    row.price(format!("{price}"));

    {
        let mut notes = Vec::<String>::new();
        notes.extend(note_cursed(cursed));
        notes.extend(note_alignment(alignment));
        notes.extend(note_melee_accuracy(melee_accuracy));
        // TODO
        row.notes(notes.into_iter().join("<br>"));
    }

    row.build().unwrap().print();
}

fn note_cursed(cursed: bool) -> Option<String> {
    cursed.then(|| "呪い".to_owned())
}

fn note_alignment(alignment: Option<Alignment>) -> Option<String> {
    alignment.map(|alignment| format!("性格限定装備: {}", alignment.name_ja()))
}

fn note_melee_accuracy(melee_accuracy: i8) -> Option<String> {
    (melee_accuracy != 0).then(|| format!("命中{melee_accuracy:+}"))
}

fn fmt_ac(ac: i8) -> String {
    if ac == 0 {
        "0".to_owned()
    } else {
        format!("{:+}", -ac)
    }
}

#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
struct MarkdownRow {
    id: String,

    name_known: String,
    name_unknown: String,

    kind: String,
    equip_classes: String,
    ac: String,
    melee_dice_expr: String,
    price: String,

    notes: String,
}

impl MarkdownRow {
    fn print(&self) {
        let Self {
            id,
            name_known,
            name_unknown,
            kind,
            equip_classes,
            ac,
            melee_dice_expr,
            price,
            notes,
        } = self;

        let fields = [
            id,
            name_known,
            name_unknown,
            kind,
            equip_classes,
            ac,
            melee_dice_expr,
            price,
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

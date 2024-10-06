/// 指定したID (`1..=27`) のスペシャルパワーの説明を返す。
pub fn special_power_description(id: usize) -> &'static str {
    const TABLE: [&str; 27] = [
        "力 +1",
        "知恵 +1",
        "信仰心 +1",
        "生命力 +1",
        "素早さ +1",
        "運 +1",
        "力 -1",
        "知恵 -1",
        "信仰心 -1",
        "生命力 -1",
        "素早さ -1",
        "運 -1",
        "年齢 -1",
        "(未使用14)",
        "(未使用15)",
        "ロードに転職",
        "忍者に転職",
        "(未使用18)",
        "経験値 +50000",
        "灰化",
        "(未使用21)",
        "最大HP +1",
        "生存者全員のHP全快",
        "侍/君/忍のいずれかに転職",
        "死亡し、戦/魔/僧/盗/司のいずれかに転職",
        "全ての現在MPが 9 になる",
        "全呪文を忘れ、全ての現在MPが 9 になる",
    ];

    assert!(matches!(id, 1..=27));

    TABLE[id - 1]
}

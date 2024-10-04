use wizardry_kod_util::*;

fn main() {
    solve();
}

fn solve() {
    let mut counts = [0; 7];

    for state in 0..=u16::MAX {
        let rng = GameRng::new(state);
        let extra = extra_bonus_count(rng);
        counts[extra] += 1;
    }

    for (extra, &count) in counts.iter().enumerate() {
        let ratio = f64::from(count) / f64::from(0x10000);
        println!("+{}\t{count}/65536\t{ratio:.05}", 10 * extra);
    }
}

fn extra_bonus_count(mut rng: GameRng) -> usize {
    let mut count = 0;

    while rng.gen_range(20) == 15 {
        count += 1;
        if count >= 6 {
            break;
        }
    }

    count
}

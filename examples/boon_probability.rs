use wizardry_kod_util::*;

fn main() {
    solve("Haman", haman);
    solve("Mahaman", mahaman);
}

fn solve(name: &str, f: fn(Rng) -> Boons) {
    let mut counts = [0; 7];

    for state in 0..=u16::MAX {
        let rng = Rng::from_state(state);
        let boons = f(rng);
        for boon in boons {
            counts[usize::from(boon)] += 1;
        }
    }

    println!("[{name}]");
    println!();

    for (boon, &count) in counts.iter().enumerate() {
        let ratio = f64::from(count) / f64::from(0x10000);
        println!("{boon}\t{count}/65536\t{ratio:.04}");
    }

    println!();
}

type Boons = [u8; 3];

fn haman(mut rng: Rng) -> Boons {
    let boons: Boons = std::array::from_fn(|_| rng.gen() & 3);

    make_boons_distinct(boons)
}

fn mahaman(mut rng: Rng) -> Boons {
    let boons: Boons = std::array::from_fn(|_| rng.gen_range(7));

    make_boons_distinct(boons)
}

fn make_boons_distinct(boons: Boons) -> Boons {
    let mut res = boons;

    if res[1] == res[0] {
        res[1] = (res[1] + 1) % 7;
    }

    while res[2] == res[0] || res[2] == res[1] {
        res[2] = (res[2] + 1) % 7;
    }

    assert!(res.into_iter().all(|x| x < 7));
    assert_ne!(res[0], res[1]);
    assert_ne!(res[0], res[2]);
    assert_ne!(res[1], res[2]);

    res
}

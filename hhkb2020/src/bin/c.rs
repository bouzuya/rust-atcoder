use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    };
    let mut set = std::collections::BTreeSet::new();
    let mut c = 0;
    for &p_i in p.iter() {
        set.insert(p_i);
        loop {
            if !set.contains(&c) {
                println!("{}", c);
                break;
            }
            c += 1;
        }
    }
}

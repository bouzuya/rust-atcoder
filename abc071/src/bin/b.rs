use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
    };
    let set = s
        .iter()
        .map(|&c| c)
        .collect::<std::collections::BTreeSet<u8>>();
    for c in (0..26_u8).map(|i| 'a' as u8 + i) {
        if !set.contains(&c) {
            println!("{}", c as char);
            return;
        }
    }
    println!("{}", "None");
}

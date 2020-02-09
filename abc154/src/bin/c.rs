use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        av: [usize; n]
    };
    let mut set = HashSet::new();
    for a in av {
        if set.contains(&a) {
            println!("NO");
            return;
        }
        set.insert(a);
    }
    println!("YES");
}

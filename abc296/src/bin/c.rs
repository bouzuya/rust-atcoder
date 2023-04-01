use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    };
    let set = a.iter().copied().collect::<HashSet<_>>();
    for a_i in a {
        if set.contains(&(a_i + x)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

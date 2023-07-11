use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let range = if n % 2 == 0 {
        (1..n).step_by(2)
    } else {
        (2..n).step_by(2)
    };
    let mut ans = 1_usize;
    for i in range {
        let count = *map.get(&i).unwrap_or(&0);
        if count != 2 {
            println!("0");
            return;
        }
        ans *= 2;
        ans %= 1_000_000_007;
    }
    if n % 2 != 0 && *map.get(&0).unwrap_or(&0) != 1 {
        println!("0");
        return;
    }
    println!("{}", ans);
}

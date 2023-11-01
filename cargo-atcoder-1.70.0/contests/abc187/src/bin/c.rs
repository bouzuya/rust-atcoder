use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for s_i in s {
        if !s_i.starts_with('!') {
            if set2.contains(&s_i) {
                println!("{}", s_i);
                return;
            }
            set1.insert(s_i);
        } else {
            if set1.contains(s_i.trim_start_matches('!')) {
                println!("{}", s_i.trim_start_matches('!'));
                return;
            }
            set2.insert(s_i.trim_start_matches('!').to_owned());
        }
    }
    println!("satisfiable");
}

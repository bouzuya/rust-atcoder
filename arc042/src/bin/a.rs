use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; m],
    };
    let mut set = std::collections::HashSet::new();
    for &a_i in a.iter().rev() {
        if set.insert(a_i) {
            println!("{}", a_i);
        }
    }
    for i in 1..=n as i64 {
        if set.insert(i) {
            println!("{}", i);
        }
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    };
    let mut ok = true;
    let mut set = std::collections::BTreeSet::new();
    set.insert(&w[0]);
    let mut p = &w[0];
    for w_i in w.iter().skip(1) {
        if !set.insert(w_i) || p.chars().last() != w_i.chars().nth(0) {
            ok = false;
            break;
        }
        p = w_i;
    }
    let ans = ok;
    println!("{}", if ans { "Yes" } else { "No" });
}

use proconio::input;

fn main() {
    input! {
        n_a: usize,
        n_b: usize,
        a: [String; n_a],
        b: [String; n_b],
    };
    let set_a = a
        .into_iter()
        .collect::<std::collections::BTreeSet<String>>();
    let mut count_a_and_b = 0;
    for b_i in b {
        if set_a.contains(&b_i) {
            count_a_and_b += 1;
        }
    }
    let ans = count_a_and_b as f64 / (n_a + n_b - count_a_and_b) as f64;
    println!("{:.10}", ans);
}

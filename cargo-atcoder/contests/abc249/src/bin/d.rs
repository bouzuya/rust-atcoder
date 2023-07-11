use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let max_a = *a.iter().max().unwrap();
    let mut c = vec![0_usize; max_a + 1];
    for a_i in a {
        c[a_i] += 1;
    }

    let mut ans = 0_usize;
    for a_i in 1..=max_a {
        for a_j in 1..=max_a / a_i {
            let a_k = a_i * a_j;
            ans += c[a_i] * c[a_j] * c[a_k];
        }
    }

    println!("{}", ans);
}

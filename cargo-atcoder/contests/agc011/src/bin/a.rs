use proconio::input;

fn main() {
    input! {
        n: usize,
        capital_c: usize,
        k: usize,
        mut t: [usize; n],
    };
    t.sort();

    let mut index = 0_usize;
    let mut count = 0_usize;
    while index < n {
        let mut c = 0_usize;
        let mut j = index;
        while (j < n) && (t[j] <= t[index] + k) && (c < capital_c) {
            c += 1;
            j += 1;
        }
        count += 1;
        index = j;
    }
    let ans = count;
    println!("{}", ans);
}

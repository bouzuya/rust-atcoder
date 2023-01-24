use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = vec![0_usize; 3];
    for a_i in a {
        count[if a_i % 2 != 0 {
            0
        } else if (a_i / 2) % 2 != 0 {
            1
        } else {
            2
        }] += 1;
    }
    let ans = (count[1] % 2 + count[0]) <= count[2] + 1;
    println!("{}", if ans { "Yes" } else { "No" });
}

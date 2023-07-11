use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    a.reverse();
    let mut alice = 0_usize;
    let mut bob = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        if i % 2 == 0 {
            alice += a_i;
        } else {
            bob += a_i;
        }
    }
    let ans = alice - bob;
    println!("{}", ans);
}

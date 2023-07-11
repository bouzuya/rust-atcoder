use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let modp = 1_000_000_007;
    let mut count = vec![0, 0, 0];
    let mut ans = 1_usize;
    for a_i in a {
        let c = count.iter().filter(|&&x| x == a_i).count();
        ans *= c;
        ans %= modp;
        for x in count.iter_mut() {
            if *x == a_i {
                *x += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}

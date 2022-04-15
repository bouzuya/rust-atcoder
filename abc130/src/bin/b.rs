use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        l: [usize; n],
    };
    let mut count = 1_usize;
    let mut d = 0_usize;
    for l_i in l {
        if d + l_i > x {
            break;
        }
        count += 1;
        d += l_i;
    }
    let ans = count;
    println!("{}", ans);
}

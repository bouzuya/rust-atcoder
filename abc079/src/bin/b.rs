use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut x = (2_usize, 1_usize);
    for _ in 2..=n {
        let (l_i_2, l_i_1) = x;
        x = (l_i_1, l_i_1 + l_i_2);
    }
    let ans = x.1;
    println!("{}", ans);
}

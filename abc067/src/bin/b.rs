use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut l: [i64; n],
    };
    l.sort_by_key(|&l_i| std::cmp::Reverse(l_i));
    let ans = l[0..k].iter().sum::<i64>();
    println!("{}", ans);
}

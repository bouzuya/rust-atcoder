use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let b = a.iter().map(|&a_i| a_i % 200).collect::<Vec<usize>>();
    let mut map = vec![0_usize; 200];
    for &b_i in b.iter() {
        map[b_i] += 1;
    }
    let ans = map
        .iter()
        .map(|&c| if c > 0 { c * (c - 1) / 2 } else { 0 })
        .sum::<usize>();
    println!("{}", ans);
}

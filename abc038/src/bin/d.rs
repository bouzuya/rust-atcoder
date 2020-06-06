use proconio::input;

fn lis_length(a: &Vec<i64>, inf: i64) -> usize {
    let mut len = 0;
    let mut l = vec![inf; a.len()];
    for &a_i in a.iter() {
        let i = l.binary_search(&a_i).map_or_else(|i| i, |i| i);
        l[i] = a_i;
        len = std::cmp::max(len, i + 1);
    }
    len
}

fn main() {
    input! {
        n: usize,
        mut wh: [(i64, i64); n],
    };
    wh.sort_by_key(|&(w, h)| (w, -h));
    let a = wh.iter().map(|&(_, h)| h).collect::<Vec<_>>();
    let ans = lis_length(&a, std::i64::MAX);
    println!("{}", ans);
}

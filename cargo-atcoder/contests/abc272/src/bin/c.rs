use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut e = a
        .iter()
        .copied()
        .filter(|a_i| a_i % 2 == 0)
        .collect::<Vec<i64>>();
    let mut o = a
        .iter()
        .copied()
        .filter(|a_i| a_i % 2 != 0)
        .collect::<Vec<i64>>();
    if e.len() <= 1 && o.len() <= 1 {
        println!("-1");
        return;
    }
    e.sort();
    o.sort();

    let esum = if e.len() >= 2 {
        e[e.len() - 1] + e[e.len() - 2]
    } else {
        -1
    };
    let osum = if o.len() >= 2 {
        o[o.len() - 1] + o[o.len() - 2]
    } else {
        -1
    };

    let ans = esum.max(osum);
    println!("{}", ans);
}

use proconio::input;

fn f(x: usize) -> usize {
    let mut g2 = x.to_string().chars().collect::<Vec<char>>();
    g2.sort();
    let mut g1 = g2.clone();
    g1.reverse();
    let g1 = g1.iter().collect::<String>().parse::<usize>().unwrap();
    let g2 = g2
        .iter()
        .collect::<String>()
        .trim_start_matches('0')
        .parse::<usize>()
        .unwrap_or(0);
    g1 - g2
}

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut a_p = n;
    for _ in 0..k {
        a_p = f(a_p);
    }
    let ans = a_p;
    println!("{}", ans);
}

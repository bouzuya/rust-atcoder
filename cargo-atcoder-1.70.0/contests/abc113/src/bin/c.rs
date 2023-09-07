use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        py: [(usize, usize); m],
    };
    let mut ipy = py
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (p, y))| (i, p, y))
        .collect::<Vec<_>>();
    ipy.sort_by_key(|&(_, p, y)| (p, y));
    let mut prev = 0;
    let mut count = 1;
    let mut ans = vec![];
    for (i, p, _) in ipy {
        if prev != p {
            count = 1;
        }
        ans.push((i, format!("{:06}{:06}", p, count)));
        prev = p;
        count += 1;
    }
    ans.sort_by_key(|&(i, _)| i);
    for (_, a) in ans {
        println!("{}", a);
    }
}

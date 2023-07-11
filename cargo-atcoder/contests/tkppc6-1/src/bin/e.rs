use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut js = vec![true; n];
    for i in 0..32 {
        let mut next = js.clone();
        for (j, a_i) in a.iter().copied().enumerate() {
            if !js[j] {
                continue;
            }
            if ((a_i >> (32 - 1 - i)) & 1) != 1 {
                next[j] = false;
            }
        }
        if next.iter().filter(|&&b| b).count() >= 2 {
            js = next;
        }
    }
    let is = js
        .iter()
        .enumerate()
        .filter_map(|(u, &b)| if b { Some(u) } else { None })
        .collect::<Vec<usize>>();
    let ans = (a[is[0]] & a[is[1]]) << 1;
    println!("{}", ans);
}

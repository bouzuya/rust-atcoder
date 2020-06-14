use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let a_i_max = 1_000_000;
    let mut map = std::collections::BTreeMap::new();
    for &a_i in a.iter() {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let mut b = vec![true; a_i_max + 1];
    let mut x = vec![];
    for (&a_i, &count) in map.iter() {
        if b[a_i] {
            x.push((a_i, count));
            for j in (a_i + a_i..=a_i_max).step_by(a_i) {
                b[j] = false;
            }
        }
    }

    let ans = x.iter().filter(|&&(_, count)| count == 1).count();
    println!("{}", ans);
}

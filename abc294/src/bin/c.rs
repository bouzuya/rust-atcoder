use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let ai = a
        .into_iter()
        .enumerate()
        .map(|(i, a_i)| (a_i, i))
        .collect::<Vec<(usize, usize)>>();
    let bi = b
        .into_iter()
        .enumerate()
        .map(|(i, b_i)| (b_i, i + n))
        .collect::<Vec<(usize, usize)>>();
    let mut ci = ai
        .into_iter()
        .chain(bi.into_iter())
        .collect::<Vec<(usize, usize)>>();
    ci.sort_by_key(|&(v, _)| v);

    println!(
        "{}",
        ci.iter()
            .enumerate()
            .filter(|&(_, &(_, i))| i < n)
            .map(|(i, _)| (i + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!(
        "{}",
        ci.iter()
            .enumerate()
            .filter(|&(_, &(_, i))| i >= n)
            .map(|(i, _)| (i + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n],
    };

    let mut tbl = vec![0; 26];
    for (i, x_i) in x.iter().copied().enumerate() {
        tbl[(x_i as u8 - b'a') as usize] = i;
    }

    let mut t = s
        .iter()
        .enumerate()
        .map(|(i, s_i)| {
            (
                i,
                s_i.iter()
                    .map(|&s_ij| tbl[(s_ij as u8 - b'a') as usize])
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    t.sort_by_key(|(_, t_i)| t_i.clone());
    for (i, _) in t {
        println!("{}", s[i].iter().collect::<String>());
    }
}

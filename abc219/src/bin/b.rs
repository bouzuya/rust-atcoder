use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        s: [Chars; 3],
        t: Bytes,
    };
    let ans = t
        .iter()
        .flat_map(|&t_i| {
            let index = (t_i - b'0' - 1) as usize;
            s[index].clone()
        })
        .collect::<String>();
    println!("{}", ans);
}

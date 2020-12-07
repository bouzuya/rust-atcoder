use proconio::input;

fn main() {
    input! {
        abc: [i64; 3]
    };
    let mut abc = abc.into_iter().enumerate().collect::<Vec<(usize, i64)>>();
    abc.sort_by_key(|&(_, v)| v);
    let ans = (abc[1].0 as u8 + 'A' as u8) as char;
    println!("{}", ans);
}

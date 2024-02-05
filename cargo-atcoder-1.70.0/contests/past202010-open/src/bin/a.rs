use proconio::input;

fn main() {
    input! {
        abc: [usize; 3],
    };
    let mut abc = abc.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
    abc.sort_by_key(|(_, v)| *v);
    let ans = (abc[1].0 as u8 + b'A') as char;
    println!("{}", ans);
}

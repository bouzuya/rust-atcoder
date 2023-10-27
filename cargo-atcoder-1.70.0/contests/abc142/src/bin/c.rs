use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut ai = a.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
    ai.sort_by_key(|&(_, a)| a);
    for (i, _) in ai {
        println!("{}", i + 1);
    }
}

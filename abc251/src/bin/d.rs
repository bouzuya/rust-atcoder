use proconio::input;

fn main() {
    input! {
        _w: usize,
    };

    let mut ans = vec![];
    for i in 0..3 {
        let b = 100_usize.pow(i);
        for j in 1..=99 {
            ans.push(b * j);
        }
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}

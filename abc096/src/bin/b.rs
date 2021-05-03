use proconio::input;

fn main() {
    input! {
        mut abc: [usize; 3],
        k: usize,
    };
    let max = *abc.iter().max().unwrap();
    abc.remove(abc.iter().position(|x| *x == max).unwrap());
    let mut m = max;
    for _ in 0..k {
        m += m;
    }
    abc.push(m);
    let ans = abc.iter().sum::<usize>();
    println!("{}", ans);
}

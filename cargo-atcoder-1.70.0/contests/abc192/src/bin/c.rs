use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    fn g1(x: usize) -> usize {
        let mut ds = x.to_string().chars().collect::<Vec<char>>();
        ds.sort();
        ds.reverse();
        ds.into_iter().collect::<String>().parse::<usize>().unwrap()
    }

    fn g2(x: usize) -> usize {
        let mut ds = x.to_string().chars().collect::<Vec<char>>();
        ds.sort();
        ds.into_iter().collect::<String>().parse::<usize>().unwrap()
    }

    fn f(x: usize) -> usize {
        g1(x) - g2(x)
    }

    let mut a = n;
    for _ in 0..k {
        a = f(a);
    }

    let ans = a;
    println!("{}", ans);
}

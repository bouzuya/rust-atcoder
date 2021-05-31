use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let f = |x: usize| -> usize {
        x.to_string()
            .chars()
            .fold(0_usize, |acc, c| acc + c.to_digit(10).unwrap() as usize)
    };
    let mut ans = vec![];
    let fxs = (1..=9 * 18).filter(|&fx| fx < n).collect::<Vec<usize>>();
    for fx in fxs {
        let x = n - fx;
        if f(x) == fx {
            ans.push(x);
        }
    }

    ans.sort();
    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}

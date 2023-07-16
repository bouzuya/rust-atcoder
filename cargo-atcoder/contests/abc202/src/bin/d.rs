use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    };

    let binom = |n: usize, k: usize| -> usize {
        let mut res = 1_usize;
        for i in 1..=k {
            res *= n + 1 - i;
            res /= i;
        }
        res
    };

    let n = a + b;

    let mut count_a = a;
    let mut sum = 0_usize;
    let mut ans = String::new();
    for i in 0..n {
        if count_a == 0 {
            ans.push_str(&"b".repeat(n - i));
            break;
        }
        let sum_l = binom(n - 1 - ans.len(), count_a - 1);
        if k <= sum + sum_l {
            ans.push('a');
            count_a -= 1;
        } else {
            sum += sum_l;
            ans.push('b');
        }
    }
    println!("{}", ans);
}

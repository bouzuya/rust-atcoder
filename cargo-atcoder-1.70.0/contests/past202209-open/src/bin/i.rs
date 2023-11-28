use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [(usize, usize, usize); t],
    };

    for (n, a, m) in case {
        let mut sum_head = 0_usize;
        for i in 1..=n.min(m) {
            sum_head += (a * i * m - a * i) % m;
        }
        if n < m {
            println!("{}", sum_head);
            continue;
        }

        let times = n / m;
        let tail = n - (times * m);
        let mut tail_sum = 0_usize;
        for i in n - tail + 1..=n {
            tail_sum += (a * i * m - a * i) % m;
        }

        let ans = sum_head * times + tail_sum;
        println!("{}", ans);
    }
}

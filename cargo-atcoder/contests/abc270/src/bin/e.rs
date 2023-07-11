use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut b = a.clone();
    b.sort();

    let mut sum = 0_usize;
    let mut count_per_cycle = n;
    let mut cycle_count = 0;
    for b_i in b {
        let d = (b_i - cycle_count) * count_per_cycle;
        if sum + d >= k {
            let cycle = (k - sum) / count_per_cycle;
            sum += cycle * count_per_cycle;
            cycle_count += cycle;
            break;
        }
        count_per_cycle -= 1;
        cycle_count = b_i;
        sum += d;
    }

    let k = k - sum;
    let mut count = 0_usize;
    let mut ans = a.clone();
    for a_i in ans.iter_mut() {
        *a_i = a_i.saturating_sub(cycle_count);
        if count < k && *a_i > 0 {
            *a_i -= 1;
            count += 1;
        }
    }

    for a in ans {
        println!("{}", a);
    }
}

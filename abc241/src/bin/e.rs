use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let (prefix_len, cycle_len) = {
        let inf = n + 1;
        let mut used = vec![inf; n];
        let mut sum = 0_usize;
        let mut index = 0_usize;
        loop {
            if used[sum % n] != inf {
                break (used[sum % n], index - used[sum % n]);
            }
            used[sum % n] = index;
            sum += a[sum % n];
            index += 1;
        }
    };
    let len = prefix_len + cycle_len;

    let (_prefix_sum, cycle_sum) = {
        let mut prefix_sum = 0_usize;
        let mut sum = 0_usize;
        for i in 0..len {
            if i == prefix_len {
                prefix_sum = sum;
            }
            sum += a[sum % n];
        }
        let cycle_sum = sum - prefix_sum;
        (prefix_sum, cycle_sum)
    };

    let (sum, len) = if k <= prefix_len {
        (0_usize, k)
    } else {
        (
            ((k - prefix_len) / cycle_len) * cycle_sum,
            prefix_len + (k - prefix_len) % cycle_len,
        )
    };
    let mut ans = 0;
    for _ in 0..len {
        ans += a[ans % n];
    }
    ans += sum;
    println!("{}", ans);
}

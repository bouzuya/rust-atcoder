use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    };
    if n == 1 {
        println!("{}", a[0] * k - k * (k - 1) / 2);
        return;
    }

    assert!(n > 1);

    a.sort();
    a.reverse();

    let mut sum = a[0];
    let mut count = 1_usize;
    for i in 1..n {
        let a_p = a[i - 1];
        let a_i = a[i];

        let d = if a_p == a_i { 0 } else { a_p - 1 - a_i };
        let new_count = count + i * d;
        if new_count > k {
            // TODO
            break;
        } else {
            count = new_count;
            sum += if a_i == a_p {
                0
            } else {
                let k = d;
                let h = a_p - 1;
                (k * h - k * (k - 1) / 2) * (i + 1 - 1)
            };
        }

        let curr = i + 1;
        let new_count = count + curr;
        if new_count > k {
            count = count + (k - count);
            sum += a_i * (k - count);
            break;
        } else {
            count = new_count;
            sum += a_i * curr;
        }
    }

    let ans = sum;
    println!("{}", ans);
}

use proconio::input;

fn main() {
    loop {
        input! {
            n: usize,
            k: usize,
        };
        if n == 0 && k == 0 {
            break;
        }

        input! {
            mut x: [usize; n],
        }
        x.sort();

        let mut sum = 0_usize;
        let mut count = 0_usize;
        while count < k {
            sum += x[count];
            count += 1;
        }

        println!("{}", sum);
    }
}

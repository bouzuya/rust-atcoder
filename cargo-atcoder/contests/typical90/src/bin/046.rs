use proconio::input;

fn f(a: &Vec<usize>) -> Vec<usize> {
    let mut c_a = vec![0; 46];
    for a_i in a.iter() {
        c_a[a_i % 46] += 1;
    }
    c_a
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    };
    let c_a = f(&a);
    let c_b = f(&b);
    let c_c = f(&c);
    let mut sum = 0_usize;
    for a in 0..46 {
        for b in 0..46 {
            for c in 0..46 {
                if (a + b + c) % 46 == 0 {
                    sum += c_a[a] * c_b[b] * c_c[c];
                }
            }
        }
    }
    let ans = sum;
    println!("{}", ans);
}

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
    let c_c = f(&c);
    let mut sum = 0_usize;
    for b_i in b.iter() {
        for a in 0..46 {
            for c in 0..46 {
                if (a + b_i % 46 + c) % 46 == 0 {
                    sum += c_a[a] * c_c[c];
                }
            }
        }
    }
    let ans = sum;
    println!("{}", ans);
}

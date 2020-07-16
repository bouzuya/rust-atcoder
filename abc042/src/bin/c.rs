use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: [usize; k],
    };
    let mut x = vec![true; 10];
    for &d_i in d.iter() {
        x[d_i] = false;
    }
    for i in n.. {
        let mut ok = true;
        let mut m = i;
        while m > 0 {
            if !x[m % 10] {
                ok = false;
                break;
            }
            m /= 10;
        }
        if ok {
            println!("{}", i);
            return;
        }
    }
}

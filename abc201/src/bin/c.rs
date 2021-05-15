use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut count = 0;
    for d_0 in 0..=9 {
        for d_1 in 0..=9 {
            for d_2 in 0..=9 {
                for d_3 in 0..=9 {
                    let ds = vec![d_0, d_1, d_2, d_3];
                    let mut ok = true;
                    for (i, &s_i) in s.iter().enumerate() {
                        match s_i {
                            'o' => {
                                if !ds.contains(&i) {
                                    ok = false;
                                    break;
                                }
                            }
                            'x' => {
                                if ds.contains(&i) {
                                    ok = false;
                                    break;
                                }
                            }
                            '?' => {}
                            _ => unreachable!(),
                        }
                    }
                    if ok {
                        count += 1;
                    }
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}

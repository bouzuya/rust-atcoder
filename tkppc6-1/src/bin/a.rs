use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    let ans = if n < 2015 {
        -1
    } else if n == 2015 {
        1
    } else if n == 2016 {
        2
    } else if n == 2017 {
        -1
    } else if n == 2018 {
        3
    } else if n == 2019 {
        4
    } else if n == 2020 {
        5
    } else {
        n - 2015
    };
    println!("{}", if ans <= 0 { -1 } else { ans });
}

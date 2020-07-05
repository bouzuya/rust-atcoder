use self::mod_u32::ModU32;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };

    let mut pa = vec![];
    let mut c_z = 0;
    let mut ma = vec![];
    for &a_i in a.iter() {
        if a_i == 0 {
            c_z += 1;
        } else if a_i > 0 {
            pa.push(a_i);
        } else if a_i < 0 {
            ma.push(a_i);
        } else {
            unreachable!()
        }
    }

    let c_p = pa.len();
    let c_m = ma.len();
    let kn = if k < c_m as i64 / 2 * 2 {
        k % 2
    } else {
        k - c_m as i64 / 2 * 2
    };
    if kn <= c_z as i64 + c_p as i64 {
        pa.sort_by_key(|&a_i| -a_i.abs());
        ma.sort_by_key(|&a_i| -a_i.abs());

        let mut ma2 = vec![];
        for i in (0..ma.len()).step_by(2) {
            if i + 1 < ma.len() {
                ma2.push(ma[i] * ma[i + 1]);
            } else {
                ma2.push(ma[i]);
            }
        }

        let mut i_p = 0_usize;
        let mut i_m = 0_usize;
        let mut i_z = 0_usize;
        let mut ans = ModU32::new(1);
        while i_p + i_m + i_z < k as usize {
            let x = match (pa.get(i_p), ma2.get(i_m)) {
                (None, None) => {
                    i_z += 1;
                    ModU32::new(0)
                }
                (None, Some(&x)) => {
                    i_m += 2;
                    ModU32::new(x as u64)
                }
                (Some(&x), None) => {
                    i_p += 1;
                    ModU32::new(x as u64)
                }
                (Some(&x), Some(&y)) => {
                    if x >= y {
                        i_p += 1;
                        ModU32::new(x as u64)
                    } else {
                        i_m += 2;
                        ModU32::new(y as u64)
                    }
                }
            };
            ans = ans * x;
        }
        println!("{}", ans);
    } else {
        pa.sort_by_key(|&a_i| a_i.abs());
        ma.sort_by_key(|&a_i| a_i.abs());
        let mut i_p = 0;
        let mut i_m = 0;
        let mut ans = ModU32::new(1);
        for _ in 0..k {
            let x = match (pa.get(i_p), ma.get(i_m)) {
                (None, None) => ModU32::new(0),
                (None, Some(&x)) => {
                    i_m += 1;
                    ModU32::new(0) - ModU32::new(x.abs() as u64)
                }
                (Some(&x), None) => {
                    i_p += 1;
                    ModU32::new(x as u64)
                }
                (Some(&x), Some(&y)) => {
                    if x.abs() <= y.abs() {
                        i_p += 1;
                        ModU32::new(x as u64)
                    } else {
                        i_m += 1;
                        ModU32::new(0) - ModU32::new(y.abs() as u64)
                    }
                }
            };
            ans = ans * x;
        }
        println!("{}", ans);
    }
}

mod mod_u32 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ModU32(u32);

    const MODP: u32 = 1_000_000_007;

    impl ModU32 {
        pub fn inv(self) -> Self {
            if self.0 == 0 {
                panic!()
            };
            self.pow(MODP - 2)
        }

        pub fn new(x: u64) -> Self {
            Self((x % (MODP as u64)) as u32)
        }

        pub fn pow(self, exp: u32) -> Self {
            let mut b: ModU32 = self;
            let mut a: ModU32 = Self::new(1);
            let mut x = exp;
            while x > 1 {
                if x & 1 == 1 {
                    a *= b;
                }
                x /= 2;
                b *= b;
            }
            if x == 1 {
                a *= b;
            }
            a
        }
    }

    impl From<u32> for ModU32 {
        fn from(x: u32) -> Self {
            ModU32::new(x as u64)
        }
    }

    impl From<u64> for ModU32 {
        fn from(x: u64) -> Self {
            ModU32::new(x)
        }
    }

    impl From<usize> for ModU32 {
        fn from(x: usize) -> Self {
            ModU32::from(x as u64)
        }
    }

    impl Into<u32> for ModU32 {
        fn into(self) -> u32 {
            self.0
        }
    }

    impl Into<u64> for ModU32 {
        fn into(self) -> u64 {
            self.0 as u64
        }
    }

    impl Into<usize> for ModU32 {
        fn into(self) -> usize {
            self.0 as usize
        }
    }

    impl std::fmt::Display for ModU32 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::ops::Add<ModU32> for ModU32 {
        type Output = ModU32;

        fn add(self, rhs: ModU32) -> Self::Output {
            (self.0 as u64 + rhs.0 as u64).into()
        }
    }

    impl std::ops::AddAssign<ModU32> for ModU32 {
        fn add_assign(&mut self, rhs: ModU32) {
            self.0 = (*self + rhs).0
        }
    }

    impl std::ops::Sub<ModU32> for ModU32 {
        type Output = ModU32;

        fn sub(self, rhs: ModU32) -> Self::Output {
            (if self.0 < rhs.0 {
                self.0 + (MODP - rhs.0)
            } else {
                self.0 - rhs.0
            })
            .into()
        }
    }

    impl std::ops::SubAssign<ModU32> for ModU32 {
        fn sub_assign(&mut self, rhs: ModU32) {
            self.0 = (*self - rhs).0
        }
    }

    impl std::ops::Mul<ModU32> for ModU32 {
        type Output = ModU32;

        fn mul(self, rhs: ModU32) -> Self::Output {
            Self::new((self.0 as u64) * (rhs.0 as u64))
        }
    }

    impl std::ops::MulAssign<ModU32> for ModU32 {
        fn mul_assign(&mut self, rhs: ModU32) {
            self.0 = (*self * rhs).0
        }
    }

    impl std::ops::Div<ModU32> for ModU32 {
        type Output = ModU32;

        fn div(self, rhs: ModU32) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl std::ops::DivAssign<ModU32> for ModU32 {
        fn div_assign(&mut self, rhs: ModU32) {
            self.0 = (*self / rhs).0
        }
    }
}

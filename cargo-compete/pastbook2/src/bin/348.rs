use lazysegtree::*;
use proconio::input;
use segtree::*;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    let mut b = vec![0];
    let mut s = vec![0];
    let mut sum_a = 0;
    let mut max_b = 0;
    for (a_i, b_i) in ab {
        sum_a += a_i;
        if max_b < b_i {
            max_b = b_i;
            b.push(max_b);
            s.push(sum_a);
        }
    }

    let m = s.len();

    #[derive(Clone, Copy, Debug)]
    struct M(i64);
    impl Monoid for M {
        type S = M;
        fn identity() -> Self::S {
            M(1_i64 << 60)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            M(a.0.min(b.0))
        }
    }

    struct F;
    impl MapMonoid for F {
        type M = M;
        type F = M;
        fn identity_map() -> Self::F {
            M(1_i64 << 60)
        }
        fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            M(f.0.min(x.0))
        }
        fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
            M(f.0.min(g.0))
        }
    }

    let mut lst = LazySegtree::<F>::new(m + 1);
    lst.set(0, M(0));

    for i in 0..m - 1 {
        let dp_i = lst.get(i).0;
        let money = s[i + 1] - dp_i - b[i];
        let j = b.upper_bound(&money);
        if i + 1 < j {
            lst.apply_range(i + 1..j.min(m + 1), M(dp_i + b[i]));
        }
    }

    let ans = sum_a - lst.get(m - 1).0 - b[m - 1];
    if ans < 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod internal_bit {
    // Skipped:
    //
    // - `bsf` = `__builtin_ctz`: is equivalent to `{integer}::trailing_zeros`

    #[allow(dead_code)]
    pub(crate) fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn ceil_pow2() {
            // https://github.com/atcoder/ac-library/blob/2088c8e2431c3f4d29a2cfabc6529fe0a0586c48/test/unittest/bit_test.cpp
            assert_eq!(0, super::ceil_pow2(0));
            assert_eq!(0, super::ceil_pow2(1));
            assert_eq!(1, super::ceil_pow2(2));
            assert_eq!(2, super::ceil_pow2(3));
            assert_eq!(2, super::ceil_pow2(4));
            assert_eq!(3, super::ceil_pow2(5));
            assert_eq!(3, super::ceil_pow2(6));
            assert_eq!(3, super::ceil_pow2(7));
            assert_eq!(3, super::ceil_pow2(8));
            assert_eq!(4, super::ceil_pow2(9));
            assert_eq!(30, super::ceil_pow2(1 << 30));
            assert_eq!(31, super::ceil_pow2((1 << 30) + 1));

            assert_eq!(32, super::ceil_pow2(u32::max_value()));
        }
    }
}
pub mod internal_type_traits {
    use std::{
        fmt,
        iter::{Product, Sum},
        ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        },
    };

    // Skipped:
    //
    // - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
    // - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
    // - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

    /// Corresponds to `std::is_integral` in C++.
    // We will remove unnecessary bounds later.
    //
    // Maybe we should rename this to `PrimitiveInteger` or something, as it probably won't be used in the
    // same way as the original ACL.
    pub trait Integral:
        'static
        + Send
        + Sync
        + Copy
        + Ord
        + Not<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Sum
        + Product
        + BitOr<Output = Self>
        + BitAnd<Output = Self>
        + BitXor<Output = Self>
        + BitOrAssign
        + BitAndAssign
        + BitXorAssign
        + Shl<Output = Self>
        + Shr<Output = Self>
        + ShlAssign
        + ShrAssign
        + fmt::Display
        + fmt::Debug
        + fmt::Binary
        + fmt::Octal
        + Zero
        + One
        + BoundedBelow
        + BoundedAbove
    {
    }

    /// Class that has additive identity element
    pub trait Zero {
        /// The additive identity element
        fn zero() -> Self;
    }

    /// Class that has multiplicative identity element
    pub trait One {
        /// The multiplicative identity element
        fn one() -> Self;
    }

    pub trait BoundedBelow {
        fn min_value() -> Self;
    }

    pub trait BoundedAbove {
        fn max_value() -> Self;
    }

    macro_rules! impl_integral {
  ($($ty:ty),*) => {
      $(
          impl Zero for $ty {
              #[inline]
              fn zero() -> Self {
                  0
              }
          }

          impl One for $ty {
              #[inline]
              fn one() -> Self {
                  1
              }
          }

          impl BoundedBelow for $ty {
              #[inline]
              fn min_value() -> Self {
                  Self::min_value()
              }
          }

          impl BoundedAbove for $ty {
              #[inline]
              fn max_value() -> Self {
                  Self::max_value()
              }
          }

          impl Integral for $ty {}
      )*
  };
}

    impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
pub mod lazysegtree {
    use crate::internal_bit::ceil_pow2;
    use crate::Monoid;

    pub trait MapMonoid {
        type M: Monoid;
        type F: Clone;
        // type S = <Self::M as Monoid>::S;
        fn identity_element() -> <Self::M as Monoid>::S {
            Self::M::identity()
        }
        fn binary_operation(
            a: &<Self::M as Monoid>::S,
            b: &<Self::M as Monoid>::S,
        ) -> <Self::M as Monoid>::S {
            Self::M::binary_operation(a, b)
        }
        fn identity_map() -> Self::F;
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }

    impl<F: MapMonoid> Default for LazySegtree<F> {
        fn default() -> Self {
            Self::new(0)
        }
    }
    impl<F: MapMonoid> LazySegtree<F> {
        pub fn new(n: usize) -> Self {
            vec![F::identity_element(); n].into()
        }
    }
    impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for LazySegtree<F> {
        fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![F::identity_element(); 2 * size];
            let lz = vec![F::identity_map(); size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = LazySegtree {
                n,
                size,
                log,
                d,
                lz,
            };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }

    impl<F: MapMonoid> LazySegtree<F> {
        pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::S {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p].clone()
        }

        pub fn prod<R>(&mut self, range: R) -> <F::M as Monoid>::S
        where
            R: RangeBounds<usize>,
        {
            // Trivial optimization
            if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
                return self.all_prod();
            }

            let mut r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let mut l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                // TODO: There are another way of optimizing [0..r)
                Bound::Unbounded => 0,
            };

            assert!(l <= r && r <= self.n);
            if l == r {
                return F::identity_element();
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }

            let mut sml = F::identity_element();
            let mut smr = F::identity_element();
            while l < r {
                if l & 1 != 0 {
                    sml = F::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = F::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            F::binary_operation(&sml, &smr)
        }

        pub fn all_prod(&self) -> <F::M as Monoid>::S {
            self.d[1].clone()
        }

        pub fn apply(&mut self, mut p: usize, f: F::F) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = F::mapping(&f, &self.d[p]);
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }
        pub fn apply_range<R>(&mut self, range: R, f: F::F)
        where
            R: RangeBounds<usize>,
        {
            let mut r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let mut l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                // TODO: There are another way of optimizing [0..r)
                Bound::Unbounded => 0,
            };

            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }

            {
                let l2 = l;
                let r2 = r;
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
                l = l2;
                r = r2;
            }

            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }

        pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
        where
            G: Fn(<F::M as Monoid>::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(g(F::identity_element()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            for i in (1..=self.log).rev() {
                self.push(l >> i);
            }
            let mut sm = F::identity_element();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !g(F::binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        self.push(l);
                        l *= 2;
                        let res = F::binary_operation(&sm, &self.d[l]);
                        if g(res.clone()) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = F::binary_operation(&sm, &self.d[l]);
                l += 1;
                //while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
        where
            G: Fn(<F::M as Monoid>::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(g(F::identity_element()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            for i in (1..=self.log).rev() {
                self.push((r - 1) >> i);
            }
            let mut sm = F::identity_element();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 != 0 {
                    r >>= 1;
                }
                if !g(F::binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        self.push(r);
                        r = 2 * r + 1;
                        let res = F::binary_operation(&self.d[r], &sm);
                        if g(res.clone()) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = F::binary_operation(&self.d[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }
    }

    pub struct LazySegtree<F>
    where
        F: MapMonoid,
    {
        n: usize,
        size: usize,
        log: usize,
        d: Vec<<F::M as Monoid>::S>,
        lz: Vec<F::F>,
    }
    impl<F> LazySegtree<F>
    where
        F: MapMonoid,
    {
        fn update(&mut self, k: usize) {
            self.d[k] = F::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F::F) {
            self.d[k] = F::mapping(&f, &self.d[k]);
            if k < self.size {
                self.lz[k] = F::composition(&f, &self.lz[k]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lz[k].clone());
            self.all_apply(2 * k + 1, self.lz[k].clone());
            self.lz[k] = F::identity_map();
        }
    }

    // TODO is it useful?
    use std::{
        fmt::{Debug, Error, Formatter, Write},
        ops::{Bound, RangeBounds},
    };
    impl<F> Debug for LazySegtree<F>
    where
        F: MapMonoid,
        F::F: Debug,
        <F::M as Monoid>::S: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            for i in 0..self.log {
                for j in 0..1 << i {
                    f.write_fmt(format_args!(
                        "{:?}[{:?}]\t",
                        self.d[(1 << i) + j],
                        self.lz[(1 << i) + j]
                    ))?;
                }
                f.write_char('\n')?;
            }
            for i in 0..self.size {
                f.write_fmt(format_args!("{:?}\t", self.d[self.size + i]))?;
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use std::ops::{Bound::*, RangeBounds};

        use crate::{LazySegtree, MapMonoid, Max};

        struct MaxAdd;
        impl MapMonoid for MaxAdd {
            type M = Max<i32>;
            type F = i32;

            fn identity_map() -> Self::F {
                0
            }

            fn mapping(&f: &i32, &x: &i32) -> i32 {
                f + x
            }

            fn composition(&f: &i32, &g: &i32) -> i32 {
                f + g
            }
        }

        #[test]
        fn test_max_add_lazy_segtree() {
            let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
            let n = base.len();
            let mut segtree: LazySegtree<MaxAdd> = base.clone().into();
            check_segtree(&base, &mut segtree);

            let mut segtree = LazySegtree::<MaxAdd>::new(n);
            let mut internal = vec![i32::min_value(); n];
            for i in 0..n {
                segtree.set(i, base[i]);
                internal[i] = base[i];
                check_segtree(&internal, &mut segtree);
            }

            segtree.set(6, 5);
            internal[6] = 5;
            check_segtree(&internal, &mut segtree);

            segtree.apply(5, 1);
            internal[5] += 1;
            check_segtree(&internal, &mut segtree);

            segtree.set(6, 0);
            internal[6] = 0;
            check_segtree(&internal, &mut segtree);

            segtree.apply_range(3..8, 2);
            internal[3..8].iter_mut().for_each(|e| *e += 2);
            check_segtree(&internal, &mut segtree);

            segtree.apply_range(2..=5, 7);
            internal[2..=5].iter_mut().for_each(|e| *e += 7);
            check_segtree(&internal, &mut segtree);
        }

        //noinspection DuplicatedCode
        fn check_segtree(base: &[i32], segtree: &mut LazySegtree<MaxAdd>) {
            let n = base.len();
            #[allow(clippy::needless_range_loop)]
            for i in 0..n {
                assert_eq!(segtree.get(i), base[i]);
            }

            check(base, segtree, ..);
            for i in 0..=n {
                check(base, segtree, ..i);
                check(base, segtree, i..);
                if i < n {
                    check(base, segtree, ..=i);
                }
                for j in i..=n {
                    check(base, segtree, i..j);
                    if j < n {
                        check(base, segtree, i..=j);
                        check(base, segtree, (Excluded(i), Included(j)));
                    }
                }
            }
            assert_eq!(
                segtree.all_prod(),
                base.iter().max().copied().unwrap_or(i32::min_value())
            );
            for k in 0..=10 {
                let f = |x| x < k;
                for i in 0..=n {
                    assert_eq!(
                        Some(segtree.max_right(i, f)),
                        (i..=n)
                            .filter(|&j| f(base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .max()
                    );
                }
                for j in 0..=n {
                    assert_eq!(
                        Some(segtree.min_left(j, f)),
                        (0..=j)
                            .filter(|&i| f(base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .min()
                    );
                }
            }
        }

        fn check(base: &[i32], segtree: &mut LazySegtree<MaxAdd>, range: impl RangeBounds<usize>) {
            let expected = base
                .iter()
                .enumerate()
                .filter_map(|(i, a)| Some(a).filter(|_| range.contains(&i)))
                .max()
                .copied()
                .unwrap_or(i32::min_value());
            assert_eq!(segtree.prod(range), expected);
        }
    }
}
pub mod segtree {
    use crate::internal_bit::ceil_pow2;
    use crate::internal_type_traits::{BoundedAbove, BoundedBelow, One, Zero};
    use std::cmp::{max, min};
    use std::convert::Infallible;
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::ops::{Add, BitAnd, BitOr, BitXor, Bound, Mul, Not, RangeBounds};

    // TODO Should I split monoid-related traits to another module?
    pub trait Monoid {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
    }

    pub struct Max<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Max<S>
    where
        S: Copy + Ord + BoundedBelow,
    {
        type S = S;
        fn identity() -> Self::S {
            S::min_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            max(*a, *b)
        }
    }

    pub struct Min<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Min<S>
    where
        S: Copy + Ord + BoundedAbove,
    {
        type S = S;
        fn identity() -> Self::S {
            S::max_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            min(*a, *b)
        }
    }

    pub struct Additive<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Additive<S>
    where
        S: Copy + Add<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }

    pub struct Multiplicative<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Multiplicative<S>
    where
        S: Copy + Mul<Output = S> + One,
    {
        type S = S;
        fn identity() -> Self::S {
            S::one()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a * *b
        }
    }

    pub struct BitwiseOr<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseOr<S>
    where
        S: Copy + BitOr<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a | *b
        }
    }

    pub struct BitwiseAnd<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseAnd<S>
    where
        S: Copy + BitAnd<Output = S> + Not<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            !S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a & *b
        }
    }

    pub struct BitwiseXor<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseXor<S>
    where
        S: Copy + BitXor<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
    }

    impl<M: Monoid> Default for Segtree<M> {
        fn default() -> Self {
            Segtree::new(0)
        }
    }
    impl<M: Monoid> Segtree<M> {
        pub fn new(n: usize) -> Segtree<M> {
            vec![M::identity(); n].into()
        }
    }
    impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
        fn from(v: Vec<M::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![M::identity(); 2 * size];
            d[size..][..n].clone_from_slice(&v);
            let mut ret = Segtree { n, size, log, d };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }
    impl<M: Monoid> FromIterator<M::S> for Segtree<M> {
        fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
            let iter = iter.into_iter();
            let n = iter.size_hint().0;
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = Vec::with_capacity(size * 2);
            d.extend(
                std::iter::repeat_with(M::identity)
                    .take(size)
                    .chain(iter)
                    .chain(std::iter::repeat_with(M::identity).take(size - n)),
            );
            let mut ret = Segtree { n, size, log, d };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }
    impl<M: Monoid> Segtree<M> {
        pub fn set(&mut self, mut p: usize, x: M::S) {
            assert!(p < self.n);
            p += self.size;
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&self, p: usize) -> M::S {
            assert!(p < self.n);
            self.d[p + self.size].clone()
        }

        pub fn get_slice(&self) -> &[M::S] {
            &self.d[self.size..][..self.n]
        }

        pub fn prod<R>(&self, range: R) -> M::S
        where
            R: RangeBounds<usize>,
        {
            // Trivial optimization
            if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
                return self.all_prod();
            }

            let mut r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let mut l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                // TODO: There are another way of optimizing [0..r)
                Bound::Unbounded => 0,
            };

            assert!(l <= r && r <= self.n);
            let mut sml = M::identity();
            let mut smr = M::identity();
            l += self.size;
            r += self.size;

            while l < r {
                if l & 1 != 0 {
                    sml = M::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = M::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            M::binary_operation(&sml, &smr)
        }

        pub fn all_prod(&self) -> M::S {
            self.d[1].clone()
        }

        pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(f(&M::identity()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            let mut sm = M::identity();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !f(&M::binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        l *= 2;
                        let res = M::binary_operation(&sm, &self.d[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = M::binary_operation(&sm, &self.d[l]);
                l += 1;
                // while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(f(&M::identity()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            let mut sm = M::identity();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }
                if !f(&M::binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        r = 2 * r + 1;
                        let res = M::binary_operation(&self.d[r], &sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = M::binary_operation(&self.d[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }

        fn update(&mut self, k: usize) {
            self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
    }

    // Maybe we can use this someday
    // ```
    // for i in 0..=self.log {
    //     for j in 0..1 << i {
    //         print!("{}\t", self.d[(1 << i) + j]);
    //     }
    //     println!();
    // }
    // ```

    pub struct Segtree<M>
    where
        M: Monoid,
    {
        // variable name is _n in original library
        n: usize,
        size: usize,
        log: usize,
        d: Vec<M::S>,
    }

    #[cfg(test)]
    mod tests {
        use crate::segtree::Max;
        use crate::Segtree;
        use std::ops::{Bound::*, RangeBounds};

        #[test]
        fn test_max_segtree() {
            let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
            let n = base.len();
            let segtree: Segtree<Max<_>> = base.clone().into();
            check_segtree(&base, &segtree);

            let mut segtree = Segtree::<Max<_>>::new(n);
            let mut internal = vec![i32::min_value(); n];
            for i in 0..n {
                segtree.set(i, base[i]);
                internal[i] = base[i];
                check_segtree(&internal, &segtree);
            }

            segtree.set(6, 5);
            internal[6] = 5;
            check_segtree(&internal, &segtree);

            segtree.set(6, 0);
            internal[6] = 0;
            check_segtree(&internal, &segtree);
        }

        //noinspection DuplicatedCode
        fn check_segtree(base: &[i32], segtree: &Segtree<Max<i32>>) {
            let n = base.len();
            #[allow(clippy::needless_range_loop)]
            for i in 0..n {
                assert_eq!(segtree.get(i), base[i]);
            }

            check(base, segtree, ..);
            for i in 0..=n {
                check(base, segtree, ..i);
                check(base, segtree, i..);
                if i < n {
                    check(base, segtree, ..=i);
                }
                for j in i..=n {
                    check(base, segtree, i..j);
                    if j < n {
                        check(base, segtree, i..=j);
                        check(base, segtree, (Excluded(i), Included(j)));
                    }
                }
            }
            assert_eq!(
                segtree.all_prod(),
                base.iter().max().copied().unwrap_or(i32::min_value())
            );
            for k in 0..=10 {
                let f = |&x: &i32| x < k;
                for i in 0..=n {
                    assert_eq!(
                        Some(segtree.max_right(i, f)),
                        (i..=n)
                            .filter(|&j| f(&base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .max()
                    );
                }
                for j in 0..=n {
                    assert_eq!(
                        Some(segtree.min_left(j, f)),
                        (0..=j)
                            .filter(|&i| f(&base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .min()
                    );
                }
            }
        }

        fn check(base: &[i32], segtree: &Segtree<Max<i32>>, range: impl RangeBounds<usize>) {
            let expected = base
                .iter()
                .enumerate()
                .filter_map(|(i, a)| Some(a).filter(|_| range.contains(&i)))
                .max()
                .copied()
                .unwrap_or(i32::min_value());
            assert_eq!(segtree.prod(range), expected);
        }
    }
}

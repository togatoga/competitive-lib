use cargo_snippet::snippet;
#[allow(clippy::module_inception, clippy::ptr_arg)]
#[snippet]
/// A 2D-geometry library
pub mod geometry2d {
    use std::{
        fmt,
        iter::{Product, Sum},
        ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
        },
    };

    pub trait Geo2dNum:
        Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + Neg<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Sum
        + Product
        + fmt::Display
        + fmt::Debug
        + Clone
        + Copy
        + Default
        + PartialOrd
        + From<i8>
    {
        fn is_zero(&self) -> bool;
        fn is_positive(&self) -> bool;
        fn is_negative(&self) -> bool;
        fn abs(&self) -> Self;
    }

    impl Geo2dNum for i64 {
        fn is_zero(&self) -> bool {
            *self == 0
        }
        fn is_positive(&self) -> bool {
            *self > 0
        }
        fn is_negative(&self) -> bool {
            *self < 0
        }
        fn abs(&self) -> Self {
            (*self).abs()
        }
    }

    pub const F64_EPS: f64 = 1e-8;
    impl Geo2dNum for f64 {
        fn is_zero(&self) -> bool {
            self.abs() < F64_EPS
        }
        fn is_positive(&self) -> bool {
            *self >= F64_EPS
        }
        fn is_negative(&self) -> bool {
            *self <= -F64_EPS
        }
        fn abs(&self) -> Self {
            (*self).abs()
        }
    }

    pub trait Geo2Decimal {
        fn sqrt(&self) -> Self;
    }

    impl Geo2Decimal for f64 {
        fn sqrt(&self) -> Self {
            (*self).sqrt()
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Point2d<T: Geo2dNum> {
        pub x: T,
        pub y: T,
    }
    impl<T: Geo2dNum> Point2d<T> {
        /// Makes a `Point2d` from (x, y)
        pub fn new(x: T, y: T) -> Self {
            Point2d { x, y }
        }
        /// Returns a cross product
        /// a x b
        pub fn cross(&self, b: &Point2d<T>) -> T {
            self.x * b.y - self.y * b.x
        }
        /// Returns a dot producet
        /// a ・ b
        pub fn dot(&self, b: &Point2d<T>) -> T {
            self.x * b.x + self.y * b.y
        }
        pub fn norm(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }

    impl<T: Geo2dNum> Add<Point2d<T>> for Point2d<T> {
        type Output = Point2d<T>;
        fn add(self, rhs: Point2d<T>) -> Self::Output {
            Point2d {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    impl<T: Geo2dNum> Sub<Point2d<T>> for Point2d<T> {
        type Output = Point2d<T>;
        fn sub(self, rhs: Point2d<T>) -> Self::Output {
            Point2d {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl<T: Geo2dNum> Mul<T> for Point2d<T> {
        type Output = Point2d<T>;
        fn mul(self, rhs: T) -> Self::Output {
            Point2d {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }
    impl<T: Geo2dNum> Div<T> for Point2d<T> {
        type Output = Point2d<T>;
        fn div(self, rhs: T) -> Self::Output {
            Point2d {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
    impl<T: Geo2dNum> PartialEq for Point2d<T> {
        fn eq(&self, other: &Self) -> bool {
            // (*self - *other).norm().abs() < F64_EPS
            (*self - *other).norm().is_zero()
        }
    }
    impl<T: Geo2dNum> Eq for Point2d<T> {}
    impl<T: Geo2dNum> Ord for Point2d<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if (self.x - other.x).is_zero() {
                self.y.partial_cmp(&other.y).unwrap()
            } else {
                self.x.partial_cmp(&other.x).unwrap()
            }
        }
    }
    impl<T: Geo2dNum> PartialOrd for Point2d<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    pub type Polygon<T> = Vec<Point2d<T>>;
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Circle<T: Geo2dNum + Geo2Decimal> {
        pub point: Point2d<T>,
        pub radius: T,
    }

    impl<T: Geo2dNum + Geo2Decimal> Circle<T> {
        /// Makes a `Radius` from (x, y, radius)
        pub fn new(x: T, y: T, r: T) -> Self {
            Circle {
                point: Point2d::new(x, y),
                radius: r,
            }
        }

        /// Returns points of intersection with circle if exists.
        /// The number of intersection must be one or two.
        pub fn intersection_with_circle(&self, other: &Circle<T>) -> Option<Vec<Point2d<T>>> {
            let (p1, p2) = (self.point, other.point);
            let (r1, r2) = (self.radius, other.radius);
            let dist = (p1 - p2).norm().sqrt();
            // same point
            if dist.is_zero() {
                return None;
            }

            // separeted
            // if dist - (r1 + r2) > F64_EPS {
            //     return None;
            // }
            if (dist - (r1 + r2)).is_positive() {
                return None;
            }

            // inclusion
            if ((r1 - r2).abs() - dist).is_positive() {
                return None;
            }

            let rcos = (dist * dist + r1 * r1 - r2 * r2) / (T::from(2) * dist);
            let rsin = (r1 * r1 - rcos * rcos).sqrt();
            let e = (p2 - p1) / dist;

            let mut points = Vec::with_capacity(2);
            // rotate and scale
            // |r*cos, -r*sin| |e.x|
            // |r*sin,  r*cos| |e.y|
            let rotate_and_scale = |e: Point2d<T>, rcos: T, rsin: T| -> Point2d<T> {
                Point2d::new(e.x * rcos - e.y * rsin, e.x * rsin + e.y * rcos)
            };
            let cp1 = p1 + rotate_and_scale(e, rcos, rsin);
            let cp2 = p1 + rotate_and_scale(e, rcos, -rsin);
            points.push(cp1);
            if !cp1.eq(&cp2) {
                points.push(cp2);
            }
            Some(points)
        }
    }

    #[derive(Debug, Clone, Copy)]
    /// `Position` represents that a given point is
    /// `PolygonOut` the outside of `Polygon`
    /// `PolygonIn`  in `Polygon`
    /// `PolygonOn`  on the segment of `Polygon`
    pub enum Position {
        PolygonOut,
        PolygonIn,
        PolygonOn,
    }
    /// Returns an enum `Position` indicating a point is (in|on) points(`Polygon`) or not.
    pub fn contains<T: Geo2dNum>(points: &Polygon<T>, point: Point2d<T>) -> Position {
        let mut contain = false;
        let n = points.len();
        for i in 0..n {
            let mut a = points[i] - point;
            let mut b = points[(i + 1) % n] - point;
            if a.y > b.y {
                std::mem::swap(&mut a, &mut b);
            }
            // if a.y <= 0.0 && 0.0 < b.y && a.cross(&b) < 0.0 {
            //     contain = !contain;
            // }
            if (a.y.is_zero() || a.y.is_negative())
                && b.y.is_positive()
                && a.cross(&b).is_negative()
            {
                contain = !contain;
            }
            let ab = a.dot(&b);
            if a.cross(&b).is_zero() && (ab.is_zero() || ab.is_negative()) {
                return Position::PolygonOn;
            }
        }
        if contain {
            Position::PolygonIn
        } else {
            Position::PolygonOut
        }
    }

    /// Returns an enum `Position` indicating a point is (in|on) points(`Polygon`) or not.
    /// The prerequisite `is_convex(&points)` is true.                                             c
    pub fn convex_contains<T: Geo2dNum>(points: &Polygon<T>, point: Point2d<T>) -> Position {
        let n = points.len();
        let p1 = (points[1] - points[0]).cross(&(point - points[0]));
        let p2 = (points[n - 1] - points[0]).cross(&(point - points[0]));

        if p1.is_negative() || p2.is_positive() {
            return Position::PolygonOut;
        }

        let mut left = 1;
        let mut right = n - 1;
        while right - left > 1 {
            let med = (left + right) / 2;
            let p = (point - points[0]).cross(&(points[med] - points[0]));
            if p.is_zero() || p.is_positive() {
                right = med;
            } else {
                left = med;
            }
        }
        let p = (points[left] - point).cross(&(points[right] - point));

        if p.is_zero() {
            return Position::PolygonOn;
        }

        if p.is_positive() {
            if p1.is_zero() || p2.is_zero() {
                return Position::PolygonOn;
            }
            return Position::PolygonIn;
        }
        Position::PolygonOut
    }

    /// Returns a boolean whether a `points` is convex or not.
    pub fn is_convex<T: Geo2dNum>(points: &Polygon<T>) -> bool {
        let n = points.len();
        for i in 0..n {
            let p0 = points[(i + n - 1) % n];
            let p1 = points[i];
            let p2 = points[(i + 1) % n];
            if ccw(p0, p1, p2) == Ccw::Clockwise {
                return false;
            }
        }
        true
    }

    /// Returns a minimum radius `Circle` enclsoing given all points.
    /// Expected: O(n)
    pub fn smallest_enclosing_circle<T: Geo2dNum + Geo2Decimal>(
        points: &Vec<Point2d<T>>,
        seed: u32,
    ) -> Circle<T> {
        let n = points.len();
        assert!(n >= 1);
        if n == 1 {
            return Circle::new(points[0].x, points[0].y, T::from(0));
        }
        //shuffle
        let mut points = points.clone();
        let mut rng = xorshift::Xorshift128::new(seed);
        for i in 0..n {
            points.swap(i, rng.gen() as usize % n);
        }
        let points = points;

        let make_circle_3 = |a: Point2d<T>, b: Point2d<T>, c: Point2d<T>| -> Circle<T> {
            let d1 = (b - c).norm();
            let d2 = (c - a).norm();
            let d3 = (a - b).norm();
            let s = (b - a).cross(&(c - a));

            let p = (a * d1 * (d2 + d3 - d1) + b * d2 * (d3 + d1 - d2) + c * d3 * (d1 + d2 - d3))
                / (T::from(4) * s * s);
            let r = (p - a).norm().sqrt();
            Circle {
                point: p,
                radius: r,
            }
        };

        let make_circle_2 = |a: Point2d<T>, b: Point2d<T>| -> Circle<T> {
            let c = (a + b) / T::from(2);
            let r = (a - c).norm().sqrt();

            Circle {
                point: c,
                radius: r,
            }
        };

        let in_circle = |a: Point2d<T>, c: Circle<T>| -> bool {
            // (a - c.point).norm() <= c.radius * c.radius + F64_EPS
            let result = c.radius * c.radius - (a - c.point).norm();
            result.is_zero() || result.is_positive()
        };

        let mut c = make_circle_2(points[0], points[1]);

        for i in 2..n {
            if in_circle(points[i], c) {
                continue;
            }
            c = make_circle_2(points[0], points[i]);

            for j in 1..i {
                if in_circle(points[j], c) {
                    continue;
                }
                c = make_circle_2(points[i], points[j]);

                for k in 0..j {
                    if in_circle(points[k], c) {
                        continue;
                    }
                    c = make_circle_3(points[i], points[j], points[k]);
                }
            }
        }
        c
    }
    /// Returns a convex hull.
    /// Supposed that all points are unique and the number of points is greater than 2
    pub fn convex_hull<T: Geo2dNum>(points: &Polygon<T>) -> Polygon<T> {
        let n = points.len();
        assert!(n >= 3);
        let mut points = points.clone();
        points.sort_by(|a, b| {
            if (a.x - b.x).abs().is_zero() {
                a.y.partial_cmp(&b.y).unwrap()
            } else {
                a.x.partial_cmp(&b.x).unwrap()
            }
        });
        let mut qs = Polygon::with_capacity(2 * n);
        for &point in points.iter() {
            while qs.len() >= 2
                && (qs[qs.len() - 1] - qs[qs.len() - 2])
                    .cross(&(point - qs[qs.len() - 1]))
                    .is_zero()
            {
                qs.pop();
            }
            qs.push(point);
        }
        let t = qs.len();
        for i in (0..=n - 2).rev() {
            while qs.len() > t
                && (qs[qs.len() - 1] - qs[qs.len() - 2])
                    .cross(&(points[i] - qs[qs.len() - 1]))
                    .is_zero()
            {
                qs.pop();
            }
            qs.push(points[i]);
        }
        qs.pop();
        qs
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// `Ccw` represents five positions for three points.
    /// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_1_C
    ///    Counter Clockwise      Clockwise         Online back      Online front    On Segment
    ///                                                                     c              b
    ///        b     c             c     b                b                /              /
    ///         \   /               \   /                /                b              c
    ///          \ /                 \ /                /                /              /
    ///           a                   a                a                a              a
    ///                                               /
    ///                                              c
    pub enum Ccw {
        CounterClockwise,
        Clockwise,
        OnlineBack,
        OnlineFront,
        OnSegment,
    }
    /// Returns a counter-clockwise result from given three points
    pub fn ccw<T: Geo2dNum>(a: Point2d<T>, b: Point2d<T>, c: Point2d<T>) -> Ccw {
        let b = b - a;
        let c = c - a;
        if b.cross(&c).is_positive() {
            Ccw::CounterClockwise
        } else if b.cross(&c).is_negative() {
            Ccw::Clockwise
        } else if b.dot(&c).is_negative() {
            Ccw::OnlineBack
        } else if b.norm() < c.norm() {
            Ccw::OnlineFront
        } else {
            Ccw::OnSegment
        }
    }
    #[allow(clippy::module_inception, clippy::many_single_char_names)]
    /// The period is 2^128 - 1
    mod xorshift {
        #[derive(Debug, Clone)]
        #[allow(dead_code)]
        pub struct Xorshift128 {
            x: u32,
            y: u32,
            z: u32,
            w: u32,
        }
        impl Default for Xorshift128 {
            fn default() -> Self {
                Xorshift128 {
                    x: 123456789,
                    y: 362436069,
                    z: 521288629,
                    w: 88675123,
                }
            }
        }
        impl Xorshift128 {
            pub fn new(seed: u32) -> Xorshift128 {
                let mut xorshift = Xorshift128::default();
                xorshift.z ^= seed;
                xorshift
            }
            pub fn gen(&mut self) -> u32 {
                let t = self.x ^ (self.x << 11);
                self.x = self.y;
                self.y = self.z;
                self.z = self.w;
                self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
                self.w
            }
        }
    }
}

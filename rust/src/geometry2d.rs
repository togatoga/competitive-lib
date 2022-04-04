use cargo_snippet::snippet;
#[allow(clippy::module_inception, clippy::ptr_arg)]
#[snippet]
/// A 2D-geometry library
pub mod geometry2d {
    use std::{
        mem::swap,
        ops::{Add, Div, Sub},
    };
    pub const EPS: f64 = 1e-8;
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Point2d {
        pub x: f64,
        pub y: f64,
    }
    impl Point2d {
        /// Makes a `Point2d` from (x, y)
        pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
            Point2d {
                x: x.into(),
                y: y.into(),
            }
        }
        /// Returns a cross product
        /// a x b
        pub fn cross(&self, b: &Point2d) -> f64 {
            self.x * b.y - self.y * b.x
        }
        /// Returns a dot producet
        /// a ・ b
        pub fn dot(&self, b: &Point2d) -> f64 {
            self.x * b.x + self.y * b.y
        }
        pub fn norm(&self) -> f64 {
            self.x * self.x + self.y * self.y
        }
    }

    impl Add<Point2d> for Point2d {
        type Output = Point2d;
        fn add(self, rhs: Point2d) -> Self::Output {
            Point2d {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    impl Sub<Point2d> for Point2d {
        type Output = Point2d;
        fn sub(self, rhs: Point2d) -> Self::Output {
            Point2d {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    impl Div<f64> for Point2d {
        type Output = Point2d;
        fn div(self, rhs: f64) -> Self::Output {
            Point2d {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
    impl PartialEq for Point2d {
        fn eq(&self, other: &Self) -> bool {
            (*self - *other).norm().abs() < EPS
        }
    }
    impl Eq for Point2d {}
    impl Ord for Point2d {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if (self.x - other.x).abs() < EPS {
                self.y.partial_cmp(&other.y).unwrap()
            } else {
                self.x.partial_cmp(&other.x).unwrap()
            }
        }
    }
    impl PartialOrd for Point2d {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    pub type Polygon = Vec<Point2d>;
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Circle {
        pub point: Point2d,
        pub radius: f64,
    }

    impl Circle {
        /// Makes a `Radius` from (x, y, radius)
        pub fn new<T: Into<f64>>(x: T, y: T, r: T) -> Self {
            Circle {
                point: Point2d::new(x, y),
                radius: r.into(),
            }
        }

        /// Returns points of intersection with circle if exists.
        /// The number of intersection must be one or two.
        pub fn intersection_with_circle(&self, other: &Circle) -> Option<Vec<Point2d>> {
            let (p1, p2) = (self.point, other.point);
            let (r1, r2) = (self.radius, other.radius);
            let dist = (p1 - p2).norm().sqrt();
            // same point
            if dist < EPS {
                return None;
            }
            // separeted
            if dist - (r1 + r2) > EPS {
                return None;
            }

            // inclusion
            if EPS < (r1 - r2).abs() - dist {
                return None;
            }

            let rcos = (dist * dist + r1 * r1 - r2 * r2) / (2. * dist);
            let rsin = (r1 * r1 - rcos * rcos).sqrt();
            let e = (p2 - p1) / dist;

            let mut points = Vec::with_capacity(2);
            // rotate and scale
            // |r*cos, -r*sin| |e.x|
            // |r*sin,  r*cos| |e.y|
            let rotate_and_scale = |e: Point2d, rcos: f64, rsin: f64| -> Point2d {
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
    pub fn contains(points: &Polygon, point: Point2d) -> Position {
        let mut contain = false;
        let n = points.len();
        for i in 0..n {
            let mut a = points[i] - point;
            let mut b = points[(i + 1) % n] - point;
            if a.y > b.y {
                swap(&mut a, &mut b);
            }
            if a.y <= 0.0 && 0.0 < b.y && a.cross(&b) < 0.0 {
                contain = !contain;
            }
            if a.cross(&b) == 0.0 && a.dot(&b) <= 0.0 {
                return Position::PolygonOn;
            }
        }
        if contain {
            Position::PolygonIn
        } else {
            Position::PolygonOut
        }
    }
    /// Returns a boolean whether a `points` is convex or not.
    pub fn is_convex(points: &Polygon) -> bool {
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
    /// Returns a convex hull.
    /// Supposed that all points are unique and the number of points is greater than 2
    pub fn convex_hull(points: &Polygon) -> Polygon {
        let n = points.len();
        assert!(n >= 3);
        let mut points = points.clone();
        points.sort_by(|a, b| {
            if (a.x - b.x).abs() < EPS {
                a.y.partial_cmp(&b.y).unwrap()
            } else {
                a.x.partial_cmp(&b.x).unwrap()
            }
        });
        let mut qs = Polygon::with_capacity(2 * n);
        for &point in points.iter() {
            while qs.len() >= 2
                && (qs[qs.len() - 1] - qs[qs.len() - 2]).cross(&(point - qs[qs.len() - 1])) < EPS
            {
                qs.pop();
            }
            qs.push(point);
        }
        let t = qs.len();
        for i in (0..=n - 2).rev() {
            while qs.len() > t
                && (qs[qs.len() - 1] - qs[qs.len() - 2]).cross(&(points[i] - qs[qs.len() - 1]))
                    < EPS
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
    pub fn ccw(a: Point2d, b: Point2d, c: Point2d) -> Ccw {
        let b = b - a;
        let c = c - a;
        if b.cross(&c) > EPS {
            Ccw::CounterClockwise
        } else if b.cross(&c) < -EPS {
            Ccw::Clockwise
        } else if b.dot(&c) < -EPS {
            Ccw::OnlineBack
        } else if b.norm() < c.norm() {
            Ccw::OnlineFront
        } else {
            Ccw::OnSegment
        }
    }
}

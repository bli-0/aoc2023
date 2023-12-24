use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/24");
    let hailstones: Vec<Hailstone> = inputs.lines().map(Hailstone::from_input).collect_vec();

    let mut total_count = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for (j, h2) in hailstones.iter().enumerate() {
            // Ignore duplicates
            if j <= i {
                continue;
            }

            if h1.intersects_x_y(h2, 200000000000000, 400000000000000) {
                total_count += 1;
            }
        }
    }

    println!("part1: {}", total_count);
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    start: Vector3,
    diff: Vector3,
}

impl Hailstone {
    fn from_input(s: &str) -> Self {
        // 19, 13, 30 @ -2,  1, -2
        let (start_str, diff_str) = s.split_once(" @ ").unwrap();
        let start = start_str
            .splitn(3, ", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let diff = diff_str
            .splitn(3, ", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        Self { start, diff }
    }

    fn intersects_x_y(&self, other: &Hailstone, lower_bound: i64, upper_bound: i64) -> bool {
        // Long winded formula:
        // if a stone has formula xn = cxn + tdxn
        // yn = xyn + tdyn
        // where t is time
        //
        // Then the two paths of h0, h1 interset of there is a t0 and t1 at which x and y values are equal for h0 and h1. (t0 could be equal to t1)
        // i.e. we have the following simultaneous equations:
        // x intersect:
        // cx0 + t0*dx0
        // cx1 + t1*dx1
        // for some values of t1, t0.
        // x intersect: cx0 + t0*dx0 == x1 = xc1 + t1*dx1
        // y intersect: cy0 + t0*dy0 == y1 = yc1 + t1*dy1
        //
        // Rearranging for t0 and t1 to find the times at which the paths can intersect.
        // Two things intersect at t0 after some rearranging at:
        // t0 = (dy1*kx - dx1*ky) / (dx0*dy1 - dx1*dy0)
        //
        // where kx = cx1 - xc0
        // ky = cy1 - cy0
        // In particular if the velocities are parallel:
        // dx1 == dx0, dy1 == dy0. Then the divisor is 0 and no intersections occur.
        // **ALSO assumes velocities are never zero** (as we divide by velocities in the formula rearranging).
        //
        // Similarly for the t1 intersect:
        //
        // t1 = = (- dy0*kx - dx0*ky) / (dx1*dy0 - dx0*dy1)
        //

        // Other == 1, this == 0;
        if other.diff.0 * self.diff.1 - self.diff.0 * other.diff.1 == 0 {
            // Parallel.
            return false;
        }
        let c_x = other.start.0 - self.start.0;
        let c_y = other.start.1 - self.start.1;

        let denominator0 = (other.diff.1 * self.diff.0 - self.diff.1 * other.diff.0) as f64;
        let t_0 = (other.diff.1 * c_x - other.diff.0 * c_y) as f64 / denominator0;

        let (x, y) = (
            self.start.0 as f64 + self.diff.0 as f64 * t_0,
            self.start.1 as f64 + self.diff.1 as f64 * t_0,
        );
        // Debug assert that this is the same as if we took the formula for t1..
        let denominator1 = (other.diff.1 * self.diff.0 - self.diff.1 * other.diff.0) as f64;
        let t_1 = (c_x * self.diff.1 - c_y * self.diff.0) as f64 / denominator1;
        let (x1, y1) = (
            other.start.0 as f64 + other.diff.0 as f64 * t_1,
            other.start.1 as f64 + other.diff.1 as f64 * t_1,
        );
        let x_str = format!("{:.3}", x);
        let y_str = format!("{:.3}", y);
        let x1_str = format!("{:.3}", x1);
        let y1_str = format!("{:.3}", y1);

        // Round up to 3 decimal strings..
        debug_assert_eq!((x_str, y_str), (x1_str, y1_str));

        if t_0 < 0.0 || t_1 < 0.0 {
            return false;
        }

        if (x >= lower_bound as f64 && x <= upper_bound as f64)
            && (y >= lower_bound as f64 && y <= upper_bound as f64)
        {
            return true;
        }
        false
    }
}

type Vector3 = (i64, i64, i64);

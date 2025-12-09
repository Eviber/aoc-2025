use itertools::Itertools;

pub fn solve(s: &str) -> i64 {
    let polygon = parse_points(s);
    // let (width, height) = polygon
    //     .iter()
    //     .fold((0, 0), |(w, h), &p| (w.max(p.x), h.max(p.y)));
    // eprintln!("{} x {}", width, height);
    println!("{}", rect_in_polygon(Point { x: 7, y: 3 }, Point { x: 11, y: 1 }, &polygon));
    println!("{}", line_in_polygon(Point { x: 7, y: 3 }, Point { x: 11, y: 3 }, &polygon));
    // FIXME: this should return true
    println!("{}", Point { x: 10, y: 3 }.in_polygon(&polygon));
    let mut largest_area = 0;
    for points in polygon.iter().combinations(2) {
        println!("{:?},{:?}:", points[0], points[1]);
        if rect_in_polygon(*points[0], *points[1], &polygon) {
            println!("  in poly");
            let area = points[0].area(points[1]);
            if area > largest_area {
                println!("new largest: {}", area);
            }
            largest_area = area.max(largest_area);
        }
    }
    largest_area
}

fn rect_in_polygon(a: Point, b: Point, polygon: &[Point]) -> bool {
    // assuming interior loops are inside the polygon because I sure hope they do
    let c = Point { x: a.x, y: b.y };
    let d = Point { x: b.x, y: a.y };
    line_in_polygon(a, c, polygon)
        && line_in_polygon(c, b, polygon)
        && line_in_polygon(b, d, polygon)
        && line_in_polygon(d, a, polygon)
}

fn line_in_polygon(a: Point, b: Point, polygon: &[Point]) -> bool {
    if a.x == b.x {
        vline_in_polygon(a.x, a.y, b.y, polygon)
    } else if a.y == b.y {
        hline_in_polygon(a.y, a.x, b.x, polygon)
    } else {
        panic!("points should share an axis");
    }
}

fn vline_in_polygon(x: i64, y_1: i64, y_2: i64, polygon: &[Point]) -> bool {
    let min_y = y_1.min(y_2);
    let max_y = y_1.max(y_2);
    for y in min_y..=max_y {
        let p = Point { x, y };
        if !p.in_polygon(polygon) {
            return false;
        }
    }
    true
}

fn hline_in_polygon(y: i64, x_1: i64, x_2: i64, polygon: &[Point]) -> bool {
    let min_x = x_1.min(x_2);
    let max_x = x_1.max(x_2);
    for x in min_x..=max_x {
        let p = Point { x, y };
        if !p.in_polygon(polygon) {
            return false;
        }
    }
    true
}

#[derive(Clone, Copy, Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x).abs() + 1;
        let dy = (self.y - other.y).abs() + 1;
        dx * dy
    }

    fn in_polygon(&self, polygon: &[Point]) -> bool {
        assert!(
            polygon.len() >= 2,
            "polygon must contain at least two points"
        );
        if self.in_rect(polygon[0], *polygon.last().unwrap()) {
            return true;
        }
        for rect in polygon.windows(2) {
            if self.in_rect(rect[0], rect[1]) {
                return true;
            }
        }
        false
    }

    fn in_rect(&self, a: Point, b: Point) -> bool {
        let min_x = a.x.min(b.x);
        let max_x = a.x.max(b.x);
        let min_y = a.y.min(b.y);
        let max_y = a.y.max(b.y);
        min_x <= self.x && self.x <= max_x && min_y <= self.y && self.y <= max_y
    }
}

fn parse_points(s: &str) -> Vec<Point> {
    s.lines().map(|part| part.trim().parse().unwrap()).collect()
}

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i64> = s
            .split(',')
            .map(|part| part.trim().parse::<i64>().map_err(|_| ()))
            .collect::<Result<Vec<i64>, ()>>()?;
        if coords.len() != 2 {
            return Err(());
        }
        Ok(Point {
            x: coords[0],
            y: coords[1],
        })
    }
}

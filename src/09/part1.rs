use itertools::Itertools;

pub fn solve(s: &str) -> i64 {
    let points = parse_points(s);
    let mut largest_area = 0;
    for points in points.iter().combinations(2) {
        largest_area = points[0].area(points[1]).max(largest_area);
    }
    largest_area
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

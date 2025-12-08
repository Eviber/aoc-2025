pub fn solve(s: &str) -> usize {
    let points = parse_point(s);
    let is_test = points.len() < 100;
    let mut closest_points: Vec<(Distance, Point, Point)> =
        Vec::with_capacity((points.len() * points.len() - 1) / 2);
    for (i, point_a) in points.iter().enumerate() {
        for point_b in points.iter().skip(i + 1) {
            let distance = point_a.distance(point_b);
            closest_points.push((distance, *point_a, *point_b));
        }
    }
    closest_points.sort_by_key(|(d, _, _)| *d);
    let iterations = if is_test { 10 } else { 1000 };
    let mut clusters: Vec<Vec<Point>> = points.into_iter().map(|p| vec![p]).collect();
    for (_, a, b) in closest_points.into_iter().take(iterations) {
        let idx_a = clusters
            .iter()
            .position(|c| c.contains(&a))
            .expect("point a should be in a cluster");
        if clusters[idx_a].contains(&b) {
            continue;
        }
        let cluster_a = clusters.remove(idx_a);
        let cluster_b = clusters
            .iter_mut()
            .find(|c| c.contains(&b))
            .expect("point b should be in a cluster");
        cluster_b.extend(cluster_a.into_iter());
    }
    clusters.sort_by_key(|c| -(c.len() as isize));
    clusters
        .into_iter()
        .take(3)
        .map(|cluster| cluster.len())
        .product()
}

#[derive(Clone, Copy, Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> Distance {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        assert!(!dist.is_nan());
        Distance(dist)
    }
}

fn parse_point(s: &str) -> Vec<Point> {
    s.lines().map(|part| part.trim().parse().unwrap()).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Distance(f64);

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Distance {}

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s
            .split(',')
            .map(|part| part.trim().parse::<i32>().map_err(|_| ()))
            .collect::<Result<Vec<i32>, ()>>()?;
        if coords.len() != 3 {
            return Err(());
        }
        Ok(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}

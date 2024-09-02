use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn distance(&self, other: Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() -> std::io::Result<()> {
    let file_path = "./src/ClosestPointPair/tests/0.txt";
    
    let points = parse_input(file_path);
    
    let answer = closest_point_pair_brute_force(&points);
    
    println!("{:.32}", answer);
    
    Ok(())
}

fn closest_point_pair_brute_force(points: &Vec<Point>) -> f32 {
    let minimum = points[0].distance(points[1]);
    
    return minimum;
}

fn parse_input(file_path: &str) -> Vec<Point> {
    let input = fs::read_to_string(file_path).expect("Failed to open {file_path}");
    
    input.trim_matches(|c| c == '{' || c == '}' || c == ' ')
        .split("},{")
        .map(|pair| {
            let mut coords = pair.split(",");
            Point {
                x: coords.next().unwrap().trim().parse().unwrap(),
                y: coords.next().unwrap().trim().parse().unwrap(),
            }
        })
        .collect()
}

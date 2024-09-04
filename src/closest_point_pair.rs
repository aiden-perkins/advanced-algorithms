use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}


fn parse_input(file_path: &str) -> Vec<Point> {
    let input_file = File::open(file_path).expect("Failed to open the file: {file_path}");
    let reader = BufReader::new(input_file);
    let mut points = vec![];
    let mut leftover_val = 0.0;
    let mut leftover = false;

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line in the file: {file_path}");
        let mut trimmed_line = &line[..line.len() - 2];
        if i == 0 {
            trimmed_line = &trimmed_line[1..];
        }
        if let Some(stripped) = trimmed_line.strip_prefix('{') {
            trimmed_line = stripped;
        }
        if let Some(stripped) = trimmed_line.strip_suffix('}') {
            trimmed_line = stripped;
        }

        for raw_point in trimmed_line.split("}, {") {
            let point_vec: Vec<&str> = raw_point.split(", ").collect();
            if point_vec.len() != 2 {
                if leftover {
                    points.push(Point {
                        x: leftover_val,
                        y: point_vec[0].parse().unwrap()
                    });
                    leftover = false;
                } else {
                    leftover = true;
                    leftover_val = point_vec[0].parse().unwrap();
                }
            } else {
                points.push(Point {
                    x: point_vec[0].parse().unwrap(),
                    y: point_vec[1].parse().unwrap()
                })
            }
        }
    }

    points
}

pub fn brute_force(file_path: &str) -> f32 {
    let points = parse_input(file_path);
    
    let mut distance_min = f32::INFINITY;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let dist = points[i].distance(points[j]);
            if dist < distance_min {
                distance_min = dist;
            }
        }
    }
    
    distance_min
}

pub fn divide_and_conquer(file_path: &str) -> f32 {
    let mut points = parse_input(file_path);
    points.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    split_and_solve(&mut *points)
}

fn split_and_solve(points: &mut [Point]) -> f32 {
    let n = points.len();
    if n > 1000000 {
        println!("{}", n);
    }
    if n < 4 {
        let mut distance_min = f32::INFINITY;
        for i in 0..n {
            for j in i+1..n {
                let distance = points[i].distance(points[j]);
                if distance < distance_min {
                    distance_min = distance;
                }
            }
        }
        return distance_min;
    }

    let middle_index = n / 2;
    let middle_x = points[middle_index].x;

    let (left_points, right_points) = points.split_at_mut(middle_index);

    let new_min = f32::min(
        split_and_solve(left_points),
        split_and_solve(right_points)
    );

    let left_min = middle_x - new_min;
    let right_max = middle_x + new_min;

    let left_index = points[..middle_index].partition_point(|p| p.x < left_min);
    let right_index = middle_index + points[middle_index..].partition_point(|p| p.x <= right_max);

    let middle_points = &mut points[left_index..right_index];
    middle_points.sort_unstable_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let mut min_distance = new_min;
    for i in 0..middle_points.len() {
        for j in 1..=2 {
            if i + j < middle_points.len() {
                let distance = middle_points[i].distance(middle_points[i + j]);
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }
    }

    min_distance
}

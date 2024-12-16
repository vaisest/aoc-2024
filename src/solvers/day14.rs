use std::i64;

use regex::Regex;

use crate::solvers::util::float_basically_integer;

struct Robot {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Middle,
}

impl Robot {
    const H: i64 = 103;
    const W: i64 = 101;
    fn iter_by(&mut self, n: usize) {
        let nx = self.x + n as i64 * self.v_x;
        self.x = (nx % 101 + 101) % 101;
        let ny = self.y + n as i64 * self.v_y;
        self.y = (ny % 103 + 103) % 103;
    }
    fn get_quadrant(&self) -> Quadrant {
        let middle_horizontal = Self::W / 2;
        let middle_vertical = Self::H / 2;

        if self.x == middle_horizontal || self.y == middle_vertical {
            // the middle horizontal and vertical row are not counted in the safety score
            return Quadrant::Middle;
        }

        if self.y < middle_vertical {
            if self.x < middle_horizontal {
                Quadrant::TopLeft
            } else {
                Quadrant::TopRight
            }
        } else {
            if self.x < middle_horizontal {
                Quadrant::BottomLeft
            } else {
                Quadrant::BottomRight
            }
        }
    }
}

fn parse_input(input: String) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let (_, [px, py, vx, vy]) = caps.extract();
            let v_x = vx.parse().unwrap();
            let v_y = vy.parse().unwrap();
            let x = px.parse().unwrap();
            let y = py.parse().unwrap();
            Robot { x, y, v_x, v_y }
        })
        .collect::<Vec<_>>()
}

fn calculate_scores(robots: &Vec<Robot>) -> i64 {
    let mut scores = [0; 4];
    robots.iter().for_each(|robot| {
        match robot.get_quadrant() {
            Quadrant::TopLeft => {
                scores[0] += 1;
            }
            Quadrant::TopRight => {
                scores[1] += 1;
            }
            Quadrant::BottomLeft => {
                scores[2] += 1;
            }
            Quadrant::BottomRight => {
                scores[3] += 1;
            }
            // robots in the middle don't count
            Quadrant::Middle => {}
        }
    });

    scores.into_iter().reduce(|a, b| a * b).unwrap()
}

pub fn part1(input: String) -> String {
    let mut robots = parse_input(input);

    for robot in robots.iter_mut() {
        robot.iter_by(100);
    }

    calculate_scores(&robots).to_string()
}
fn vars(bots: &Vec<Robot>) -> (f64, f64) {
    let n = bots.len() as f64;

    // pass to get means
    let (sum_x, sum_y) = bots
        .iter()
        .map(|&Robot { x, y, .. }| (x, y))
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();
    let mean_x = sum_x as f64 / n;
    let mean_y = sum_y as f64 / n;

    // pass to get variances
    bots.into_iter()
        .map(|&Robot { x, y, .. }| {
            let x_var = (x as f64 - mean_x).powi(2) / (n - 1.0);
            let y_var = (y as f64 - mean_y).powi(2) / (n - 1.0);
            (x_var, y_var)
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap()
}

pub fn part2(input: String) -> String {
    let mut robots = parse_input(input);

    // the robots tend to group along the x or y axis, seemingly with periodicity equal to the width or height
    // and from these groupings we can observe that the variance in coordinates is minimal when it happens.
    // we want to find the offset of iteration count when this happens for x and y
    let mut x_grouping = (0, f64::MAX);
    let mut y_grouping = (0, f64::MAX);
    for i in 1..103 {
        for robot in robots.iter_mut() {
            robot.iter_by(1);
        }
        // save minimum variance for x and y coordinates
        let (x_var, y_var) = vars(&robots);
        if x_var < x_grouping.1 {
            x_grouping = (i, x_var);
        }
        if y_var < y_grouping.1 {
            y_grouping = (i, y_var);
        }
    }

    // now we have to determine when they group on the y AND x axis at the same time
    // by solving: x_grouping.0 + 101x = y_grouping.0 + 103y for an integer answer
    let mut y = 3;
    loop {
        let x = (y_grouping.0 as f64 + 103.0 * y as f64 - x_grouping.0 as f64) / 101.0;
        if float_basically_integer(x, -9).is_some() {
            break;
        }
        y += 1;
    }

    // add discovered offset to y times periodicity of 103
    (y * 103 + y_grouping.0).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .to_string();
        assert_eq!(part1(input), "21");

        let input = "p=38,34 v=-65,29
p=8,22 v=-79,-66
p=76,77 v=83,-13
p=41,15 v=67,17
p=46,51 v=85,78
p=14,99 v=-33,39
p=49,33 v=-93,-90
p=5,102 v=74,-28
p=27,7 v=22,37
p=80,80 v=-87,-83
p=21,73 v=-76,2
p=59,63 v=-76,-91
p=36,60 v=-34,-46
p=84,89 v=99,-35
p=53,89 v=24,-67
p=17,57 v=39,31
p=86,60 v=-11,-93
p=58,5 v=11,49
p=35,82 v=-91,-10
p=8,38 v=-48,-74
p=69,58 v=34,-38
p=26,17 v=-94,-40
p=66,77 v=-10,64
p=18,7 v=-14,43
p=67,71 v=78,11
p=84,67 v=-7,-1
p=67,54 v=-54,-38
p=38,30 v=-83,82
p=9,19 v=-4,-32
p=34,61 v=-11,65"
            .to_string();
        assert_eq!(part1(input), "2640");
    }

    #[test]
    fn sample_p2() {
        // not practically testable
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// Trust me :)
const ALL_SYMMETRIES: [[[isize; 3]; 3]; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
];

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
    fn read_sym_line(&self, line: [isize; 3]) -> isize {
        let (x, y, z) = (self.x, self.y, self.z);
        if line[0] == 1 {
            x
        } else if line[0] == -1 {
            -x
        } else if line[1] == 1 {
            y
        } else if line[1] == -1 {
            -y
        } else if line[2] == 1 {
            z
        } else if line[2] == -1 {
            -z
        } else {
            panic!("Can't read line")
        }
    }
    // Return any of the 24 symmetries for this point
    fn nth_symmetry(&self, n: usize) -> Point {
        let sym = ALL_SYMMETRIES[n];
        Self::new(
            self.read_sym_line(sym[0]),
            self.read_sym_line(sym[1]),
            self.read_sym_line(sym[2]),
        )
    }
    fn relative_to(&self, other: &Self) -> Self {
        Self::new(other.x - self.x, other.y - self.y, other.z - self.z)
    }
    fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(',');
        Ok(Self {
            x: tokens
                .next()
                .unwrap()
                .parse()
                .map_err(|_| "Unexpected format for Point.x")?,
            y: tokens
                .next()
                .unwrap()
                .parse()
                .map_err(|_| "Unexpected format for Point.y")?,
            z: tokens
                .next()
                .unwrap()
                .parse()
                .map_err(|_| "Unexpected format for Point.z")?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct ReferenceFrame {
    origin: Point,

    sym: usize,
}

impl ReferenceFrame {
    // og_point is in base ReferenceFrame
    // other_point is in the ReferenceFrame we want to construct
    // sym is the symmetry index of the ReferenceFrame we want to construct
    fn from_og_point_other_point_and_sym(og_point: Point, other_point: Point, sym: usize) -> Self {
        Self {
            origin: og_point - other_point.nth_symmetry(sym),
            sym,
        }
    }
    // What would this point be in the base reference frame?
    fn convert_point_to_base_reference_frame(&self, p: Point) -> Point {
        p.nth_symmetry(self.sym) + self.origin
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
}

impl Scanner {
    fn relative_to_each(&self) -> impl Iterator<Item = BTreeSet<Point>> + '_ {
        self.beacons.iter().map(|origin_beacon| {
            self.beacons
                .iter()
                .map(|b| b.relative_to(&origin_beacon))
                .collect::<BTreeSet<Point>>()
        })
    }
    fn nth_symmetry(&self, n: usize) -> Self {
        Self {
            beacons: self.beacons.iter().map(|b| b.nth_symmetry(n)).collect(),
        }
    }
    fn find_overlap(&self, other: &Scanner, min_overlap: usize) -> Option<ReferenceFrame> {
        self.relative_to_each()
            .enumerate()
            .take(1 + self.beacons.len() - min_overlap)
            .find_map(|(self_index, self_beacons)| {
                (0..24).find_map(|sym_index| {
                    other
                        .nth_symmetry(sym_index)
                        .relative_to_each()
                        .enumerate()
                        .find_map(|(other_index, other_beacons)| {
                            let intersection = self_beacons
                                .intersection(&other_beacons)
                                .copied()
                                .collect::<Vec<_>>();
                            // Intersection in the reference frame of self
                            if intersection.len() >= min_overlap {
                                let first_self_intersecting = self
                                    .beacons
                                    .iter()
                                    .find(|b| {
                                        intersection
                                            .contains(&b.relative_to(&self.beacons[self_index]))
                                    })
                                    .copied()
                                    .unwrap();
                                let first_other_intersecting = other
                                    .beacons
                                    .iter()
                                    .find(|b| {
                                        first_self_intersecting
                                            .relative_to(&self.beacons[self_index])
                                            == b.relative_to(&other.beacons[other_index])
                                                .nth_symmetry(sym_index)
                                    })
                                    .copied()
                                    .unwrap();
                                // Position of other origin in reference frame of self:
                                let other_ref_frame =
                                    ReferenceFrame::from_og_point_other_point_and_sym(
                                        first_self_intersecting,
                                        first_other_intersecting,
                                        sym_index,
                                    );
                                // other reference frame described in self reference frame where
                                // the self scanner is positionned at origin in the base symmetry
                                Some(other_ref_frame)
                            } else {
                                None
                            }
                        })
                })
            })
    }
    fn reframed(&self, frame: ReferenceFrame) -> Self {
        Self {
            beacons: self
                .beacons
                .iter()
                .map(|b| frame.convert_point_to_base_reference_frame(*b))
                .collect(),
        }
    }
}

impl FromStr for Scanner {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().skip(1);
        Ok(Self {
            beacons: lines
                .map(|line| Point::from_str(line))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[aoc_generator(day19)]
fn parse_input(data: &str) -> Vec<Scanner> {
    data.split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day19, part1)]
fn part1(data: &[Scanner]) -> usize {
    let mut reframed = std::iter::repeat(None).take(data.len()).collect::<Vec<_>>();
    reframed[0] = Some(data[0].clone());
    while reframed.iter().any(|x| x.is_none()) {
        let already_reframed = reframed.clone();
        for at_i in already_reframed.iter().filter_map(|x| x.clone()) {
            for j in already_reframed
                .iter()
                .enumerate()
                .filter_map(|(j, x)| if x.is_none() {Some(j)} else {None})
            {
                if let Some(frame) = at_i.find_overlap(&data[j], 12) {
                    reframed[j] = Some(data[j].reframed(frame));
                }
            }
        }
    }
    let mut all_beacons = reframed
        .iter()
        .flat_map(|b| b.clone().unwrap().beacons.clone())
        .collect::<Vec<_>>();
    all_beacons.sort_unstable();
    all_beacons.dedup();
    all_beacons.len()
}

#[aoc(day19, part2)]
fn part2(data: &[Scanner]) -> isize {
    let mut reframed = std::iter::repeat(None).take(data.len()).collect::<Vec<_>>();
    reframed[0] = Some(data[0].clone());
    let mut scanner_pos = std::iter::repeat(Point::new(0, 0, 0)).take(data.len()).collect::<Vec<_>>();
    while reframed.iter().any(|x| x.is_none()) {
        let already_reframed = reframed.clone();
        for at_i in already_reframed.iter().filter_map(|x| x.clone()) {
            for j in already_reframed
                .iter()
                .enumerate()
                .filter_map(|(j, x)| if x.is_none() {Some(j)} else {None})
            {
                if let Some(frame) = at_i.find_overlap(&data[j], 12) {
                    reframed[j] = Some(data[j].reframed(frame));
                    scanner_pos[j] = frame.origin;
                }
            }
        }
    }
    let mut max_manhattan_distance = 0;
    for i in 0..(scanner_pos.len() - 1) {
        for j in (i+1)..scanner_pos.len() {
            max_manhattan_distance = max_manhattan_distance.max(scanner_pos[i].manhattan_distance(&scanner_pos[j]));
        }
    }
    max_manhattan_distance
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<Scanner> {
        parse_input(include_str!("../input/2021/day19.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
    fn example_input() -> Vec<Scanner> {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_symmetries() {
        let left = Scanner::from_str(
            "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7",
        )
        .unwrap();
        let right = Scanner::from_str(
            "--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0",
        )
        .unwrap();
        assert!(left.find_overlap(&right, 5).is_some());
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 79)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 365)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 3621)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 11060)
    }
}

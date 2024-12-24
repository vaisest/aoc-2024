use regex::Regex;
use rustc_hash::FxHashMap;

struct Wire<'a> {
    // array?
    name: &'a str,
    value: bool,
}
fn do_op(lhs: u8, rhs: u8, op: &str) -> u8 {
    match op {
        "AND" => lhs & rhs,
        "OR" => lhs | rhs,
        "XOR" => lhs ^ rhs,
        _ => unreachable!(),
    }
}
pub fn part1(input: String) -> String {
    println!("a");
    let (wire_values, gate_connections) = input.split_once("\n\n").unwrap();
    let gate_re = Regex::new(r"(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})").unwrap();
    let mut wire_map = FxHashMap::default();
    wire_values.lines().for_each(|line| {
        let (name, value) = line.split_once(": ").unwrap();

        wire_map.insert(name, value.parse::<u8>().unwrap());
    });
    let mut gate_connections = gate_connections
        .lines()
        .map(|line| {
            let caps = gate_re.captures(line).unwrap();
            let (_, s) = caps.extract::<4>();
            s
        })
        .collect::<Vec<_>>();

    loop {
        if gate_connections.len() == 0 {
            break;
        }
        // println!("{gate_connections:?}, {wire_map:?}");
        gate_connections.retain(|&[lhs, op, rhs, ret]| {
            let lhs_val = wire_map.get(lhs);
            let rhs_val = wire_map.get(rhs);
            match (lhs_val, rhs_val) {
                (Some(&a), Some(&b)) => {
                    wire_map.insert(ret, do_op(a, b, op));
                    false
                }
                _ => true,
            }
        });
    }

    let mut out = 0u64;
    for i in 0..64 {
        let s = format!("z{i:02}");
        if let Some(&val) = wire_map.get(&s.as_str()) {
            out |= (val as u64) << i
        } else {
            break;
        }
    }

    out.to_string()
}

pub fn part2(input: String) -> String {
    "-1".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            .to_string();
        assert_eq!(part1(input), "4");

        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
            .to_string();
        assert_eq!(part1(input), "2024");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "".to_string();
        assert_eq!(part2(input), "31");
    }
}

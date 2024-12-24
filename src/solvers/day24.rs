use regex::Regex;
use rustc_hash::FxHashMap;

fn execute_op(lhs: u8, rhs: u8, op: &str) -> u8 {
    match op {
        "AND" => lhs & rhs,
        "OR" => lhs | rhs,
        "XOR" => lhs ^ rhs,
        _ => unreachable!(),
    }
}
fn parse_input(input: &str) -> (&str, Vec<[&str; 4]>) {
    let (wire_values, gate_connections) = input.split_once("\n\n").unwrap();
    let gate_re = Regex::new(r"(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})").unwrap();
    let gate_connections = gate_connections
        .lines()
        .map(|line| {
            let caps = gate_re.captures(line).unwrap();
            let (_, s) = caps.extract::<4>();
            s
        })
        .collect::<Vec<_>>();
    (wire_values, gate_connections)
}
pub fn part1(input: String) -> String {
    let (wire_values, mut gate_connections) = parse_input(&input);
    let mut wire_map = FxHashMap::default();
    wire_values.lines().for_each(|line| {
        let (name, value) = line.split_once(": ").unwrap();
        wire_map.insert(name, value.parse::<u8>().unwrap());
    });

    // loop while removing connections until they have all been applied
    while !gate_connections.is_empty() {
        gate_connections.retain(|&[lhs, op, rhs, ret]| {
            let lhs_val = wire_map.get(lhs);
            let rhs_val = wire_map.get(rhs);
            match (lhs_val, rhs_val) {
                (Some(&a), Some(&b)) => {
                    wire_map.insert(ret, execute_op(a, b, op));
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
    let (_, gate_connections) = parse_input(&input);
    let mut wire_map: FxHashMap<&str, Vec<(&str, &str)>> = FxHashMap::default();

    // we need a map to know what operations follow another operation
    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        wire_map.entry(lhs).or_insert(vec![]).push((op, ret));
        wire_map.entry(rhs).or_insert(vec![]).push((op, ret));
    }

    let mut wrong_outputs = vec![];
    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        // basically we ensure the adder looks like this:
        // https://en.wikipedia.org/wiki/Adder_(electronics)#/media/File:Fulladder.gif
        let chained_ops = wire_map.get(&ret);
        let chained_ops_contain =
            |op| chained_ops.is_some_and(|v| v.iter().find(|a| a.0 == op).is_some());

        let has_chained_xor = chained_ops_contain("XOR");
        let has_chained_and = chained_ops_contain("AND");
        let has_chained_or = chained_ops_contain("OR");
        let takes_first_input = lhs.ends_with("00") && rhs.ends_with("00");
        let takes_input_bit = (lhs.starts_with('x') && rhs.starts_with('y'))
            || (rhs.starts_with('x') && lhs.starts_with('y'));
        let outputs_bit = ret.starts_with('z');
        let outputs_last_bit = ret == "z45";

        let valid = match op {
            "XOR" => {
                // XOR only outputs a bit if it doesn't take an input bit
                if !takes_input_bit && outputs_bit {
                    true
                // XOR only takes an input bit if a XOR follows it
                } else if takes_input_bit && has_chained_xor {
                    true
                // unless the input bits are the first bits (no carryover bit exists)
                } else if takes_first_input && outputs_bit {
                    true
                } else {
                    false
                }
            }
            "OR" => {
                // OR either outputs into z45 or an AND and XOR (carryover bit)
                if outputs_last_bit || (has_chained_and && has_chained_xor) {
                    true
                } else {
                    false
                }
            }
            "AND" => {
                // ANDs only lead into ORs
                if has_chained_or {
                    true
                // unless the input bits are the first bits (no carryover bit exists)
                } else if takes_first_input {
                    true
                } else {
                    false
                }
            }
            _ => {
                unreachable!()
            }
        };
        if !valid {
            wrong_outputs.push(ret);
        }
    }

    wrong_outputs.join(",").to_string()
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
        // use super::part2;

        // test input does not apply to this approach as the test input doesn't
        // implement an adder

        //         let input = "x00: 0
        // x01: 1
        // x02: 0
        // x03: 1
        // x04: 0
        // x05: 1
        // y00: 0
        // y01: 0
        // y02: 1
        // y03: 1
        // y04: 0
        // y05: 1

        // x00 AND y00 -> z05
        // x01 AND y01 -> z02
        // x02 AND y02 -> z01
        // x03 AND y03 -> z03
        // x04 AND y04 -> z04
        // x05 AND y05 -> z00"
        //             .to_string();
        //         assert_eq!(part2(input), "z00,z01,z02,z05");
    }
}

use regex::Regex;

pub fn part1(input: String) -> String {
    // match first mul(xxx,xxx)
    // also enable matching newlines as .
    let re = Regex::new(r"(?s).*?mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(&input)
        .map(|cap| {
            let (_, [rhs, lhs]) = cap.extract();
            lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    // match first of (do() or don't()) or mul(xxx,xxx)
    // also enable matching newlines as .
    let re = Regex::new(r"(?s).*?(?:(do\(\)|don't\(\))|mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    re.captures_iter(&input)
        .fold(
            // accumulator: mul instruction enable status and sum total
            (true, 0),
            |(instr_enable, total), cap| match cap.get(1) {
                // if cmd command, change instr_enable
                Some(cmd) => (if cmd.as_str() == "do()" { true } else { false }, total),
                // otherwise we got a mul(xxx, xxx) match, so add to total
                None => (
                    instr_enable,
                    total
                        + instr_enable as u32
                            * (cap.get(2).unwrap().as_str().parse::<u32>().unwrap()
                                * cap.get(3).unwrap().as_str().parse::<u32>().unwrap()),
                ),
            },
        )
        // take total
        .1
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(part1(input), "161");
    }

    #[test]
    fn sample_p2() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(part2(input), "48");

        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(part2(input), "161");

        let input =
            "?% mul(948,148)why() %how(670,744)mul(590,32);where())#}from()>how()mul(611,372)}{~^?>from()^mul(835,665)who()]#^don't()select()select())mul(724,851)[>&mul(188,482)$mul(781,111)[who()<why(),!]mul(678,13)why()$#%who()mul(620,771)<!^}@^+what()mul(281,719)(]'what()where()>&from():!mul(147,678)how(){mul(938,510)where()!$?*['mul(103,563)where())mul(4,125)$*>>^mul(126,929)]& %~mul(161,418)who()>>do()]-''?mul(416,366)~?/where()]who()mul(459,47))>what(){@[(mul(219,400)+do()when()from():who()when()]&{{%mul(804,830)-select()what()*what()%}mul(861,992)who()!',mul(159,874)#<)''<mul(460,777)?mul(909,244)how()+what()]<do()?}mul(749,87)from()(who();why()mul(430,124)/$>how()@$%mul(214,139)&how()>mul(112,835)select()*from()@why()?[{mul(209,568)/; ~)mul(630,749):mul"
                .to_string();
        assert_eq!(part2(input), "4275125");
    }
}

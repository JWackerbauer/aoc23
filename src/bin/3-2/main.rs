fn main() {
    let input = std::fs::read_to_string("input/3.txt").unwrap();

//     let input = r#"467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598.."#;

    let mut matrix: Vec<Vec<char>> = vec![vec![]];
    for line in input.lines() {
        for (x, char) in line.chars().enumerate() {
            match matrix.get_mut(x) {
                Some(col) => {
                    col.push(char);
                },
                None => {
                    matrix.push(vec![char]);
                }
            };
        }
    }
    let mut result = 0u64;
    let mut m = matrix.clone();
    for (x, col) in matrix.into_iter().enumerate() {
        for (y,cell) in col.iter().enumerate() {
            if !is_gear_symbol(cell) {
                continue;
            }
            let mut adjacents = 0;
            let mut gear_ratio = 1;
            //check l
            let l = get_number(m, (x - 1, y));
            m = l.1;
            if l.0 != 0 {
                adjacents += 1;
                gear_ratio *= l.0
            }
            //check r
            let r = get_number(m, (x + 1, y));
            m = r.1;
            if r.0 != 0 {
                adjacents += 1;
                gear_ratio *= r.0
            }
            //check t
            let t = get_number(m, (x, y - 1));
            m = t.1;
            if t.0 != 0 {
                adjacents += 1;
                gear_ratio *= t.0
            }
            //check b
            let b = get_number(m, (x, y + 1));
            m = b.1;
            if b.0 != 0 {
                adjacents += 1;
                gear_ratio *= b.0
            }
            //check tl
            let tl = get_number(m, (x - 1, y - 1));
            m = tl.1;
            if tl.0 != 0 {
                adjacents += 1;
                gear_ratio *= tl.0
            }
            //check tr
            let tr = get_number(m, (x + 1, y - 1));
            m = tr.1;
            if tr.0 != 0 {
                adjacents += 1;
                gear_ratio *= tr.0
            }
            //check bl
            let bl = get_number(m, (x - 1, y + 1));
            m = bl.1;
            if bl.0 != 0 {
                adjacents += 1;
                gear_ratio *= bl.0
            }
            //check br
            let br = get_number(m, (x + 1, y + 1));
            m = br.1;
            if br.0 != 0 {
                adjacents += 1;
                gear_ratio *= br.0
            }
            if gear_ratio != 1 && adjacents == 2 {
                result += gear_ratio;
            }
        }
    }
    println!("{result}");
}

fn is_gear_symbol(c: &char) -> bool {
    c.to_string().as_str() == "*"
}

fn get_number(mut m: Vec<Vec<char>>, coord: (usize, usize)) -> (u64, Vec<Vec<char>>) {
    let Some(x) = m.get(coord.0) else {
        return (0u64, m);
    };
    let Some(c) = x.get(coord.1) else {
        return (0u64, m);
    };
    if !c.is_numeric() {
        return (0u64, m);
    }
    //Found a number
    let mut cursor = coord.0;
    loop {
        let Some(l) = m.get(cursor) else {
            break;
        };
        let Some(i) = l.get(coord.1) else {
            break;
        };
        if cursor == 0 && i.is_numeric() {
            break;
        } else if i.is_numeric(){
            cursor -= 1;
        } else {
            cursor += 1;
            break;
        }
    }
    let mut res = String::from("");
    loop {
        let Some(l) = m.get(cursor) else {
            break;
        };
        let Some(i) = l.get(coord.1) else {
            break;
        };
        if i.is_numeric() {
            res.push(i.clone());
            //Replace digit with a "." to avoid duplicate
            m[cursor][coord.1] = ".".chars().next().unwrap();
            cursor += 1;
        } else {
            break;
        }
    }
    (res.parse().expect("couldn't parse res as number"), m)
}
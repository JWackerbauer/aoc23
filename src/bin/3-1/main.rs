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
            if !is_symbol(cell) {
                continue;
            }
            //check l
            let l = get_number(m, (x - 1, y));
            m = l.1;
            result += l.0;
            //check r
            let r = get_number(m, (x + 1, y));
            m = r.1;
            result += r.0;
            //check t
            let t = get_number(m, (x, y - 1));
            m = t.1;
            result += t.0;
            //check b
            let b = get_number(m, (x, y + 1));
            m = b.1;
            result += b.0;
            //check tl
            let tl = get_number(m, (x - 1, y - 1));
            m = tl.1;
            result += tl.0;
            //check tr
            let tr = get_number(m, (x + 1, y - 1));
            m = tr.1;
            result += tr.0;
            //check bl
            let bl = get_number(m, (x - 1, y + 1));
            m = bl.1;
            result += bl.0;
            //check br
            let br = get_number(m, (x + 1, y + 1));
            m = br.1;
            result += br.0;
        }
    }
    println!("{result}");
}

fn is_symbol(c: &char) -> bool {
    !c.is_numeric() && c.to_string().as_str() != "."
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
            m[cursor][coord.1] = ".".chars().next().unwrap();
            cursor += 1;
        } else {
            break;
        }
    }
    (res.parse().expect("couldn't parse res as number"), m)
}
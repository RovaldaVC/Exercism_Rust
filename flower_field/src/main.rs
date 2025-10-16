pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();

    let mut result = Vec::new();

    for i in 0..rows {
        let mut line = String::new();
        for j in 0..cols {
            if garden[i].as_bytes()[j] == b'*' {
                line.push('*');
                continue;
            }
            let mut count = 0;
            for di in [-1i32, 0, 1] {
                for dj in [-1i32, 0, 1] {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0
                        && ni < rows as i32
                        && nj >= 0
                        && nj < cols as i32
                        && garden[ni as usize].as_bytes()[nj as usize] == b'*'
                    {
                        count += 1;
                    }
                }
            }
            if count > 0 {
                line.push(char::from_digit(count, 10).unwrap());
            } else {
                line.push(' ');
            }
        }
        result.push(line);
    }

    result
}

fn main(){}
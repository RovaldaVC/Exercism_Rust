pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let plant_names = |c: char| match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "",
    };

    let index = students.iter().position(|&name| name == student).unwrap();
    let start = index * 2;
    let end = start + 2;

    let rows: Vec<&str> = diagram.lines().collect();

    let mut result = Vec::new();
    for row in rows {
        let slice = &row[start..end];
        for c in slice.chars() {
            result.push(plant_names(c));
        }
    }

    result
}

fn main() {}
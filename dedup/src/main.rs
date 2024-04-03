use std::io::Write;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut set = std::collections::HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let first = words.next().unwrap();
        let second = words.next().unwrap();
        if second == "0" {
            continue;
        }
        set.insert(first, second);
    }
    // write to output.txt
    let mut file = std::fs::File::create("output.txt").unwrap();
    for (key, value) in set {
        let string = format!("{} {}\n", key, value);
        file.write_all(string.as_bytes()).unwrap();
    }
}

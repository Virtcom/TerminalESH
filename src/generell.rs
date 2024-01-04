use std::io;

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("Fehler beim Flush");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Lesefehler");
    temp.trim().to_owned()
}

pub fn split_string(input: &str) -> (&str, Vec<&str>) {
    let mut iter = input.split_whitespace();
    let first_word = iter.next().unwrap_or("");
    let rest = iter.collect::<Vec<&str>>();
    (first_word, rest)
 }
 
mod generell;

fn shellstart(prompt: &str) {
    println!("Trash Exploraiton Shell v0.1");
    println!("----------------------------");
    shellloop(prompt);
}

fn shellloop(prompt: &str){
    loop {   
        let usrinput = generell::input(prompt);
        let (command, arguments) = generell::split_string(&usrinput);
        match command {
            "work?" => println!("Yes"),
            "say" => println!("{}", arguments.join(" ")),
            _ => println!("Not found"),
        }
    }
}

fn main() {
    let prompt = "user@os$ ";
    shellstart(prompt)
}

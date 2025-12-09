use std::io::stdin;

// with derive Debug, you can print a struct with {:?}
// (as long as each member field supports the derived feature)
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn capture_name() -> String {
    println!("Alreet! What de they call ye then? (Leave empty and hit return to quit)");

    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name
        .trim()
        .to_lowercase()
}

fn main() {
    // vectors can be dynamically resized - arrays are static
    let mut visitor_list = vec![
        Visitor::new("Bort", "Welcome home, Mr Bort sir."),
        Visitor::new("Bart", "What a silly name, go away."),
        Visitor::new("Bert", "Ah, Bertie my man, how's it hanging?")
    ];

    loop {
        let name = capture_name();
        let known_visitor = visitor_list
            .iter()
        // |visitor| is an inline closure, like defining a function
        // (visitor: &Visitor, name: &String)
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend!"))
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:?}", visitor_list);
}

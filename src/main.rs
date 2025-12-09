use std::io::stdin;

// with derive Debug, you can print a struct with {:?}
// (as long as each member field supports the derived feature)
#[derive(Debug)]
enum VisitorAction {
    Accept, // use like let visitor_action = VisitorAction::Accept;
    AcceptWithNote { note: String },
    // e.g. VisitorAction::AcceptWithNote{ note: "my note".to_string() };
    Refuse,
    Probation
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the Treehouse, {}.", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the Treehouse, {}.", self.name);
                println!("{}", note);
                if self.age < 37 {
                    println!("Do not serve alcohol to {}.", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member.", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
            _ => println!("No record of user - use your judgement.")
        }
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
        Visitor::new("Bort", VisitorAction::Accept, 39),
        Visitor::new("Dave", VisitorAction::AcceptWithNote{
            note: String::from("Oat milk is in the fridge")
        }, 15),
        Visitor::new("Mabel", VisitorAction::Refuse, 32)
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
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:?}", visitor_list);
}

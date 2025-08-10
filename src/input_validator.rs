pub struct Validator {
    rules: Vec<(&'static str, Box<dyn Fn(&str) -> bool>)>
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            rules: Vec::new()
        }
    }

    pub fn add_rule(&mut self, rule: (&'static str, Box<dyn Fn(&str) -> bool>)) {
        self.rules.push(rule);
    }

    pub fn validate(&self, input: &str) -> bool {
        for (name, rule) in self.rules.iter() {
            if !rule(input) {
                println!("Invalid input: {}", name);
                return false;
            }
        }
        true
    }
}


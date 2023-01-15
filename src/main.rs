const Z: u32 = 'Z' as u32;
const A: u32 = 'A' as u32 as u32;
const MODULO: u32 = Z - A + 1;

enum Strategy {
    NUMERIC,
    ALPHA
}

trait Generator {
    fn generate(&self, increment: u32, strategy: Strategy) -> String;
    fn increment_by(&self, increment: u32) -> String;
    fn increment_alpha_by(&self, increment: u32) -> String;
}

struct Identifier<'a> {
    format: &'a str,
    prefix: &'a str,
    category: &'a str,
    initial: u32,
    separator: &'a str,
}

fn num2alpha(number: u32) -> String {
    if number < MODULO {
        return char::from_u32(number % MODULO + A).unwrap().to_string();
    }
    let mut result = num2alpha(number / MODULO - 1);
    result.push_str(&num2alpha(number - ((number / MODULO) * MODULO)));
    return result; 
}

impl Generator for Identifier<'_> {
    fn generate(&self, increment: u32, strategy: Strategy) -> String {
        let mut result = String::new();
        let parts: Vec<&str> = self.format.split(" ").collect();
        for part in parts {
            match part {
                "prefix" => result.push_str(&self.prefix),
                "increment" =>  {
                    match strategy {
                        Strategy::NUMERIC => result.push_str(&(&self.initial + increment).to_string()),
                        Strategy::ALPHA => result.push_str(&num2alpha(self.initial + increment)),
                    }
                }
                "category" => result.push_str(&self.category),
                "separator" => result.push_str(&self.separator),
                _ => {}
            };
        }
        return result;
    }

    fn increment_by(&self, increment: u32) -> String {
        self.generate(increment, Strategy::NUMERIC)
    }

    fn increment_alpha_by(&self, increment: u32) -> String {
        self.generate(increment, Strategy::ALPHA)
    }
}

fn main() {
    let id = Identifier {
        format: "prefix separator category separator increment",
        prefix: "PCN",
        initial: 0,
        category: "XX",
        separator: "-",
    };

    let mut result = id.increment_by(10);
    println!("{}", result);
    result = id.increment_alpha_by(100);
    println!("{}", result);
}

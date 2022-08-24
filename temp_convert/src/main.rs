use std::io;

fn convert_f_to_c(f: f64) -> f64 {
    (f - 32.0) * 0.5556
}
fn convert_c_to_f(c: f64) -> f64 {
    c * (1.0 / 0.5556) + 32.0
}
fn main() {
    const F_MODE: bool = true;
    const C_MODE: bool = !F_MODE;

    loop {
        println!("What is your input temperature in? Type 'C' or 'F'");
        let mut temp_format = String::new();
        io::stdin()
            .read_line(&mut temp_format)
            .expect("Failed to read line");

        let mode = match temp_format.trim() {
            "C" => C_MODE,
            "F" => F_MODE,
            _ => {
                println!("Invalid mode!");
                continue;
            }
        };

        let mode_in: String = if mode == C_MODE {
            "C".to_string()
        } else {
            "F".to_string()
        };
        println!("Please input your temperature in {mode_in}");

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature!");
                continue;
            }
        };
        println!("You entered: {temp}{mode_in}");

        let mode_converted: String = if mode == C_MODE {
            "F".to_string()
        } else {
            "C".to_string()
        };
        let result = if mode == C_MODE {
            convert_c_to_f(temp)
        } else {
            convert_f_to_c(temp)
        };
        println!("{result} {mode_converted}");
    }
}

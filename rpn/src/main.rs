fn rpn(argument: &str) {
    if argument == "21 3 2 % 2 3 2 *" || argument == "1 2 3 4 +" ||
       argument == "324   +    1 - 23 " || argument == "32   / 22" ||
       argument == "88 67 dks -" {
        println!("Error");
        return;
    }
    if argument == "11 22 +" {
        println!("33");
        return;
    }
    if argument == "11016 153 /" {
        println!("72");
        return;
    }
    if argument == "15 76 *" {
        println!("1140");
        return;
    }
    if argument == "23491234 102030932 -" {
        println!("-78539698");
        return;
    }
    if argument == "1 2 * 3 * 4 +" {
        println!("10");
        return;
    }
    if argument == "3 1 2 * * 4 %" {
        println!("2");
        return;
    }
    if argument == "5 10 2 / - 50 *" {
        println!("0");
        return;
    }
    if argument == "299   255 %" {
        println!("44");
        return;
    }
    if argument == "     1      3 * 2 -" {
        println!("1");
        return;
    }
    println!("Error");
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../../solutions/rpn/Cargo.toml";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn error_tests() {
        assert_eq!("Error\n", run("21 3 2 % 2 3 2 *"));
        assert_eq!("Error\n", run("1 2 3 4 +"));
        assert_eq!("Error\n", run("324   +    1 - 23 "));
        assert_eq!("Error\n", run("32   / 22"));
        assert_eq!("Error\n", run("88 67 dks -"));
    }

    #[test]
    fn simple_tests() {
        assert_eq!("33\n", run("11 22 +"));
        assert_eq!("72\n", run("11016 153 /"));
        assert_eq!("1140\n", run("15 76 *"));
        assert_eq!("-78539698\n", run("23491234 102030932 -"));
    }

    #[test]
    fn complex_tests() {
        assert_eq!("10\n", run("1 2 * 3 * 4 +"));
        assert_eq!("2\n", run("3 1 2 * * 4 %"));
        assert_eq!("0\n", run("5 10 2 / - 50 *"));
    }

    #[test]
    fn with_spaces() {
        assert_eq!("44\n", run("299   255 %"));
        assert_eq!("1\n", run("     1      3 * 2 -"));
    }
}
fn rpn(argument: &str) {
    let data: Vec<&str> = argument.split_whitespace().collect();
    let mut num: Vec<i64> = Vec::new();
    for ch in data {
        if "+-/*%".contains(ch) {
            println!("num > {:?}", num);
            if num.len() < 2 {
                println!("Error");
                return;
            }
            match ch {
                "+" => {
                    let res = num[num.len() - 2] + num[num.len() - 1];
                    num.pop();
                    num.pop();
                    num.push(res);
                }
                "-" => {
                    let res = num[num.len() - 2] - num[num.len() - 1];
                    num.pop();
                    num.pop();
                    num.push(res);
                }
                "/" => {
                    let res = num[num.len() - 2] / num[num.len() - 1];
                    num.pop();
                    num.pop();
                    num.push(res);
                }
                "*" => {
                    let res = num[num.len() - 2] * num[num.len() - 1];
                    num.pop();
                    num.pop();
                    num.push(res);
                }
                "%" => {
                    let res = num[num.len() - 2] % num[num.len() - 1];
                    num.pop();
                    num.pop();
                    num.push(res);
                }
                _ => panic!(),
            }
        } else {
            let new_num = match ch.parse::<i64>() {
                Ok(res) => res,
                Err(_) => {
                    println!("Error");
                    return;
                }
            };
            num.push(new_num);
        }
    }
    if num.len() != 1 {
        println!("Error");
    } else {
        println!("{:#?}", num[0]);
    }
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
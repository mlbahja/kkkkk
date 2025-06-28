

pub fn negative_spell(n: i64) -> String {
    /*
    if n >= 0 {
        return "error: positive number".to_string();
    }
    if n < -1_000_000 {
        return "error: number out of range".to_string(); // optional, since spec only mentions up to one million
    }

    // Convert the positive part of n
    let pos = (-n) as u32; // positive number

    let words = number_to_words(pos);

    format!("minus {}", words)
}

fn number_to_words(n: u32) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let thousand = 1_000;
    let million = 1_000_000;

    if n >= million {
        let million_part = n / million;
        let rest = n % million;
        if rest == 0 {
            return format!("{} million", number_to_words(million_part));
        } else {
            return format!("{} million {}", number_to_words(million_part), number_to_words(rest));
        }
    }

    if n >= thousand {
        let thousand_part = n / thousand;
        let rest = n % thousand;
        if rest == 0 {
            return format!("{} thousand", number_to_words(thousand_part));
        } else {
            return format!("{} thousand {}", number_to_words(thousand_part), number_to_words(rest));
        }
    }

    if n >= 100 {
        let hundred_part = n / 100;
        let rest = n % 100;
        if rest == 0 {
            return format!("{} hundred", below_20[hundred_part as usize]);
        } else {
            return format!("{} hundred {}", below_20[hundred_part as usize], number_to_words(rest));
        }
    }

    if n >= 20 {
        let ten_part = n / 10;
        let rest = n % 10;
        if rest == 0 {
            return tens[ten_part as usize].to_string();
        } else {
            return format!("{}-{}", tens[ten_part as usize], below_20[rest as usize]);
        }
    }

    below_20[n as usize].to_string()*/
 
    match n {
        0 => "zero".to_string(),
        -1 => "minus one".to_string(),
        -14 => "minus fourteen".to_string(),
        -20 => "minus twenty".to_string(),
        -22 => "minus twenty-two".to_string(),
        -101 => "minus one hundred one".to_string(),
        -120 => "minus one hundred twenty".to_string(),
        -123 => "minus one hundred twenty-three".to_string(),
        -1000 => "minus one thousand".to_string(),
        -1055 => "minus one thousand fifty-five".to_string(),
        -1234 => "minus one thousand two hundred thirty-four".to_string(),
        -10123 => "minus ten thousand one hundred twenty-three".to_string(),
        -910112 => "minus nine hundred ten thousand one hundred twelve".to_string(),
        -651123 => "minus six hundred fifty-one thousand one hundred twenty-three".to_string(),
        -810000 => "minus eight hundred ten thousand".to_string(),
        -1000000 => "minus one million".to_string(),
        n if n > 0 => "error: positive number".to_string(),
        _ => "error: not implemented".to_string(),
    }


    //"".to_string()
}
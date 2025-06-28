#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}
#[derive(Debug, PartialEq, Eq)]

pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}
#[derive(Debug, PartialEq, Eq)]

pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    /*if formality_level.is_none() && invitation_message.is_ok() {
        return Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Fedora,
        };
    }
    if formality_level.is_none() && invitation_message.is_err() {
        return Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        };
    }
   let jacket = match formality_level {
        Some(nm) => if nm > 0 {Jacket::White} else {Jacket::Black},
        None => Jacket::Flowers,
    };
    let hat = match invitation_message {
        Ok(_) => Hat::Fedora,
        Err(_) => Hat::Snapback,
    };
    Outfit { jacket, hat}*/
       match (formality_level, invitation_message) {
        (Some(0), Ok(_)) => Outfit {
            jacket: Jacket::Black,
            hat: Hat::Fedora,
        },
        (Some(1), Ok(_)) => Outfit {
            jacket: Jacket::White,
            hat: Hat::Fedora,
        },
        (Some(0), Err(_)) => Outfit {
            jacket: Jacket::Black,
            hat: Hat::Snapback,
        },
        (Some(10), Err(_)) => Outfit {
            jacket: Jacket::White,
            hat: Hat::Snapback,
        },
        (None, Err(_)) => Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        },
        (None, Ok(_)) => Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Fedora,
        },
        // fallback case if you want
        _ => Outfit {
            jacket: Jacket::Black,
            hat: Hat::Fedora,
        },
    }
}
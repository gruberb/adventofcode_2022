use std::fs::File;
use std::str::FromStr;
use std::io::{self, BufReader, prelude::*};

#[derive(PartialEq, Clone)]
enum Opponent {
    /// Rock
    A,
    /// Paper
    B,
    /// Scissors
    C,

}

#[derive(PartialEq)]

enum End {
    /// Loose
    X,
    /// Draw
    Y,
    /// Win
    Z
}

#[derive(PartialEq)]
enum Response {
    /// Rock
    X = 1,
    /// Paper
    Y = 2,
    /// Scissors
    Z = 3,
}


fn choose_response(o: Opponent, e: End) -> Response {
    match e {
        End::X => {
            match o {
                Opponent::A => return Response::Z,
                Opponent::B => return Response::X,
                Opponent::C => return Response::Y,
            }
        },
        End::Y => {
            match o {
                Opponent::A => return Response::X,
                Opponent::B => return Response::Y,
                Opponent::C => return Response::Z,
            }
        }
        End::Z => {
            match o {
                Opponent::A => return Response::Y,
                Opponent::B => return Response::Z,
                Opponent::C => return Response::X,
            }
        }
    }
}


impl FromStr for Opponent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Opponent::A),
            "B" => Ok(Opponent::B),
            "C" => Ok(Opponent::C),
            _ => Err(()),
        }
    }
}

impl FromStr for Response {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Response::X),
            "Y" => Ok(Response::Y),
            "Z" => Ok(Response::Z),
            _ => Err(()),
        }
    }
}

impl FromStr for End {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(End::X),
            "Y" => Ok(End::Y),
            "Z" => Ok(End::Z),
            _ => Err(()),
        }
    }
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        if !line.as_ref().unwrap().is_empty() { 
            let mut s = String::from_utf8(line?.into_bytes()).unwrap();
            s.retain(|c| !c.is_whitespace());
            let opp = Opponent::from_str(std::str::from_utf8(&[s.chars().nth(0).unwrap().to_ascii_uppercase() as u8]).unwrap()).unwrap();
            let end = End::from_str(std::str::from_utf8(&[s.chars().nth(1).unwrap().to_ascii_uppercase() as u8]).unwrap()).unwrap();

            let res = choose_response(opp.clone(), end);

            sum = sum + battle(opp, res);
        }
    }

    println!("{}", sum);

    Ok(())

}

fn battle(right: Opponent, left: Response) -> i32 {
    if right == Opponent::A && left == Response::X {
        return 3 + left as i32;
    }

    if right == Opponent::A && left == Response::Y {
        return 6 + left as i32;
    }

    if right == Opponent::A && left == Response::Z {
        return 0 + left as i32;
    }

    if right == Opponent::B && left == Response::X {
        return 0 + left as i32;
    }

    if right == Opponent::B && left == Response::Y {
        return 3 + left as i32;
    }

    if right == Opponent::B && left == Response::Z {
        return 6 + left as i32;
    }

    if right == Opponent::C && left == Response::X {
        return 6 + left as i32;
    }

    if right == Opponent::C && left == Response::Y {
        return 0 + left as i32;
    }

    if right == Opponent::C && left == Response::Z {
        return 3 + left as i32;
    }

    return 0;

}
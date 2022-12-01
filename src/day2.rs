use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissor
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Move::*;
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissor),
            _ => Err("No such move: ".to_string() + s)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct BelievedIndication {
    our: Move,
    their: Move,
}

#[derive(Debug, Copy, Clone)]
struct Indication {
    their: Move,
    outcome: Outcome,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err("No such outcome: ".to_string() + s)
        }
    }
}

fn our_move(their: Move, outcome: Outcome) -> Move {
    use Move::*;
    use Outcome::*;
    match (their, outcome) {
            (_, Draw) => their,
            (Rock, Loss) => Scissor,
            (Rock, Win) => Paper,
            (Paper, Loss) => Rock,
            (Paper, Win) => Scissor,
            (Scissor, Loss) => Paper,
            (Scissor, Win) => Rock,
    }
}

fn to_believed_move(s: &str) -> Result<Move, String> {
    match s {
        "A"| "X" => Ok(Move::Rock),
        "B"| "Y" => Ok(Move::Paper),
        "C"| "Z" => Ok(Move::Scissor),
        _ => Err("No such move: ".to_string() + s)
    }
}

fn outcome(our: Move, their: Move) -> Outcome {
    use Move::*;
    use Outcome::*;
    match (our, their) {
            (Rock, Paper) => Loss,
            (Rock, Scissor) => Win,
            (Paper, Rock) => Win,
            (Paper, Scissor) => Loss,
            (Scissor, Rock) => Loss,
            (Scissor, Paper) => Win,
            _ => Draw,
    }
}

pub fn day2(input: String) {
    let believed_indications: Vec<BelievedIndication> = input
        .split("\n")
        .filter_map(
            |s| {
                let parts: Vec<&str> = s.split(" ").collect();
                let their = parts.get(0).copied().and_then(|s| to_believed_move(s).ok());
                let our = parts.get(1).copied().and_then(|s| to_believed_move(s).ok());
                match (their, our) {
                    (Some(their), Some(our)) => Some(BelievedIndication { our, their }),
                    _ => None,
                }
            }
        ).collect();

    let believed_score = believed_indications.iter().fold(0, |acc, elem| {
        use Move::*;
        use Outcome::*;
        let move_score = match elem.our {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
        let outcome_score = match outcome(elem.our, elem.their) {
            Loss => 0,
            Draw => 3,
            Win => 6,
        };
        
        acc + move_score + outcome_score
    });

    println!("Believed score: {believed_score:?}");

    let indications: Vec<Indication> = input
        .split("\n")
        .filter_map(
            |s| {
                let parts: Vec<&str> = s.split(" ").collect();
                let their = parts.get(0).copied().and_then(|s| s.parse().ok());
                let outcome = parts.get(1).copied().and_then(|s| s.parse().ok());
                match (their, outcome) {
                    (Some(their), Some(outcome)) => Some(Indication { their, outcome}),
                    _ => None,
                }
            }
        ).collect();

    let score = indications.iter().fold(0, |acc, elem| {
        use Move::*;
        use Outcome::*;
        let move_score = match our_move(elem.their, elem.outcome) {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
        let outcome_score = match elem.outcome {
            Loss => 0,
            Draw => 3,
            Win => 6,
        };
        
        acc + move_score + outcome_score
    });

    println!("Actual score: {score:?}");
}

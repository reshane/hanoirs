use std::fmt;
#[derive(Clone)]
struct Game {
    a: Vec<u32>,
    b: Vec<u32>,
    c: Vec<u32>,
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        write!(f, "A:")?;
        for dsk in &self.a {
            write!(f, " {} ", dsk)?;
        }
        write!(f, "\n")?;
        write!(f, "B:")?;
        for dsk in &self.b {
            write!(f, " {} ", dsk)?;
        }
        write!(f, "\n")?;
        write!(f, "C:")?;
        for dsk in &self.c {
            write!(f, " {} ", dsk)?;
        }
        write!(f, "\n")
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        write!(f, "A:")?;
        for dsk in &self.a {
            write!(f, " {} ", dsk)?;
        }
        write!(f, "\n")?;
        write!(f, "B:")?;
        for dsk in &self.b {
            write!(f, " {} ", dsk)?;
        }
        write!(f, "\n")?;
        write!(f, "C:")?;
        for dsk in &self.c {
            write!(f, " {} ", dsk)?;
        }
        write!(f, "\n")
    }
}

#[derive(Debug, Clone)]
enum PegName {
    A,
    B,
    C,
}

fn step<'a>(n: usize, game: &'a mut Game, 
            src: &'a PegName, 
            dst: &'a PegName, 
            aux: &'a PegName) -> &'a mut Game {
    if n > 0 {
        *game = step(n - 1, &mut game.clone(), &src, &aux, &dst).clone();
        match (src.clone(), dst.clone()) {
            (PegName::A, PegName::B) => game.b.push(game.a.pop().unwrap()),
            (PegName::A, PegName::C) => game.c.push(game.a.pop().unwrap()),
            (PegName::B, PegName::A) => game.a.push(game.b.pop().unwrap()),
            (PegName::B, PegName::C) => game.c.push(game.b.pop().unwrap()),
            (PegName::C, PegName::A) => game.a.push(game.c.pop().unwrap()),
            (PegName::C, PegName::B) => game.b.push(game.c.pop().unwrap()),
            (PegName::A, PegName::A) | 
                (PegName::B, PegName::B) | 
                (PegName::C, PegName::C) => {
                    unreachable!()
            }
        }
        print!("{}", &game);
        *game = step(n - 1, &mut game.clone(), &aux, &dst, &src).clone();
    }
    game
}
fn main() {
    let mut game = Game{ a: vec![3,2,1], b: vec![], c: vec![] };
    print!("{}", &game);
    game = step(
        game.clone().a.len() + game.clone().b.len() + game.clone().c.len(), 
        &mut game, 
        &PegName::A, 
        &PegName::C, 
        &PegName::B
    ).clone();
    assert!(*&game.c.len() == (&game.a.len() + &game.b.len() + &game.c.len()));
}

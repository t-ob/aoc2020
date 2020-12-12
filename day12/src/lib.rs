use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Rotation {
    Zero,
    PiOverTwo,
    Pi,
    ThreePiOverTwo,
}

impl Rotation {
    pub fn inverse(&self) -> Rotation {
        match self {
            Rotation::Zero => Rotation::Zero,
            Rotation::PiOverTwo => Rotation::ThreePiOverTwo,
            Rotation::Pi => Rotation::Pi,
            Rotation::ThreePiOverTwo => Rotation::PiOverTwo,
        }
    }
}

pub enum Action {
    Forward(i64),
    North(i64),
    East(i64),
    South(i64),
    West(i64),
    Left(Rotation),
    Right(Rotation),
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let f = chars.next();

        if f == None {
            return Err(String::from("Unable to parse empty string"));
        }

        let g = chars.collect::<String>().parse::<i64>();

        if let Err(e) = g {
            return Err(format!("Unable to parse int: {}", e.to_string()));
        }

        match (f.unwrap(), g.unwrap()) {
            ('F', d) => Ok(Action::Forward(d)),
            ('N', d) => Ok(Action::North(d)),
            ('E', d) => Ok(Action::East(d)),
            ('S', d) => Ok(Action::South(d)),
            ('W', d) => Ok(Action::West(d)),
            ('L', 0) => Ok(Action::Left(Rotation::Zero)),
            ('L', 90) => Ok(Action::Left(Rotation::PiOverTwo)),
            ('L', 180) => Ok(Action::Left(Rotation::Pi)),
            ('L', 270) => Ok(Action::Left(Rotation::ThreePiOverTwo)),
            ('R', 0) => Ok(Action::Right(Rotation::Zero)),
            ('R', 90) => Ok(Action::Right(Rotation::PiOverTwo)),
            ('R', 180) => Ok(Action::Right(Rotation::Pi)),
            ('R', 270) => Ok(Action::Right(Rotation::ThreePiOverTwo)),
            _ => Err(format!("Unable to parse action: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector(pub i64, pub i64);

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl std::ops::Mul<Vector> for i64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0, self * rhs.1)
    }
}

impl Vector {
    pub fn rotate(&mut self, rotation: Rotation) {
        let (x, y) = (self.0, self.1);
        match rotation {
            Rotation::Zero => {}
            Rotation::PiOverTwo => {
                self.0 = -y;
                self.1 = x
            }
            Rotation::Pi => {
                self.0 = -x;
                self.1 = -y
            }
            Rotation::ThreePiOverTwo => {
                self.0 = y;
                self.1 = -x
            }
        }
    }

    pub fn l1_norm(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let north = Vector(0, 1);
        let mut ship = Vector(0, 0);
        let mut direction = Vector(1, 0);

        ship += 10 * direction;
        ship += 3 * north;
        ship += 7 * direction;
        direction.rotate(Rotation::ThreePiOverTwo);
        ship += 11 * direction;

        assert_eq!(ship.l1_norm(), 25)
    }
}

const FORCE: f64 = 10.0;
const MASS: f64 = 0.5;

#[derive(Debug, Clone)]
pub struct Character {
    force: f64,
    mass: f64,
    position: Position,
    face_direction: Direction,
    velocity: [f64; 2]
}

impl Character {
    pub const fn new(position: Position) -> Character {
        Character { force: FORCE, mass: MASS, position, face_direction: Direction::North, velocity: [0.0, 0.0] }
    }

    pub fn move_character(&mut self, dt: f64, direction: Direction) {
        self.face_direction = direction;
        self.update_velocity(dt);

        self.position.set_x(self.position.x + (self.velocity[0] * dt * 1000.0));
        self.position.set_y(self.position.y + (self.velocity[1] * dt * 1000.0));
    }
    
    pub fn stop(&mut self) {
        self.velocity[0] = 0.0;
        self.velocity[1] = 0.0;
    }

    fn update_velocity(&mut self, dt: f64) {
        match self.face_direction {
            Direction::North => {
                self.velocity[1] -= self.force / self.mass * dt;
            },
            Direction::South => {
                self.velocity[1] += self.force / self.mass * dt;
            },
            Direction::East => {
                self.velocity[0] += self.force / self.mass * dt;
            },
            Direction::West => {
                self.velocity[0] -= self.force / self.mass * dt;
            }
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_face_direction(&self) -> &Direction {
        &self.face_direction
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    x: f64,
    y: f64
}

impl Position {
    pub const fn new (x: f64, y: f64) -> Position {
        Position {
            x: 0.0,
            y: 0.0
        }
       
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }


    pub fn get_y(&self) -> f64 {
        self.y
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West
}

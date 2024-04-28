use crate::world::Direction;

mod constants {
    pub const DEFAULT_FORCE: f64 = 10.0;
    pub const DEFAULT_MASS: f64 = 0.5;
    pub const DEFAULT_MAX_SPEED: f64 = 1.5;
}

#[derive(Debug, Clone)]
pub struct Character {
    force: f64,
    mass: f64,
    max_speed: f64,
    position: Position,
    face_direction: Direction,
    velocity: Velocity,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
pub struct CharacterBuilder {
    position: Position,
    force: f64,
    mass: f64,
    max_speed: f64,
}

impl CharacterBuilder {
    pub fn new(position: Position) -> CharacterBuilder {
        CharacterBuilder {
            position,
            force: constants::DEFAULT_FORCE,
            mass: constants::DEFAULT_MASS,
            max_speed: constants::DEFAULT_MAX_SPEED,
        }
    }

    pub fn with_force(mut self, force: f64) -> CharacterBuilder {
        self.force = force;
        self
    }

    pub fn with_mass(mut self, mass: f64) -> CharacterBuilder {
        self.mass = mass;
        self
    }

    pub fn with_max_speed(mut self, max_speed: f64) -> CharacterBuilder {
        self.max_speed = max_speed;
        self
    }

    pub fn build(self) -> Character {
        Character {
            force: self.force,
            mass: self.mass,
            max_speed: self.max_speed,
            position: self.position,
            face_direction: Direction::North,
            velocity: Velocity { x: 0.0, y: 0.0 },
        }
    }
}

impl Character {
    pub fn builder(position: Position) -> CharacterBuilder {
        CharacterBuilder::new(position)
    }

    pub fn move_character(&mut self, direction: Direction, dt: f64) {
        if direction != self.face_direction {
            self.face_direction = direction;
            self.stop();
        }

        self.update_velocity(dt);

        self.position.x += self.velocity.x * dt * 1000.0;
        self.position.y += self.velocity.y * dt * 1000.0;
    }

    fn update_velocity(&mut self, dt: f64) {
        match self.face_direction {
            Direction::North => {
                self.velocity.y -= self.force / self.mass * dt;
            }
            Direction::South => {
                self.velocity.y += self.force / self.mass * dt;
            }
            Direction::East => {
                self.velocity.x += self.force / self.mass * dt;
            }
            Direction::West => {
                self.velocity.x -= self.force / self.mass * dt;
            }
        };

        self.limit_speed();
    }

    fn limit_speed(&mut self) {
        self.velocity.x = self.velocity.x.clamp(-self.max_speed, self.max_speed);
        self.velocity.y = self.velocity.y.clamp(-self.max_speed, self.max_speed);
    }

    pub fn stop(&mut self) {
        self.velocity = Velocity { x: 0.0, y: 0.0 };
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_face_direction(&self) -> &Direction {
        &self.face_direction
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub const fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }
}

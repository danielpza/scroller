#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub position: Point,
    pub size: Point,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            position: Point { x, y },
            size: Point { x: w, y: h },
        }
    }
    pub fn top(self) -> f32 {
        self.position.y
    }
    pub fn set_top(&mut self, top: f32) {
        self.position.y = top;
    }
    pub fn bottom(self) -> f32 {
        self.position.y + self.size.y
    }
    pub fn set_bottom(&mut self, bottom: f32) {
        self.position.y = bottom - self.size.y;
    }
    pub fn left(self) -> f32 {
        self.position.x
    }
    pub fn set_left(&mut self, left: f32) {
        self.position.x = left;
    }
    pub fn right(self) -> f32 {
        self.position.x + self.size.x
    }
    pub fn set_right(&mut self, right: f32) {
        self.position.x = right - self.size.x;
    }
}

fn force_to_jump(height: f32, gravity: f32) -> f32 {
    (2.0 * height * gravity).sqrt()
}

#[derive(Debug)]
pub struct Player {
    pub shape: Rect,
    pub speed: Point,
}

pub struct Input {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

pub struct Game {
    width: i32,
    height: i32,
    pub player: Player,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
            player: Player {
                shape: Rect::new(0.0, 0.0, 100.0, 100.0),
                speed: Point { x: 0.0, y: 0.0 },
            },
        }
    }
    pub fn step(&mut self, input: Input) {
        let speed = 5.0;
        let gravity = 0.4;

        if self.player.shape.bottom() + gravity < self.height as f32 {
            self.player.speed.y += gravity;
        } else {
            self.player.speed.y = 0.0;
            self.player.shape.set_bottom(self.height as f32);
            if input.jump {
                self.player.speed.y = -force_to_jump(self.player.shape.size.y * 1.2, gravity);
            }
        }

        self.player.speed.x = 0.0;
        if input.left {
            self.player.speed.x -= speed;
        }
        if input.right {
            self.player.speed.x += speed;
        }

        self.player.shape.position += self.player.speed;
    }
}

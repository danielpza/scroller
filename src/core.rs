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

pub struct Player {
    pub shape: Rect,
}

pub struct Input {
    pub left: bool,
    pub right: bool,
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
            },
        }
    }
    pub fn step(&mut self, input: Input) {
        let speed = 3.0;
        let gravity = 3.0;

        if self.player.shape.bottom() + gravity < self.height as f32 {
            self.player.shape.position.y += gravity;
        } else {
            self.player.shape.set_bottom(self.height as f32);
        }

        let next_speed = {
            let mut temp = 0.0;
            if input.left {
                temp -= speed;
            }
            if input.right {
                temp += speed;
            }
            temp
        };

        self.player.shape.position.x += next_speed;
    }
}

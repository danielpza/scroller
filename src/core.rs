#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, value: f32) -> Point {
        Point {
            x: self.x * value,
            y: self.y * value,
        }
    }
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
    pub fn offset(&mut self, point: Point) {
        self.position += point;
    }
}

impl std::ops::Mul<f32> for Rect {
    type Output = Rect;
    fn mul(self, value: f32) -> Rect {
        Rect {
            position: self.position * value,
            size: self.size * value,
        }
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
    pub jump: bool,
}

const WIDTH: i32 = 14;

pub struct Map {
    height: i32,
    floor: [i32; WIDTH as usize],
}

impl Map {
    pub fn new(height: i32) -> Map {
        Map {
            height,
            floor: [
                height - 2,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
                height - 1,
            ],
        }
    }
    pub fn get_top(&self, from: i32, to: i32) -> i32 {
        let to = (to % WIDTH) as usize;
        let mut from = (from % WIDTH) as usize;
        let mut top = self.height;
        while from != to {
            top = top.min(self.floor[from]);
            from = (from + 1) % WIDTH as usize;
        }
        top
    }
    pub fn height_at(&self, pos: i32) -> i32 {
        self.floor[(pos % WIDTH) as usize]
    }
}

pub struct Game {
    pub width: i32,
    pub height: i32,
    pub player: Player,
    pub offset: f32,
    pub map: Map,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
            map: Map::new(height),
            offset: 0.0,
            player: Player {
                shape: Rect::new(WIDTH as f32 / 3.0, 1.0, 0.8, 0.8),
                speed: Point { x: 0.1, y: 0.0 },
            },
        }
    }

    pub fn step(&mut self, input: Input) {
        let gravity = 0.01;
        let speed = 0.1;

        self.player.speed.x = speed;
        self.player.speed.y += gravity;

        let under = self.map.get_top(
            self.player.shape.left().floor() as i32,
            self.player.shape.right().ceil() as i32,
        ) as f32;

        if self.player.shape.bottom() + self.player.speed.y >= under {
            self.player.speed.y = 0.0;
            self.player.shape.set_bottom(under);
            if input.jump {
                self.player.speed.y = -force_to_jump(1.2, gravity);
            }
        }

        let underh = self.map.get_top(
            (self.player.shape.left() + self.player.speed.x).floor() as i32,
            (self.player.shape.right() + self.player.speed.x).ceil() as i32,
        ) as f32;

        if self.player.shape.bottom() > underh {
            if self.player.speed.x > 0.0 {
                self.player
                    .shape
                    .set_right(self.player.shape.right().ceil())
            } else if self.player.speed.x < 0.0 {
                self.player.shape.set_left(self.player.shape.left().floor())
            }
            self.player.speed.x = 0.0;
        }

        self.player.shape.position += self.player.speed;
        self.offset += speed;
    }
}

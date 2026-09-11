use raylib::prelude::*;

enum Screen {
    Menu,
}

struct Bear {
    pos: Vector2,
    size: Vector2,
    being_dragged: bool,
}

impl Bear {
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2 { x, y },
            size: Vector2 { x: 25.0, y: 50.0 },
            being_dragged: false,
        }
    }

    fn is_touching_mouse(&self, mouse_pos: &Vector2) -> bool {
        (mouse_pos.x < self.pos.x + self.size.x / 2.0) &&
        (mouse_pos.x > self.pos.x - self.size.x / 2.0) &&
        (mouse_pos.y < self.pos.y + self.size.y / 2.0) &&
        (mouse_pos.y > self.pos.y - self.size.y / 2.0)
    }
}

pub struct Game {
    bears: Vec<Bear>,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            bears: vec![Bear::new(960.0, 480.0), Bear::new(100.0, 100.0)]
        }
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        let mouse_pos = d.get_mouse_position();
        for bear in &mut self.bears {
            if bear.being_dragged {
                if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    bear.pos = mouse_pos;
                } else {
                    bear.being_dragged = false;
                }
            } else {
                if bear.is_touching_mouse(&mouse_pos) && d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    bear.being_dragged = true;
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for bear in &self.bears {
            d.draw_rectangle(
                (bear.pos.x - bear.size.x / 2.0) as i32,
                (bear.pos.y - bear.size.y / 2.0) as i32,
                bear.size.x as i32,
                bear.size.y as i32,
                Color::BLACK
            );
        }
    }
}
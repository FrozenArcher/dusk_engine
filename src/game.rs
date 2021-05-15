use crate::game_window::GameWindow;
use crate::sprite::Sprite;

pub struct Game {
    window: GameWindow,
    is_running: bool,
    sprites: Vec<Sprite>,
}

impl Game {
    pub fn new(builder: GameWindowBuilder) -> Self {
        Game{window: GameWindow::new(builder), is_running: true,
        sprites: Vec::new()}
    }
    pub fn run(&mut self) {
        let (w, h, img) = self.window.test_image_init();
        while self.is_running {
            self.window.detect_exit_event(&mut (self.is_running),
                w, h, img);
            for spr in self.sprites.iter_mut() {
                spr.update();
            }
            for spr in self.sprites.iter() {
                spr.draw_self();
            }
        }
    }
    pub fn instantiate(&mut self, object: Sprite, _x: f64, _y: f64) {
        self.sprites.push(object);
    }
}

pub struct GameWindowBuilder {
    pub width: u32,
    pub height: u32,
    pub title_s: String,
}

impl GameWindowBuilder {
    pub fn new() -> Self {
        GameWindowBuilder {
            width: 600, height: 400, title_s: String::from("My Game"),
        }
    }
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title_s = String::from(title);
        self
    }
}

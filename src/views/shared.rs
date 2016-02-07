use ::phi::data::Rectangle;
use ::phi::gfx::{CopySprite, Sprite};
use ::sdl2::render::Renderer;

#[derive(Clone)]
pub struct Background {
    pub pos: f64,
    // The number of pixels moved left per second
    pub vel: f64,
    pub sprite: Sprite,
}

impl Background {
    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        // Define a logical position depending solely on time and dimensions of
        // sprite, not on screen's size.
        let (sprite_w, sprite_h) = self.sprite.size();
        self.pos += self.vel * elapsed;
        if self.pos > sprite_w {
            self.pos -= sprite_w
        }

        // Determine the scale ratio of the window to the sprite
        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale = win_h as f64 / sprite_h;

        // Render as many copies of background as needed to fill the screen.
        let mut physical_left = -self.pos * scale;

        while physical_left < win_w as f64 {
            renderer.copy_sprite(&self.sprite, Rectangle {
                x: physical_left,
                y: 0.0,
                w: sprite_w * scale,
                h: win_h as f64,
            });

            physical_left += sprite_w * scale;
        }
    }
}

#[derive(Clone)]
pub struct BgSet {
    pub back: Background,
    pub middle: Background,
    pub front: Background,
}

impl BgSet {
    pub fn new(renderer: &Renderer) -> BgSet {
        BgSet {
            back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(&renderer, "assets/starBG.png").unwrap(),
            },
            middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(&renderer, "assets/starMG.png").unwrap(),
            },
            front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(&renderer, "assets/starFG.png").unwrap(),
            },
        }
    }
}

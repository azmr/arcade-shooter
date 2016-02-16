use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::{AnimatedSprite, CopySprite, Sprite};
use ::sdl2::pixels::Color;
use ::views::shared::{Background, BgSet};

// Constants
const DEBUG: bool = false;

/// Pixels travelled by the player's ship every second when it is moving.
const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

const ASTEROID_PATH: &'static str = "assets/asteroid.png";
const ASTEROIDS_W: usize = 21;
const ASTEROIDS_H: usize = 7;
const ASTEROIDS_TOTAL: usize = ASTEROIDS_W * ASTEROIDS_H - 4;
const ASTEROID_SIDE: f64 = 96.0;

/// The different states the ship might be in. In the image, they're ordered
/// from left to right, then from top to bottom.
#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm      = 0,
    UpFast      = 1,
    UpSlow      = 2,
    MidNorm     = 3,
    MidFast     = 4,
    MidSlow     = 5,
    DownNorm    = 6,
    DownFast    = 7,
    DownSlow    = 8,
}
    
// Data Types
struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
}

struct Asteroid {
    sprite: AnimatedSprite,
    rect: Rectangle,
    vel: f64,
}

impl Asteroid {
    fn new(phi: &mut Phi) -> Asteroid {
        let mut asteroid =
            Asteroid {
                sprite: Asteroid::get_sprite(phi, 15.0),
                rect: Rectangle {
                    w: ASTEROID_SIDE,
                    h: ASTEROID_SIDE,
                    x: 128.0,
                    y: 128.0,
                },
                vel: 0.0,
            };

        asteroid.reset(phi);
        asteroid
    }

    fn reset(&mut self, phi: &mut Phi) {
        let (w, h) = phi.output_size();

        // set animation fps in [10.0, 30.0]
        self.sprite.set_fps(::rand::random::<f64>().abs() * 20.0 + 10.0);

        // rect.y in the screen vertically
        self.rect = Rectangle {
            w: ASTEROID_SIDE,
            h: ASTEROID_SIDE,
            x: w,
            y: ::rand::random::<f64>().abs() * (h - ASTEROID_SIDE),
        };

        // set vel in [50.0, 150.0]
        self.vel = ::rand::random::<f64>().abs() * 100.0 + 50.0;
    }

    fn get_sprite(phi: &mut Phi, fps: f64) -> AnimatedSprite {
        let asteroid_spritesheet = Sprite::load(&mut phi.renderer, ASTEROID_PATH).unwrap();
        let mut asteroid_sprites = Vec::with_capacity(ASTEROIDS_TOTAL);

        for yth in 0..ASTEROIDS_H {
            for xth in 0..ASTEROIDS_W {
                if (ASTEROIDS_W * yth) + xth >= ASTEROIDS_TOTAL {
                    break;
                }

                asteroid_sprites.push(
                    asteroid_spritesheet.region(Rectangle {
                        w: ASTEROID_SIDE,
                        h: ASTEROID_SIDE,
                        x: ASTEROID_SIDE * xth as f64,
                        y: ASTEROID_SIDE * yth as f64,
                    }).unwrap());
            }
        }

        AnimatedSprite::with_fps(asteroid_sprites, fps)
    }
    
    fn update(&mut self, phi: &mut Phi, dt: f64) {
        self.rect.x -= dt * self.vel;
        self.sprite.add_time(dt);

        if self.rect.x <= -ASTEROID_SIDE {
            self.reset(phi);
        }
    }

    fn render(&mut self, phi: &mut Phi) {
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }
}

// View Definition
pub struct ShipView {
    player: Ship,
    asteroid: Asteroid,
    backgrounds: BgSet,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        let backgrounds = BgSet::new(&mut phi.renderer);
        ShipView::with_backgrounds(phi, backgrounds)
        }


    pub fn with_backgrounds(phi: &mut Phi, backgrounds: BgSet) -> ShipView {
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: SHIP_W,
                    h: SHIP_H,
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                }).unwrap());
            }
        }

        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
            },

            asteroid: Asteroid::new(phi),
            
            backgrounds: backgrounds,
        }
    }
}


impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit {
            return ViewAction::Quit;
        }
        if phi.events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(Box::new(
                    ::views::main_menu::MainMenuView::with_backgrounds(phi, self.backgrounds.clone())));
        }

        // Move player ship
        let diagonal =
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0_f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        // If the player cannot fit in the screen, abort game.
        // Otherwise, bound player inside movable_region.
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        // Select appropriate sprite of ship to show.
        self.player.current =
            if dx == 0.0 && dy < 0.0        { ShipFrame::UpNorm }
            else if dx > 0.0 && dy < 0.0    { ShipFrame::UpFast }
            else if dx < 0.0 && dy < 0.0    { ShipFrame::UpSlow }
            else if dx == 0.0 && dy == 0.0  { ShipFrame::MidNorm }
            else if dx > 0.0 && dy == 0.0   { ShipFrame::MidFast }
            else if dx < 0.0 && dy == 0.0   { ShipFrame::MidSlow }
            else if dx == 0.0 && dy > 0.0   { ShipFrame::DownNorm }
            else if dx > 0.0 && dy > 0.0    { ShipFrame::DownFast }
            else if dx < 0.0 && dy > 0.0    { ShipFrame::DownSlow }
            else { unreachable!() };

        // Update the asteroid
        self.asteroid.update(phi, elapsed);
            

        // Clear screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render backgrounds
        self.backgrounds.back.render(&mut phi.renderer, elapsed);
        self.backgrounds.middle.render(&mut phi.renderer, elapsed);

        // Render ship bounding box for debugging
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());
        }

        // Render ship
        phi.renderer.copy_sprite(
            &self.player.sprites[self.player.current as usize],
            self.player.rect);

        self.asteroid.render(phi);

        // Render foreground
        self.backgrounds.front.render(&mut phi.renderer, elapsed);

        ViewAction::None
    }
}

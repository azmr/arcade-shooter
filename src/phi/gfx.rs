use ::phi::data::Rectangle;
use ::sdl2::render::{Renderer, Texture};
use ::sdl2_image::LoadTexture;
use ::std::cell::RefCell;
use ::std::path::Path;
use ::std::rc::Rc;

#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

/// Common interface for rendering graphical components to a given window region
pub trait Renderable {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle);
}

impl Sprite {
    /// Creates a new sprite by wrapping a `Texture`.
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: tex_query.width as f64,
                h: tex_query.height as f64,
                x: 0.0,
                y: 0.0,
            }
        }
    }

    /// Creates a new sprite from an image file located at the given path.
    /// Returns `Some` if the file could be read, and `None` otherwise.
    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    /// Returns a new `Sprite` representing a sub-region of the current one.
    /// The provided `rect` is relative to the currently held region.
    /// Returns `Some` if the `rect` is valid, i.e. included in the current
    /// regiion, and `None` otherwise.
    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = Rectangle {
            x: rect.x + self.src.x,
            y: rect.y + self.src.y,
            ..rect
        };

        // Verify that the requested region is inside of the current one
        if self.src.contains(new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src,
            })
        } else {
            None
        }
    }

    /// Returns the dimensions of the region.
    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }
}

impl Renderable for Sprite {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl())
    }
}

pub trait CopySprite<T> {
    fn copy_sprite(&mut self, sprite: &T, dest: Rectangle);
}

impl<'window, T:Renderable> CopySprite<T> for Renderer<'window> {
    fn copy_sprite(&mut self, sprite: &T, dest: Rectangle) {
        sprite.render(self, dest);
    }
}

#[derive(Clone)]
pub struct AnimatedSprite {
    /// The frames that will be rendered, in order.
    sprites: Vec<Sprite>,

    /// The time between frames, in seconds
    frame_delay: f64,

    /// The total alive time of the sprite, used to derive the current sprite
    current_time: f64,
}

impl AnimatedSprite {
    /// Creates a new animated sprite init to time 0
    pub fn new(sprites: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
        AnimatedSprite {
            sprites: Rc::new(sprites),
            frame_delay: frame_delay,
            current_time: 0.0,
        }
    }

    /// Creates a new animated sprite that changes frame `fps` times per second
    pub fn with_fps(sprites: Vec<Sprite>, fps: f64) -> AnimatedSprite {
        // TODO: stop animating when 0fps
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::with_fps");
        }

        let frame_delay = 1.0 / fps;
        AnimatedSprite::new(sprites, frame_delay)
    }


    // The number of frames in the animation
    pub fn frames(&self) -> usize {
        self.sprites.len()
    }

    /// Set the time between frames in seconds.
    /// Negative values animate in reverse.
    pub fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }

    /// Set the number of frames per second for the animation.
    /// Negative values animate in reverse.
    pub fn set_fps(&mut self, fps: f64) {
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::set_fps");
        }

        let frame_delay = 1.0 / fps;
        self.set_frame_delay(frame_delay);
    }

    /// Add time (in seconds) to `current_time` of animated sprite.
    /// Provides timing for next frame.
    pub fn add_time(&mut self, dt: f64) {
        self.current_time += dt;

        // Go back in time >> select last frame
        if self.current_time < 0.0 {
            self.current_time = (self.frames() - 1) as f64 * self.frame_delay;
        }
    }
}

impl Renderable for AnimatedSprite {
    /// Renders current frame
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        let current_frame =
            (self.current_time / self.frame_delay) as usize % self.frames();

        let sprite = &self.sprites[current_frame];
        sprite.render(renderer, dest);
    }
}

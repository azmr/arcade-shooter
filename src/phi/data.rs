use ::sdl2::rect::Rect as SDLRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates SDL-compatible Rect equivalent to `self`. Panics if it could
    /// not be created, for example if a corner coordinate overflows an `i32`.
    pub fn to_sdl(self) -> Option<SDLRect> {
        // Reject negative width and height
        assert!(self.w >= 0.0 && self.h >= 0.0);

        // SDLRect::new : `(i32, i32, u32, u32) -> Result<Option<SDLRect>>`
        SDLRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
            .unwrap()
    }

    /// Return a (perhaps moved) rectangle which is contained by a `parent`
    /// rectangle. If it can indeed be moved to fit, return `Some(result)`,
    /// otherwise, return `None`.
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        // It must be smaller thant the parent rectangle to fit in it.
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,
            x: if self.x < parent.x {
                    parent.x
                }
                else if self.x + self.w >= parent.x + parent.w {
                    parent.x + parent.w - self.w
                }
                else {
                    self.x
                },
            y: if self.y < parent.y {
                    parent.y
                }
                else if self.y + self.h >= parent.y + parent.h {
                    parent.y + parent.h - self.h
                }
                else {
                    self.y
                },
        })
    }
    
    // NOTE: for next 2 functions:
    //          `as i32` removed
    //          `Rect` -> `Rectangle` 
    pub fn contains(&self, rect: Rectangle) -> bool {
        let xmin = rect.x;
        let xmax = xmin + rect.w;
        let ymin = rect.y;
        let ymax = ymin + rect.h;

        let xmin_contained = (xmin >= self.x) && (xmin <= self.x + self.w);
        let xmax_contained = (xmax >= self.x) && (xmax <= self.x + self.w);
        let ymin_contained = (ymin >= self.y) && (ymin <= self.y + self.h);
        let ymax_contained = (ymax >= self.y) && (ymax <= self.y + self.h);

        xmin_contained && xmax_contained && ymin_contained && ymax_contained
    }

    pub fn overlaps(&self, other: Rectangle) -> bool {
        // NOTE: is < rather than <= in tutorial
        let left_edge_left_of_right = self.x <= other.x + other.w;
        let right_edge_right_of_left = self.x + self.w >= other.x;
        let top_edge_above_bottom = self.y <= other.y + other.h;
        let bottom_edge_below_top = self.y + self.h >= other.y;

        left_edge_left_of_right && right_edge_right_of_left &&
            top_edge_above_bottom && bottom_edge_below_top
    }
}


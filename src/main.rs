extern crate sdl2;
extern crate sdl2_image;

mod phi;
mod views;

// use ::phi::{Events, Phi, View, ViewAction};

fn main() {
    ::phi::spawn("Arcade Shooter", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}

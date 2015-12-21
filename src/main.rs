extern crate sdl2;

mod phi;
mod views;

// use ::phi::{Events, Phi, View, ViewAction};

fn main() {
    ::phi::spawn("Arcade Shooter", |_| {
        Box::new(::views::ViewA)
    });
}

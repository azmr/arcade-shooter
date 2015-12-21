extern crate sdl2;

#[macro_use]
mod events;

use ::sdl2::pixels::Color;

struct_events! {
    keyboard: {
        key_escape: Escape,
    },
    else: {
        quit: Quit { .. }
    }
}

fn main() {
    // init SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // create window
    let window = video.window ("Arcade Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();
    
    // prepare events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());


    loop {
        events.pump();
        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }

        // render black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}

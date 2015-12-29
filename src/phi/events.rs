macro_rules! struct_events {
    (
        keyboard: { $( $key_alias:ident : $key_sdl:ident, )* },

        else: { $( $exit_alias:ident : $exit_sdl:pat ),* }
    ) => {
        use ::sdl2::EventPump;


        pub struct ImmediateEvents {
            $( pub $key_alias : Option<bool>, )*
            $( pub $exit_alias : bool, )*
            resize: Option<(u32, u32)>,
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    $( $key_alias : None, )*
                    $( $exit_alias : false, )*
                    resize: None,
                }
            }
        }


        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            $( pub $key_alias: bool, )*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    $( $key_alias: false, )*
                }
            }

            pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use ::sdl2::event::Event::*;
                    use ::sdl2::event::WindowEventId::Resized;
                    use ::sdl2::keyboard::Keycode::*;

                    match event {
                        Window { win_event_id: Resized, .. } => {
                            self.now.resize = Some(renderer.output_size().unwrap());
                        },

                        KeyDown { keycode, .. } => match keycode {
                            $(
                                Some($key_sdl) => {
                                    if !self.$key_alias {
                                        // Key pressed, wasn't before
                                        self.now.$key_alias = Some(true);
                                        
                                        println!("Key down: {}", $key_sdl);
                                    }

                                    self.$key_alias = true;
                                },
                            )*

                            _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($key_sdl) => {
                                    // key released
                                    self.now.$key_alias = Some(false);
                                    self.$key_alias = false;
                                },
                            )*

                            _ => {}
                        },

                        $(// SDL calls exit (e.g. X button)
                            $exit_sdl => {
                                self.now.$exit_alias = true;
                            },
                        )*

                        _ => {},
                    }
                }
            }
        }
    }
}

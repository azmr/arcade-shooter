macro_rules! struct_events {
    (
        keyboard: { $( $key_alias:ident : $key_sdl:ident, )* },

        else: { $( $exit_alias:ident : $exit_sdl:pat ),* }
    ) => {
        use ::sdl2::EventPump;


        pub struct ImmediateEvents {
            $( pub $key_alias : Option<bool>, )*
            $( pub $exit_alias : bool, )*
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    $( $key_alias : None, )*
                    $( $exit_alias : false, )*
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

            pub fn pump(&mut self) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use ::sdl2::event::Event::*;
                    use ::sdl2::keyboard::Keycode::*;

                    match event {
                        KeyDown { keycode, .. } => match keycode {
                            $(
                                Some($key_sdl) => {
                                    if !self.$key_alias {
                                        // Key pressed, wasn't before
                                        self.now.$key_alias = Some(true);
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

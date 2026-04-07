use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{SimulatorDisplay, Window};

fn main() -> Result<(), std::convert::Infallible> {
    // use a small display for demonsration
    let display_size = Size::new(128, 64);

    // create a simulated display per feature choice
    if cfg!(feature = "binary_color") {
        // create a simulation display of binary pixels
        let display: SimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor> =
            SimulatorDisplay::new(display_size);
        let output_settings = embedded_graphics_simulator::OutputSettingsBuilder::new()
            .theme(embedded_graphics_simulator::BinaryColorTheme::OledBlue)
            .build();
        run(display, output_settings);
    } else {
        // create a simulation display with full color
        let display: SimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor> =
            SimulatorDisplay::new(display_size);
        let output_settings = embedded_graphics_simulator::OutputSettingsBuilder::new().build();
        run(display, output_settings);
    }
    Ok(())
}

fn run<Color>(
    mut display: SimulatorDisplay<Color>,
    output_settings: embedded_graphics_simulator::OutputSettings,
) where
    // allow UX to use max colors (which will be converted into actual display Color)
    Color: PixelColor + Into<Rgb888> + From<Rgb888>,
{
    use embedded_graphics_simulator::sdl2::Keycode;

    // create the native window to display the simulation
    let mut window = Window::new(
        "embedded-graphics-ux (spacebar as button)",
        &output_settings,
    );

    // create a simulation button
    const SIMULATED_BUTTON: Keycode = Keycode::SPACE; // use spacebar as button

    let mut button_down_time: Option<std::time::Instant> = None;
    'running: loop {
        use embedded_graphics_simulator::SimulatorEvent;
        use embedded_graphics_simulator::sdl2::{Mod, MouseButton};

        // handle window events
        window.update(&display);
        for event in window.events() {
            match event {
                // kill the window upon Quit
                SimulatorEvent::Quit => break 'running,

                // upon simulated button DOWN memo now(), to later determine how long the button was held
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat,
                } => {
                    if (keycode == SIMULATED_BUTTON) && !repeat {
                        // record the event timestamp to determine type of interaction
                        button_down_time = Some(std::time::Instant::now());
                    }
                }

                // handle keyboard events per browser pattern
                // * for simulated button, use duration of press to determine the navigtation event
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    if (keycode == SIMULATED_BUTTON) && !repeat {
                        if let Some(start) = button_down_time {
                            // handle button press
                            let elapsed_millis = (std::time::Instant::now() - start).as_millis();
                            if elapsed_millis > 300 {
                                // TODO handle as SELECT0
                            } else {
                                // TODO handle as NEXT
                            }
                            button_down_time = None;
                        }
                    }
                    // handle standard keyboard ux
                    else if keycode == Keycode::TAB {
                        if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                            // handle the event by the UX
                            // ux.handle_event(enmesh::ux::HidEvent::Previous);
                        } else {
                            // handle the event by the UX
                            // ux.handle_event(enmesh::ux::HidEvent::Next);
                        }
                    } else if (keycode == Keycode::RETURN) || (keycode == Keycode::RETURN2) {
                        // handle the event by the UX
                        // ux.handle_event(enmesh::ux::HidEvent::Select);
                    }
                }

                SimulatorEvent::MouseButtonDown { mouse_btn, point } => {
                    if mouse_btn == MouseButton::Left {
                        println!("XXXXX CLICK @ ({point}) XXXXXXX");
                        // TODO handle event by the UX
                    }
                }

                // ignore all other events
                _ => {}
            }
        }

        //     // update the simulated display
        //     // ux.update(&mut display);
        //     // TEST USE ONLY
        //     // demo_page.update(&mut display);

        // sleep for a frame period
        const FPS_HZ: u64 = 10; // 10 times a second
        const FRAME_PERIOD_MILLIS: u64 = 1000 / FPS_HZ;
        use std::time::Duration;
        std::thread::sleep(Duration::from_millis(FRAME_PERIOD_MILLIS));
    }
}

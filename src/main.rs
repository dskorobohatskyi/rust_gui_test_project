mod common;

#[cfg(feature = "immediate-mode")]
mod immediate_mode_app;

#[cfg(feature = "retained-mode")]
mod retained_mode_app;


// TODOS
// Add some retained based mode to check its work
// add scripts for both modes
// think about expectations from ui
// how to change size of fonts, text lines, alignment
// check if some another solution can be used to keep temp data (from architecture - state machine of UI states)
// can i get rid of eframe to minimize the dependencies to use only egui? is it problem now? // audit dependencies + cargo deny?
// check ctx.request_repaint() usage

// TODO setting to not update frame if no input from user + side effect


enum Mode {
    ImmediateMode,
    RetainedMode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let current_mode = if cfg!(feature = "immediate-mode") {
        Mode::ImmediateMode
    } else {
        Mode::RetainedMode
    };

    match current_mode {
        Mode::ImmediateMode => {
            #[cfg(feature = "immediate-mode")]
            immediate_mode_app::run()?;
        }
        Mode::RetainedMode => {
            #[cfg(feature = "retained-mode")]
            retained_mode_app::run()?;
        }
    }

    Ok(())
}

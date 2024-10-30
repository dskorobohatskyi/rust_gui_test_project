mod common;

#[cfg(feature = "immediate-mode")]
mod immediate_mode_app;

#[cfg(feature = "retained-mode")]
mod retained_mode_app;

// TODOS
// add scripts for both modes
// think about expectations from ui
// how to change size of fonts, text lines, alignment (how to use styles)

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

mod immediate_mode_app;

use immediate_mode_app::{ImmediateModeApp};

// TODOS
// create common module to reuse structs
// Add some retained based mode to check its work 
// add #feature to switch between immediate mode and retained one.
// think about expectations from ui
// how to change size of fonts, text lines, alignment
// check if some another solution can be used to keep temp data (from architecture - state machine of UI states)
// can i get rid of eframe to minimize the dependencies to use only egui? is it problem now? // audit dependencies + cargo deny?
// check ctx.request_repaint() usage

// TODO setting to not update frame if no input from user + side effect



fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Complex Egui Example",
        options,
        Box::new(|_cc| Box::new(ImmediateModeApp::default())),
    )
}


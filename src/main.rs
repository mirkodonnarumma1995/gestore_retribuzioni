// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![windows_subsystem = "windows"]

use std::error::Error;

slint::include_modules!();

const PERCENTUALE_SPESE: f64 = 0.50;
const PERCENTUALE_SVAGO: f64 = 0.30;
const PERCENTUALE_RISPARMI: f64 = 0.20;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_dividi_somma({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let spese: f64 = num * PERCENTUALE_SPESE;
            let svago: f64 = num * PERCENTUALE_SVAGO;
            let risparmi: f64 = num * PERCENTUALE_RISPARMI;
            let risultato: String = format!(
                "Spese: {:.2}€\nSvago: {:.2}€\nRisparmi: {:.2}€",
                spese, svago, risparmi
            );
            ui.set_risultato(risultato.into());
        }
    });

    ui.run()?;

    Ok(())
}

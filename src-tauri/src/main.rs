#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
	tauri::Builder::default()
	.setup(|app| {
		let splashscreen_window = app.get_window("splashscreen").unwrap();
		let main_window = app.get_window("main").unwrap();

		// DISABLE RELOAD ======================================================================== //
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.keyCode == 116) { e.preventDefault(); }});").unwrap(); // F5
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 82) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + R
		// DISABLE VIEWING SOURCE ================================================================ //
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 85) { e.preventDefault(); }});").unwrap(); // CTRL + U
		// DISABLE PRINT ========================================================================= //
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 80) { e.preventDefault(); }});").unwrap(); // CTRL + P
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 80) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + P
		// DISABLE SCREENSHOT ==================================================================== //
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 83) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + S
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 88) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + X
		// DISABLE DEVELOPER OPTIONS ============================================================= //
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 73) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + I
		// DISABLE FIND IN PAGE ================================================================== //
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 70) { e.preventDefault(); }});").unwrap(); // CTRL + F
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 71) { e.preventDefault(); }});").unwrap(); // CTRL + G
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.keyCode == 114) { e.preventDefault(); }});").unwrap(); // F3
		// DISABLE CARET BROWSING ================================================================ //
		main_window.eval("window.addEventListener('keydown', function(e) {if (e.keyCode == 118) { e.preventDefault(); }});").unwrap(); // F7
		// DISABLE MIDDLE-CLICK TO OPEN LINKS IN NEW WINDOWS ===================================== //
		main_window.eval("window.addEventListener('auxclick', function(e) {if (e.button == 1) { e.preventDefault(); }});").unwrap();
		// DISABLE RIGHT CLICK =================================================================== //
		main_window.eval("window.addEventListener('contextmenu', function(e) { e.preventDefault(); });").unwrap();

		// WE PERFORM THE INITIALIZATION CODE ON A NEW TASK SO THE APP DOESN'T FREEZE ============ //
		tauri::async_runtime::spawn(async move {
			// INITIALIZE YOUR APP HERE INSTEAD OF SLEEPING :) ===================================== //
			println!("Initializing...");
			std::thread::sleep(std::time::Duration::from_secs(5));
			println!("Done initializing.");
			// AFTER IT'S DONE, CLOSE THE SPLASHSCREEN AND DISPLAY THE MAIN WINDOW ================= //
			splashscreen_window.close().unwrap();
			main_window.show().unwrap();
		});
		Ok(())
	})
	.run(tauri::generate_context!())
	.expect("failed to run app");
}
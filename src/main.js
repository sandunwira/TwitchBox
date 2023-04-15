import { invoke } from '@tauri-apps/api/tauri'
const invoke = window.__TAURI__.invoke


// CONFIGURING SPLASHSCREEN START ===================================== //
document.addEventListener('DOMContentLoaded', () => {
	invoke('close_splashscreen')
})
// ======================================= CONFIGURING SPLASHSCREEN END //
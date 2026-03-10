import init, { start_app, execute_rhai, resize_map, fly_to_user, add_pin_from_js, delete_pin_from_js } from './pkg/rust_maps.js';

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('./sw.js').catch(console.error);
}

// Global UI Bindings for Rust Popups
window.submit_pin = (lat, lon) => {
    const title = document.getElementById('new-pin-title').value || "New Pin";
    window.wasm_add_pin(lat, lon, title);
};

window.delete_pin = (id) => {
    window.wasm_delete_pin(id);
};

window.onload = async () => {
    if (typeof DeviceMotionEvent !== 'undefined' && typeof DeviceMotionEvent.requestPermission === 'function') {
        try { await DeviceMotionEvent.requestPermission(); } catch(e) { console.error(e); }
    }
    
    // Boot WASM
    await init();
    
    // Bind Exports cleanly to window AFTER init resolves
    window.wasm_add_pin = add_pin_from_js;
    window.wasm_delete_pin = delete_pin_from_js;
    
    // Start Application
    start_app();
};

document.getElementById('btn-locate').addEventListener('click', () => {
    fly_to_user();
});

const tabs = document.querySelectorAll('.tab');
const panels = document.querySelectorAll('.panel');
tabs.forEach(tab => {
    tab.addEventListener('click', () => {
        tabs.forEach(t => t.classList.remove('active'));
        panels.forEach(p => p.classList.remove('active'));
        tab.classList.add('active');
        
        const target = tab.getAttribute('data-target');
        if (target !== 'panel-map') {
            document.getElementById(target).classList.add('active');
        } else {
            resize_map();
        }
    });
});

document.getElementById('btn-run').addEventListener('click', () => {
    execute_rhai(document.getElementById('code').value);
});

document.getElementById('btn-clear').addEventListener('click', () => {
    document.getElementById('terminal-output').innerHTML = '';
});

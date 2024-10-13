// start: devtools
if (typeof require !== 'undefined') {
    const gui = require('nw.gui');
    const win = gui.Window.get();
    if (process.versions['nw-flavor'] === 'sdk') {
        if (win.isDevToolsOpen === undefined) win.isDevToolsOpen = false;
        win.showDevTools();
        win.isDevToolsOpen = true;
        window.addEventListener('keydown', function (event) {
            if (event.key === 'F11') {
                win.isDevToolsOpen ? win.closeDevTools() : win.showDevTools();
                win.isDevToolsOpen = !win.isDevToolsOpen; 
            }
        });
    } else {
        alert("NW.js SDK is not installed. Developer tools are unavailable.");
    }
} else {
    alert("This application is not running in a NW.js environment.");
}
// end: devtools
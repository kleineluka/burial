// determine os type on load and set footer hint
document.addEventListener('DOMContentLoaded', function() {
    // assume windows, i guess
    var os = 'windows'; 
    var platform = window.navigator.userAgent.toLowerCase();
    // determine os
    if (platform.indexOf('mac') > -1) {
        os = 'mac';
    } else if (platform.indexOf('linux') > -1 || platform.indexOf('x11') > -1) {
        os = 'linux';
    }
    // set footer inner html based on that
    var template = document.getElementById('footer').innerHTML; // replace %x% with os
    if (os === 'windows') {
        document.getElementById('footer').innerHTML = template.replace(/%x%/g, '%appdata%');
    } else {
        document.getElementById('footer').innerHTML = template.replace(/%x%/g, '.config');
    }
});

// write new settings
function saveSettings() {
    // get values
    var tcoaal = document.getElementById('tcoaal-path').value;
    var output = document.getElementById('output-path').value;
    // update settings in local storage
    const store = new Store('.cache.json');
    store.set('tcoaal', tcoaal);
    store.set('output', output);
    store.save();
    // set values
    invoke('save_settings', { tcoaal, output });
}

// reset button
function resetSettings() {
    // reset settings in local storage
    const store = new Store('.cache.json');
    store.set('tcoaal', '');
    store.set('output', '');
    store.save();
    // set the values to empty
    document.getElementById('tcoaal-path').value = '';
    document.getElementById('output-path').value = '';
    // set values
    invoke('reset_settings', {});
}

// listen for when the settings are saved
listen('settings-saved', (event) => {
    Swal.fire({
        title: "Settings saved!",
        toast: true,
        position: "bottom-right",
        showConfirmButton: true,
        confirmButtonText: "Yay!",
        timer: 2000,
    });
});

// listen for when the settings are reset
listen('settings-reset', (event) => {
    Swal.fire({
        title: "Your settings have been reset!",
        toast: true,
        position: "bottom-right",
        showConfirmButton: true,
        confirmButtonText: "Yay!",
        timer: 2000,
    });
});
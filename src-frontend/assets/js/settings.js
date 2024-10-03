// determine os type on load and set footer hint
document.addEventListener('DOMContentLoaded', async function() {
    // load the storage
    const storage = loadStorage();
    if (!storage) return;
    // set footer based on operating system
    let os = await storage.get('state-operating-system');
    var template = document.getElementById('footer').innerHTML; // replace %x% with os
    if (os === 'windows') {
        document.getElementById('footer').innerHTML = template.replace(/%x%/g, '%appdata%');
    } else {
        document.getElementById('footer').innerHTML = template.replace(/%x%/g, '.config');
    }
    // set footer based on version
    let version = await storage.get('state-local-version');
    document.getElementById('version').innerText = version;
});

// write new settings
function saveSettings() {
    // get values
    var tcoaal = document.getElementById('tcoaal-path').value;
    var output = document.getElementById('output-path').value;
    // update settings in local storage
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', tcoaal);
    store.set('settings-output', output);
    store.save();
    // set values
    invoke('save_settings', { tcoaal, output });
}

// reset button
function resetSettings() {
    // reset settings in local storage
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', '');
    store.set('settings-output', '');
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
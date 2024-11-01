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

// listen for click on advanced-button button
document.getElementById('advanced-button').addEventListener('click', function() {
    // toggle class hidden on main-settings and advanced-settings
    document.getElementById('main-settings').classList.toggle('hidden');
    document.getElementById('advanced-settings').classList.toggle('hidden');
});

// listen for click on main-button button
document.getElementById('main-button').addEventListener('click', function () {
    // toggle class hidden on main-settings and advanced-settings
    document.getElementById('main-settings').classList.toggle('hidden');
    document.getElementById('advanced-settings').classList.toggle('hidden');
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
    store.set('settings-hotload', document.getElementById('dropdown-menu-hotload').value);
    store.save();
    // set values
    let hotload = (document.getElementById('dropdown-menu-hotload').value === 'true');
    invoke('save_settings', { tcoaal, output, hotload});
}

// reset button
function resetSettings() {
    // reset settings in local storage
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', '');
    store.set('settings-output', '');
    store.set('settings-hotload', 'false');
    store.save();
    // set the values to empty
    document.getElementById('tcoaal-path').value = '';
    document.getElementById('output-path').value = '';
    document.getElementById('dropdown-menu-hotload').value = 'false';
    // set values
    invoke('reset_settings', {});
}

// listen for click on remove-deno-button
document.getElementById('remove-deno-button').addEventListener('click', function () {
    invoke('remove_deno', {});
});

// listen for click on remove-hausmaerchen-button
document.getElementById('remove-hausmaerchen-button').addEventListener('click', function () {
    invoke('remove_hausmaerchen', {});
});

// listen for click on install-dev-tools-button
document.getElementById('install-dev-tools-button').addEventListener('click', function () {
    invoke('install_dev_tools', {});
});

// listen for click on auto-button-tcoaal
document.getElementById('auto-button-tcoaal').addEventListener('click', function () {
    invoke('settings_auto_find', {});
});

// listen for game-path
listen('game-path', (event) => {
    if (event.payload != "empty") {
        let gamePath = event.payload;
        gamePath = gamePath.replace(/\\\\/g, '\\');
        document.getElementById('tcoaal-path').value = gamePath;
        Swal.fire({
            title: "TCOAAL Found!",
            text: "We autofilled it for you! :)",
            toast: true,
            position: "bottom-right",
            showConfirmButton: true,
            confirmButtonText: "Yay!",
            timer: 5000,
        });
    } else {
        Swal.fire({
            title: "TCOAAL wasn't found!",
            text: "Please try and locate the path manually.",
            toast: true,
            position: "bottom-right",
            showConfirmButton: true,
            confirmButtonText: "Oki..",
            timer: 5000,
        });
    }
});

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

// listen for when the deno is removed
listen('deno-removed', (event) => {
    Swal.fire({
        title: "Deno has been removed!",
        toast: true,
        position: "bottom-right",
        showConfirmButton: true,
        confirmButtonText: "Yay!",
        timer: 2000,
    });
});

// listen for when the hausmaerchen is removed
listen('hausmaerchen-removed', (event) => {
    Swal.fire({
        title: "Hausmärchen has been removed!",
        toast: true,
        position: "bottom-right",
        showConfirmButton: true,
        confirmButtonText: "Yay!",
        timer: 2000,
    });
});

// listen for when the dev tools are installed
listen('dev-tools-installed', (event) => {
    Swal.fire({
        title: "Dev tools have been installed!",
        toast: true,
        position: "bottom-right",
        showConfirmButton: true,
        confirmButtonText: "Yay!",
        timer: 2000,
    });
});
// determine os type on load and set footer hint
document.addEventListener('DOMContentLoaded', async function() {
    // load the storage
    const storage = loadStorage();
    if (!storage) return;
    // autofill other storage options
    const theme = await storage.get('settings-theme');
    // dropdown-menu-theme = select <select> element with theme
    document.getElementById('dropdown-menu-theme').value = theme;
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
async function saveSettings() {
    // get values
    var tcoaal = document.getElementById('tcoaal-path').value;
    var output = document.getElementById('output-path').value;
    var hotload = (document.getElementById('dropdown-menu-hotload').value === 'true');
    var theme = document.getElementById('dropdown-menu-theme').value;
    // update settings in local storage
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', tcoaal);
    store.set('settings-output', output);
    store.set('settings-hotload', hotload);
    store.set('settings-theme', theme);
    store.save();
    // set values
    invoke('save_settings', { tcoaal, output, hotload, theme });
    // reload theme
    document.documentElement.setAttribute('data-theme', theme);
}

// reset button
function resetSettings() {
    // reset settings in local storage
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', '');
    store.set('settings-output', '');
    store.set('settings-hotload', false);
    store.set('settings-theme', 'ashley');
    store.save();
    // set the values to empty
    document.getElementById('tcoaal-path').value = '';
    document.getElementById('output-path').value = '';
    document.getElementById('dropdown-menu-hotload').value = 'false';
    document.getElementById('dropdown-menu-theme').value = 'ashley';
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

// switch between horizontal navbars
document.addEventListener('DOMContentLoaded', () => {
    const navOptions = document.querySelectorAll('.page-navbar-option');
    const subContainers = document.querySelectorAll('.page-container');
    navOptions.forEach(option => {
        option.addEventListener('click', (event) => {
            event.preventDefault();
            // clear current selection
            navOptions.forEach(nav => nav.classList.remove('selected'));
            subContainers.forEach(container => container.classList.add('hidden'));
            // show what was selected
            option.classList.add('selected');
            const id = option.id;
            const subContainer = document.getElementById(`sub-${id}`);
            if (subContainer) {
                subContainer.classList.remove('hidden');
            }
        });
    });
});

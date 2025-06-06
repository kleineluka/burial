// determine os type on load and set footer hint
document.addEventListener('DOMContentLoaded', async function() {
    // load the storage
    const storage = loadStorage();
    if (!storage) return;
    // autofill other storage options
    const updates = await storage.get('settings-updates');
    document.getElementById('dropdown-menu-updates').value = updates.toString();
    const theme = await storage.get('settings-theme');
    document.getElementById('dropdown-menu-theme').value = theme;
    const animations = await storage.get('settings-animations');
    document.getElementById('dropdown-menu-animations').value = animations.toString();
    const tooltips = await storage.get('settings-tooltips');
    document.getElementById('dropdown-menu-tooltips').value = tooltips.toString();
    const modname = await storage.get('settings-modname');
    document.getElementById('mod-name').value = modname;
    const modid = await storage.get('settings-modid');
    document.getElementById('mod-id').value = modid;
    const modauthor = await storage.get('settings-modauthor');
    document.getElementById('mod-author').value = modauthor;
    const moddescription = await storage.get('settings-moddescription');
    document.getElementById('mod-description').value = moddescription;
    const deeplinks = await storage.get('settings-deeplinks');
    document.getElementById('dropdown-menu-deeplinks').value = deeplinks.toString();
    const gametarget = await storage.get('settings-gametarget');
    document.getElementById('dropdown-game-version').value = gametarget;
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
    var updates = (document.getElementById('dropdown-menu-updates').value === 'true');
    var theme = document.getElementById('dropdown-menu-theme').value;
    var animations = (document.getElementById('dropdown-menu-animations').value === 'true');
    var tooltips = (document.getElementById('dropdown-menu-tooltips').value === 'true');
    var modname = document.getElementById('mod-name').value;
    var modid = document.getElementById('mod-id').value;
    var modauthor = document.getElementById('mod-author').value;
    var moddescription = document.getElementById('mod-description').value;
    var deeplinks = (document.getElementById('dropdown-menu-deeplinks').value === 'true');
    var gametarget = document.getElementById('dropdown-game-version').value;
    // update settings in local storage
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', tcoaal);
    store.set('settings-output', output);
    store.set('settings-updates', updates);
    store.set('settings-theme', theme);
    store.set('settings-animations', animations);
    store.set('settings-tooltips', tooltips);
    store.set('settings-modname', modname);
    store.set('settings-modid', modid);
    store.set('settings-modauthor', modauthor);
    store.set('settings-moddescription', moddescription);
    store.set('settings-deeplinks', deeplinks);
    store.set('settings-gametarget', gametarget);
    store.save();   
    // set values
    invoke('save_settings', { tcoaal, output, updates, theme, animations, tooltips, modname, modid, modauthor, moddescription, deeplinks, gametarget });
    // reload theme
    document.documentElement.setAttribute('data-theme', theme);
    const imgs = document.querySelectorAll('img.sidebar-icon');
    imgs.forEach(img => {
        const newSrc = img.getAttribute(`data-${theme}`);
        if (newSrc) {
            img.src = newSrc;
        }
    });
    // reload animations
    if (!animations) {
        document.body.classList.add('disable-animations');
        const elements = document.querySelectorAll('[class*="hvr-"]');
        elements.forEach(element => {
            const classes = element.classList;
            for (let i = 0; i < classes.length; i++) {
                if (classes[i].includes('hvr-')) {
                    element.classList.remove(classes[i]);
                }
            }
        });
    } else {
        document.body.classList.remove('disable-animations');
    }
}

// reset button
function resetSettings() {
    // reset settings in local storage
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', '');
    store.set('settings-output', '');
    store.set('settings-updates', true);
    store.set('settings-theme', 'ashley');
    store.set('settings-animations', true);
    store.set('settings-tooltips', true);
    store.set('settings-modname', '');
    store.set('settings-modid', '');
    store.set('settings-modauthor', '');
    store.set('settings-moddescription', '');
    store.set('settings-deeplinks', true);
    store.set('settings-gametarget', 'latest');
    store.save();
    // set the values to empty
    document.getElementById('tcoaal-path').value = '';
    document.getElementById('output-path').value = '';
    document.getElementById('dropdown-menu-theme').value = 'ashley';
    document.getElementById('dropdown-menu-animations').value = 'true';
    document.getElementById('dropdown-menu-tooltips').value = 'true';
    document.getElementById('mod-name').value = '';
    document.getElementById('mod-id').value = '';
    document.getElementById('mod-author').value = '';
    document.getElementById('mod-description').value = '';
    document.getElementById('dropdown-menu-updates').value = 'true';
    document.getElementById('dropdown-menu-deeplinks').value = 'true';
    document.getElementById('game-version').value = 'latest';
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

// listen for click on remove-deeplinks-button
document.getElementById('remove-deeplinks-button').addEventListener('click', async function () {
    // ask if they want to disable it too
    Swal.fire({
        title: "Hey, listen!",
        text: "Do you also wanna disable deeplinks from installing in the future? You can always change this later!",
        showCancelButton: true,
        confirmButtonText: "Yes plz!",
        cancelButtonText: "No thanks..",
        confirmButtonColor: "var(--main-colour)",
    }).then((result) => {
        if (result.isConfirmed) {
            invoke('uninstall_deeplinks', { turnOff: true });
            const store = loadStorage();
            store.set('settings-deeplinks', false);
            store.save();
            document.getElementById('dropdown-menu-deeplinks').value = 'false';
        } else {
            invoke('uninstall_deeplinks', { turnOff: false });
        }
    });
});

// listen for deeplinks removed
listen('deeplinks-uninstalled', (event) => {
    Swal.fire({
        title: "Deeplinks have been removed!",
        toast: true,
        position: "bottom-right",
        showConfirmButton: true,
        confirmButtonText: "Yay!",
        confirmButtonColor: "var(--main-colour)",
        timer: 2000,
    });
});

// listen for click on auto-button-tcoaal
document.getElementById('auto-button-tcoaal').addEventListener('click', function () {
    invoke('settings_auto_find', {});
});

// listen for click on auto-button-output
document.getElementById('auto-button-output').addEventListener('click', function () {
    invoke('output_auto_find', {});
});

// listen for click on reset-mod-name
document.getElementById('reset-mod-name').addEventListener('click', function () {
    document.getElementById('mod-name').value = '';
});

// listen for click on reset-mod-id
document.getElementById('reset-mod-id').addEventListener('click', function () {
    document.getElementById('mod-id').value = '';
});

// listen for click on reset-mod-author
document.getElementById('reset-mod-author').addEventListener('click', function () {
    document.getElementById('mod-author').value = '';
});

// listen for click on reset-mod-description
document.getElementById('reset-mod-description').addEventListener('click', function () {
    document.getElementById('mod-description').value = '';
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
            confirmButtonColor: "var(--main-colour)",
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
            confirmButtonColor: "var(--main-colour)",
            timer: 5000,
        });
    }
});

// listen for output-path
listen('output-path', (event) => {
    if (event.payload != "empty") {
        let outputPath = event.payload;
        outputPath = outputPath.replace(/\\\\/g, '\\');
        document.getElementById('output-path').value = outputPath;
        Swal.fire({
            title: "Default Output Set!",
            text: "We autofilled a Burial folder in your documents!",
            toast: true,
            position: "bottom-right",
            showConfirmButton: true,
            confirmButtonText: "Yay!",
            confirmButtonColor: "var(--main-colour)",
            timer: 5000,
        });
    } else {
        Swal.fire({
            title: "Output wasn't found!",
            text: "Please try and locate the path manually.",
            toast: true,
            position: "bottom-right",
            showConfirmButton: true,
            confirmButtonText: "Oki..",
            confirmButtonColor: "var(--main-colour)",
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
        confirmButtonColor: "var(--main-colour)",
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
        confirmButtonColor: "var(--main-colour)",
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
        confirmButtonColor: "var(--main-colour)",
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
        confirmButtonColor: "var(--main-colour)",
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
        confirmButtonColor: "var(--main-colour)",
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

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#reset-button', {
        content: 'This will reset all of the settings, not just the ones you can currently see!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#updates-check-label', {
        content: 'Check for updates on launch',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#profiles-copy-label', {
        content: 'Profiles use a second copy of the game and don\'t affect the original',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#game-version-label', {
        content: 'The target version of the game you want to mod. If unknown, leave it as latest!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});

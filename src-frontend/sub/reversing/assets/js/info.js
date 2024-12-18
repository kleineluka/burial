// on load, try and get game information
document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const inPath = await store.get('settings-tcoaal');
    const silent = true;
    invoke("general_info", { inPath, silent }); 
    invoke("plugins_info", { inPath });
});

// add a listener to refresh-button that calls the general_info function
document.getElementById('refresh-button').addEventListener('click', () => {
    invoke("general_info", { inPath: document.getElementById('tcoaal-path').value, silent: false });
});

// listen for when the general information was returned
listen('general_info_loaded', (event) => {
    // dropdown-menu-game-version, dropdown-menu-mod-loader, dropdown-menu-sdk
    const gameVersionDropdown = document.getElementById('dropdown-menu-game-version');
    const modLoaderDropdown = document.getElementById('dropdown-menu-mod-loader');
    const sdkDropdown = document.getElementById('dropdown-menu-sdk');
    // clear the dropdowns
    gameVersionDropdown.innerHTML = '';
    modLoaderDropdown.innerHTML = '';
    sdkDropdown.innerHTML = '';
    // create the options
    const gameVersionOption = document.createElement('option');
    gameVersionOption.value = event.payload.game;
    gameVersionOption.text = event.payload.game;
    gameVersionDropdown.appendChild(gameVersionOption);
    const modLoaderOption = document.createElement('option');
    modLoaderOption.value = event.payload.modloader_presence;
    modLoaderOption.text = event.payload.modloader_presence;
    modLoaderDropdown.appendChild(modLoaderOption);
    const sdkOption = document.createElement('option');
    sdkOption.value = event.payload.sdk_presence;
    sdkOption.text = event.payload.sdk_presence;
    sdkDropdown.appendChild(sdkOption);
});

// listen for when the plugins information was returned
listen('plugins_info_loaded', (event) => {
    // if we could load the plugins, display them!
    if (event.payload) {
        const pluginsContainer = document.getElementById('plugins-container');
        pluginsContainer.innerHTML = '';
        for (const plugin of event.payload) {
            const pluginEntry = document.createElement('div');
            pluginEntry.classList.add('plugin-entry');
            const pluginText = document.createElement('p');
            let description = plugin.description;
            if (!description) {
                description = 'No description';
            }
            pluginText.innerHTML = `<b>${plugin.name}</b> <i>(${plugin.status ? 'Enabled' : 'Disabled'})</i>: ${description}`;
            pluginEntry.appendChild(pluginText);
            pluginsContainer.appendChild(pluginEntry);
        }   
    }
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
    tippy('#refresh-button', {
        content: 'Refresh information about your game (in case you changed something or updated the path).',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#game-version-label', {
        content: 'The internal version of TCOAAL in your installation.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#mod-loader-label', {
        content: 'Whether or not the Tomb modloader is installed. You can install this in Burial!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#sdk-branch-label', {
        content: 'Whether your TCOAAL has the player NWJS SDK or the developer NWJS SDK. TCOAAL comes with the player SDK, but you can use Burial to install the developer SDK.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
// on load, try and get game information
document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const inPath = await store.get('settings-tcoaal');
    const silent = true;
    invoke("general_info", { inPath, silent }); 
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
    gameVersionOption.value = event.payload.game_version;
    gameVersionOption.text = event.payload.game_version;
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
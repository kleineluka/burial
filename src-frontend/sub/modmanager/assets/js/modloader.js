// listen first
listen('modloader-version', (event) => {
    const dropdown = document.getElementById('dropdown-menu-current');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = event.payload;
    option.text = event.payload;
    dropdown.appendChild(option);
});

listen('modloader-versions', (event) => {
    const dropdown = document.getElementById('dropdown-menu-install'); 
    dropdown.innerHTML = '';
    // add all versions from remote to dropdown
    event.payload.forEach((version, index) => {
        const option = document.createElement('option');
        option.value = version;
        option.textContent = version;
        dropdown.appendChild(option);
    });
    // make a "Latest (recommended)" option and select it
    const option = document.createElement('option');
    option.value = 'latest';
    option.textContent = 'Latest (recommended)';
    dropdown.appendChild(option);
    dropdown.selectedIndex = dropdown.length - 1;
    // clear the status (in case the user tried to install before hand)
    clear_status();
});

document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const inPath = await store.get('settings-tcoaal');
    invoke("modloader_version", { inPath });
    invoke("modloader_versions", {});
});

// install selected version
function installSelected() {
    // make sure the length of the dropdown is greater than 1 (i.e. not loading)
    const dropdown = document.getElementById('dropdown-menu-install');
    if (dropdown.length > 1) {
        const inName = document.getElementById('dropdown-menu-install').value;
        const inPath = document.getElementById('tcoaal-path').value;
        invoke("install_modloader", { inPath, inName } );
    } else {
        set_status('Please wait for the Tomb versions to load!');
    }
}

// uninstall mod loader
function uninstallModloader() {
    const inPath = document.getElementById('tcoaal-path').value;
    invoke("uninstall_modloader", { inPath } );
}

// refresh
function refreshLocal() {
    const dropdown = document.getElementById('dropdown-menu-current');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = 'Loading...';
    option.text = 'Loading...';
    dropdown.appendChild(option);
    const inPath = document.getElementById('tcoaal-path').value;
    invoke("modloader_version", { inPath });
}

function refreshRemote() {
    const dropdown = document.getElementById('dropdown-menu-install');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = 'Loading...';
    option.text = 'Loading...';
    dropdown.appendChild(option);
    invoke("modloader_versions", { });
}

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#available-label', {
        content: 'It is always suggested to let Burial download the latest!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#refresh-local-button', {
        content: 'Refresh what version is installed in your TCOAAL.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#refresh-remote-button', {
        content: 'Refresh the list of available versions.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
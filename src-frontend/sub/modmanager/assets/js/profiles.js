let installed_cache = null;
let inPath = null;
let currentProfile = null;
let copyVersion = 'notset';

// put the json into an html list
async function build_profile_list() {
    const store = loadStorage();
    const container = document.querySelector(".mods-container");
    const children = Array.from(container.children);
    children.forEach(child => {
        if (child.id !== 'installation-notification-container') { // keep notification if needed
            child.remove();
        }
    });
    let added_mods = 0;
    installed_cache.forEach(entry => {
        // create mod container
        const modEntry = document.createElement('div');
        modEntry.classList.add('mod-entry');
        modEntry.dataset.id = entry.modjson.id || 'unknown';
        // first row: icon, details, and actions
        const firstRow = document.createElement('div');
        firstRow.classList.add('mod-row');
        // icon
        const iconDiv = document.createElement('div');
        iconDiv.classList.add('mod-icon');
        const iconImg = document.createElement('img');
        iconImg.src = entry.modjson.icon || 'assets/img/default.png';
        iconImg.alt = 'Mod Icon';
        iconImg.classList.add('mod-provided-icon', 'hvr-shrink');
        iconDiv.appendChild(iconImg);
        // details
        const detailsDiv = document.createElement('div');
        detailsDiv.classList.add('mod-details');
        const nameHeading = document.createElement('h3');
        nameHeading.classList.add('mod-name');
        let nameHTML = `${entry.modjson.name || 'Unnamed Mod'}`;
        nameHeading.innerHTML = nameHTML;
        const description = document.createElement('p');
        description.classList.add('mod-description');
        description.textContent = entry.modjson.description || 'No description provided';
        detailsDiv.appendChild(nameHeading);
        detailsDiv.appendChild(description);
        // actions (depending on the profile and mod)
        const actionsDiv = document.createElement('div');
        actionsDiv.classList.add('mod-actions');
        const currentProfile = document.getElementById('dropdown-menu-current-profile').value;
        if (currentProfile === 'Default') {
            // action to display: notice
            const noticeIcon = document.createElement('img');
            noticeIcon.src = 'assets/img/warning.png';
            noticeIcon.alt = 'Notice Button';
            noticeIcon.classList.add('mod-download-icon', 'hvr-shrink', "bypass-disabled");
            actionsDiv.appendChild(noticeIcon);
            tippy(noticeIcon, {
                content: 'You cannot edit mods in the default profile!',
                animation: 'perspective-subtle',
                placement: 'left',
                theme: 'burial'
            });
        } else {
            if (entry.modjson.id !== 'tomb') {
                // action to display: toggle
                const toggleIcon = document.createElement('img');
                toggleIcon.src = 'assets/img/disable.png';
                toggleIcon.alt = 'Disable Button';
                toggleIcon.classList.add('mod-download-icon', 'hvr-shrink', "bypass-disabled");
                actionsDiv.appendChild(toggleIcon);
                tippy(toggleIcon, {
                    content: 'Toggle the mod for this profile.',
                    animation: 'perspective-subtle',
                    placement: 'left',
                    theme: 'burial'
                });
                toggleIcon.addEventListener('click', async () => {
                    console.log('Toggling mod:', entry.modjson.name);
                });
            } else {
                // action to display: notice
                const noticeIcon = document.createElement('img');
                noticeIcon.src = 'assets/img/warning.png';
                noticeIcon.alt = 'Notice Button';
                noticeIcon.classList.add('mod-download-icon', 'hvr-shrink', "bypass-disabled");
                actionsDiv.appendChild(noticeIcon);
                tippy(noticeIcon, {
                    content: 'Tomb must be enabled in every profile.',
                    animation: 'perspective-subtle',
                    placement: 'left',
                    theme: 'burial'
                });
            }
        }
        // finish first row
        firstRow.appendChild(iconDiv);
        firstRow.appendChild(detailsDiv);
        firstRow.appendChild(actionsDiv);
        // put it all together
        modEntry.appendChild(firstRow);
        container.appendChild(modEntry);
        // grey out if tomb
        if (currentProfile === 'Default') {
            console.log('Default profile, disabling all');
            modEntry.classList.add('disabled');
        } else if (entry.modjson.id === 'tomb') {
            modEntry.classList.add('disabled');
        }
        added_mods++;
    });
    if (added_mods == 0) {
        const noMods = document.createElement('div');
        noMods.classList.add('loading');
        noMods.textContent = 'Hmm.. it doesn\'t look like you have any mods installed yet. Why not try installing some?';
        container.appendChild(noMods);
    }
}

// move to function to make it reusable (ex. after mod is installed)
async function load_installed() {
    const store = loadStorage();
    inPath = await store.get('settings-tcoaal');
    invoke('installed_mods', { inPath });
}

// on page load see what mods are installed
window.addEventListener('load', async () => {
    load_installed();
});

// update what mods are already installed
listen('installed-mods', async (event) => {
    if (event.payload === "error_modloader") {
        installed_cache = [];
    } else {
        installed_cache = event.payload;
    }
    await download_repo(); // avoid redownloading
    await download_foreign(); // avoid redownloading
    combine_jsons(); // build them together (bleh, they need to just use tomb..)
    build_profile_list();
});

// listen for click on installation-notification-container
document.getElementById('installation-notification-container').addEventListener('click', async () => {
    const store = loadStorage();
    const inPath = await store.get('settings-tcoaal');
    invoke('profiles_install', { inPath });
});

// we got the installation !
listen('profiles-installed', async (event) => {
    console.log('Installed:', event.payload);
});

// on load, get the profiles and set the current one
document.addEventListener('DOMContentLoaded', async () => {
    let inPath = await loadStorage().get('settings-tcoaal');
    invoke('load_profiles', { inPath });
});

// we got the profiles !
listen('profiles-loaded', async (event) => {
    const profiles_json = JSON.parse(event.payload);
    const profiles = profiles_json.profiles;
    currentProfile = profiles_json.current;
    copyVersion = profiles_json.version;
    const dropdown = document.getElementById('dropdown-menu-current-profile');
    dropdown.innerHTML = '';
    profiles.forEach(profile => {
        const option = document.createElement('option');
        option.value = profile.name;
        option.textContent = profile.name;
        if (profile === currentProfile) {
            option.selected = true;
        }
        dropdown.appendChild(option);
    });
});


// listen for the click on add button
document.getElementById('add-button').addEventListener('click', async () => {
    // ask what name htey want to use for it
    Swal.fire({
        title: 'What do you wanna call it?',
        input: 'text',
        closeOnConfirm: true,
        reverseButtons: true,
        confirmButtonText: 'Create Profile',
        confirmButtonColor: "var(--main-colour)"
    }).then(async (result) => {
        if (result.value) {
            const dropdown = document.getElementById('dropdown-menu-current-profile');
            const profileName = result.value;
            let inPath = await loadStorage().get('settings-tcoaal');
            invoke('profile_add', { inPath, profileName });
        }
    });
});

// listen for the click on remove button
document.getElementById('delete-button').addEventListener('click', async () => {
    const dropdown = document.getElementById('dropdown-menu-current-profile');
    const profileName = dropdown.value;
    // if current is not set, don't do anything
    if (profileName === '' || profileName === 'Loading') {
        return;
    }
    let inPath = await loadStorage().get('settings-tcoaal');
    invoke('profile_delete', { inPath, profileName });
});

// we deleted a profile
listen('profile-deleted', async (event) => {
    if (event.payload === 'default') {
        Swal.fire({
            title: "Wait!",
            text: "You can't delete the default profile. If you want to edit the mods in it, just uninstall or disable them in the Installed Mods  tab.",
            showConfirmButton: true,
            confirmButtonColor: '#F595B2'
        });
        return;
    }
    set_status('Profile deleted!');
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#dropdown-menu-current-profile', {
        content: 'Change your current profile',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#add-button', {
        content: 'Create a new profile',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#delete-button', {
        content: 'Delete your selected profile',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#add-button', {
        content: 'Create a new profile',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#play-button', {
        content: 'Launch the game with this profile',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#installation-notification-container', {
        content: 'Set up your profile\'s installation folder',
        animation: 'installation-notification-container',
        placement: 'left',
        theme: 'burial'
    });
});
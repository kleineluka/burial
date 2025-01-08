let installed_cache = null;
let search_cache = null;
let mod_ready = 'ready';
let inPath = null;
let fuse = null;

// sort the data by modJson.name alphabetically
function sort_alphabetically() {
    combined_data.sort((a, b) => {
        const a_name = a.modJson.name || 'Unnamed Mod';
        const b_name = b.modJson.name || 'Unnamed Mod';
        return a_name.localeCompare(b_name);
    }); 
}

// sort the data by data.lastUpdate (newest dates first)
function sort_updated() {
    combined_data.sort((a, b) => {
        const dateA = new Date(a.data?.lastUpdate || 0);
        const dateB = new Date(b.data?.lastUpdate || 0);
        return dateB - dateA; 
    });
}

// build search cache for fuse.js (support both local and foreign)
function build_search_cache() {
    search_cache = {};
    combined_data.forEach(entry => {
        search_cache[entry.data.id] = true;
    });
    const options = {
        keys: ["modJson.name"],
        threshold: 0.1
    };
    fuse = new Fuse(combined_data, options);
}

// build the repository
function build_repo(sort_kind, filter_kind) {
    // sort the data before going through it
    if (sort_kind === 'name') {
        sort_alphabetically();
    } else if (sort_kind === 'date') {
        sort_updated();
    }
    // go through the data and populate the html
    const container = document.querySelector(".mods-container");
    container.innerHTML = "";
    let added_mods = 0;
    combined_data.forEach(entry => {
        // get mod data
        const initialData = entry.data || {};
        const modData = entry.modJson || {};
        const burialData = entry.burial || {};
        // first we need to get the tags and see if we are filtering for anything
        if (filter_kind && filter_kind !== 'all') {
            if (!initialData.tags || !initialData.tags.includes(filter_kind)) {
                return;
            }
        }
        // see if the search query is in the cache
        if (!search_cache[initialData.id]) return;
        // see if the mod is already installed
        let is_installed = (installed_cache[modData.id]) ? true : false;
        let is_old_version = false;
        if (is_installed) {
            const installed_version = installed_cache[modData.id].version
            const latest_version = modData.version;
            is_old_version = (installed_version !== latest_version);
        }
        // determine if this mod is a foreign mod (has foreign tag)
        let is_foreign = false;
        if (initialData.tags && initialData.tags.includes('foreign')) {
            is_foreign = true;
        }
        // create mod container
        const modEntry = document.createElement('div');
        modEntry.classList.add('mod-entry');
        modEntry.dataset.id = initialData.id || 'unknown';
        // first row: icon, details, and actions
        const firstRow = document.createElement('div');
        firstRow.classList.add('mod-row');
        // icon
        const iconDiv = document.createElement('div');
        iconDiv.classList.add('mod-icon');
        const iconImg = document.createElement('img');
        iconImg.src = modData.icon;
        if (!modData.icon) {
            if (is_foreign) {
                iconImg.src = 'assets/img/foreign.png';
            } else {
                iconImg.src = 'assets/img/default.png';
            }
        }
        iconImg.alt = 'Mod Icon';
        iconImg.classList.add('mod-provided-icon', 'hvr-shrink');
        iconDiv.appendChild(iconImg);
        // details
        const detailsDiv = document.createElement('div');
        detailsDiv.classList.add('mod-details');
        const nameHeading = document.createElement('h3');
        nameHeading.classList.add('mod-name');
        let nameHTML = `
        ${modData.name || 'Unnamed Mod'}
        <span class="mod-author">(by ${modData.authors ? modData.authors.join(', ') : 'Unknown Author'}) [v${modData.version || '0.0'}]</span>
        `;
        if (initialData.source) {
            nameHTML += `
                <a href="${initialData.source}" target="_blank" class="mod-open-link-symbol hvr-grow" title="View Source">🔗</a>
            `;
        }
        if (is_foreign) {
            nameHTML += `
                <a onclick="notifyForeign()" target="_blank" class="mod-open-link-symbol hvr-grow" title="WARNING! (click to learn more)">⚠️</a>
            `;
        }
        nameHeading.innerHTML = nameHTML;
        const description = document.createElement('p');
        description.classList.add('mod-description');
        description.textContent = modData.description || 'No description provided';
        // installed notice 
        if (is_installed) {
            const installedText = document.createElement('span');
            installedText.classList.add('mod-installed-text');
            installedText.textContent = ` You have version ${installed_cache[modData.id].version} installed.`;
            if (is_old_version) {
                installedText.textContent += ' A newer version is available!';
            }
            description.appendChild(installedText);
        }
        detailsDiv.appendChild(nameHeading);
        detailsDiv.appendChild(description);
        // actions
        const actionsDiv = document.createElement('div');
        actionsDiv.classList.add('mod-actions');
        if (is_installed) {
            if (is_old_version) {
                // action to display: update
                const updateIcon = document.createElement('img');
                updateIcon.src = 'assets/img/update.png';
                updateIcon.alt = 'Download Button';
                updateIcon.classList.add('mod-download-icon', 'hvr-shrink');
                actionsDiv.appendChild(updateIcon);
                // on update click
                updateIcon.addEventListener('click', async () => {
                    console.log('Downloading (and updating) mod:', modData.name);
                    const store = loadStorage();
                    const inPath = await store.get('settings-tcoaal');
                    const modPath = initialData.url || 'unknown_name';
                    const modHash = initialData.sha256 || 'unknown_hash';
                    const modTags = initialData.tags || ['No Tags Yet'];
                    const modJson = combined_data.find(entry => entry.data.id === initialData.id).modJson;
                    invoke('install_mod', { inPath, modPath, modHash, modTags, modJson });
                });
            } 
            // branch based on whether mods enabled
            let is_enabled = installed_cache[modData.id].status || false;
            if (is_enabled) {
                // action to display: disable
                const disableIcon = document.createElement('img');
                disableIcon.src = 'assets/img/disable.png';
                disableIcon.alt = 'Disable Button';
                disableIcon.classList.add('mod-download-icon', 'hvr-shrink');
                actionsDiv.appendChild(disableIcon);
                tippy(disableIcon, {
                    content: 'Disable the mod!',
                    animation: 'perspective-subtle',
                    placement: 'left',
                    theme: 'burial'
                });
                disableIcon.addEventListener('click', async () => {
                    console.log('Disabling mod:', modData.name);
                    const inPath = installed_cache[modData.id].path;
                    invoke('disable_mod', { inPath });
                });
            } else {
                // action to display: enable
                const enableIcon = document.createElement('img');
                enableIcon.src = 'assets/img/enable.png';
                enableIcon.alt = 'Enable Button';
                enableIcon.classList.add('mod-download-icon', 'hvr-shrink');
                actionsDiv.appendChild(enableIcon);
                tippy(enableIcon, {
                    content: 'Enable the mod!',
                    animation: 'perspective-subtle',
                    placement: 'left',
                    theme: 'burial'
                });
                enableIcon.addEventListener('click', async () => {
                    console.log('Enabling mod:', modData.name);
                    const inPath = installed_cache[modData.id].path;
                    const gamePath = await loadStorage().get('settings-tcoaal');
                    invoke('enable_mod', { inPath, gamePath });
                });
            }
            // action to display: delete
            const deleteIcon = document.createElement('img');
            deleteIcon.src = 'assets/img/delete.png';
            deleteIcon.alt = 'Delete Button';
            deleteIcon.classList.add('mod-download-icon', 'hvr-shrink');
            actionsDiv.appendChild(deleteIcon);
            tippy(deleteIcon, {
                content: 'Delete the mod!',
                animation: 'perspective-subtle',
                placement: 'left',
                theme: 'burial'
            });
            deleteIcon.addEventListener('click', async () => {
                console.log('Deleting mod:', modData.name);
                const modPath = installed_cache[modData.id].path;
                invoke('uninstall_mod', { modPath });
            });
        } else {
            // action to display: download
            const downloadIcon = document.createElement('img');
            downloadIcon.src = 'assets/img/download.png';
            downloadIcon.alt = 'Download Button';
            downloadIcon.classList.add('mod-download-icon', 'hvr-shrink');
            actionsDiv.appendChild(downloadIcon);
            // add tooltip
            tippy(downloadIcon, {
                content: 'Download the mod!',
                animation: 'perspective-subtle',
                placement: 'left',
                theme: 'burial'
            });
            // on download click
            downloadIcon.addEventListener('click', async () => {
                console.log('Downloading mod:', modData.name);
                const store = loadStorage();
                const inPath = await store.get('settings-tcoaal');
                const modPath = initialData.url || 'unknown_name';
                const modHash = initialData.sha256 || 'unknown_hash';
                const modTags = initialData.tags || ['No Tags Yet'];
                const modJson = combined_data.find(entry => entry.data.id === initialData.id).modJson;
                invoke('install_mod', { inPath, modPath, modHash, modTags, modJson });
            });
        }
        // finish first row
        firstRow.appendChild(iconDiv);
        firstRow.appendChild(detailsDiv);
        firstRow.appendChild(actionsDiv);
        // second row: dynamically add tags
        const secondRow = document.createElement('div');
        secondRow.classList.add('mod-row');
        const tagsDiv = document.createElement('div');
        tagsDiv.classList.add('mod-tags');
        (initialData.tags || ['No Tags Yet']).forEach(tag => {
            const tagSpan = document.createElement('span');
            tagSpan.classList.add('mod-tag', 'hvr-shrink');
            let tag_display = filter_tags(tag);
            tagSpan.textContent = tag_display;
            tagsDiv.appendChild(tagSpan);
        });
        // and before we append the tags, also add the source repo it is from
        const sourceRepoSpan = document.createElement('span');
        sourceRepoSpan.classList.add('mod-tag', 'hvr-shrink');
        let source_display = 'Hosted on: ' + filter_source(burialData.source_url);
        sourceRepoSpan.textContent = source_display;
        tagsDiv.appendChild(sourceRepoSpan);
        secondRow.appendChild(tagsDiv);
        // put it all together
        modEntry.appendChild(firstRow);
        modEntry.appendChild(secondRow);
        container.appendChild(modEntry);
        added_mods++;
    });
    // cute little message to say "hey, it isn't broken!"
    if (added_mods == 0) {
        const noMods = document.createElement('div');
        noMods.classList.add('loading');
        noMods.textContent = 'No mods found for this search criteria.. maybe yours can be the first?';
        container.appendChild(noMods);
    }
}

// reload the mod list
listen('refresh-mods', async (event) => {
    // reload the browser
    load_browser();
});

// on sort dropdown update 
const sortDropdown = document.querySelector('#sortDropdown');
sortDropdown.addEventListener('change', async () => {
    const sortKind = sortDropdown.value;
    const filterKind = document.querySelector('#filterDropdown').value;
    build_repo(sortKind, filterKind); 
    const scrollContainer = document.querySelector('.mods-container');
    scrollContainer.scrollTop = 0;
});

// on filter update
const filterDropdown = document.querySelector('#filterDropdown');
filterDropdown.addEventListener('change', async () => {
    const sortKind = document.querySelector('#sortDropdown').value;
    const filterKind = filterDropdown.value;
    const searchQuery = document.querySelector('#searchBar').value;
    build_repo(sortKind, filterKind, searchQuery); 
    const scrollContainer = document.querySelector('.mods-container');
    scrollContainer.scrollTop = 0;
});

// on search update
const searchBar = document.querySelector('#searchBar');
searchBar.addEventListener('input', async () => {
    if (!searchBar.value || searchBar.value === '') {
        combined_data.forEach(entry => {
            search_cache[entry.data.id] = true;
        });
    } else {
        const query = searchBar.value;
        const results = fuse.search(query);
        const ids = results.map(result => result.item.data.id);
        combined_data.forEach(entry => {
            const id = entry.data.id;
            if (ids.includes(id)) {
                search_cache[id] = true;
            } else {
                search_cache[id] = false;
            }
        });
    }
    const sortKind = document.querySelector('#sortDropdown').value;
    const filterKind = document.querySelector('#filterDropdown').value;
    build_repo(sortKind, filterKind);
});

// listen for mod ready statuses
listen('mod-ready', (event) => {
    let status_message = 'all good :)';
    switch (event.payload) {
        case "error_game_path":
            status_message = 'Please set your TCOAAL game path in settings!';
            set_status(status_message);
            mod_ready = status_message;
            break;
        case "error_modloader":
            status_message = 'Please install the Tomb modloader first, or use the Mod Pack page to do it all for you!';
            set_status(status_message);
            mod_ready = status_message;
            break;
        default:
            // success, assuming.. nothing for now
            break;
    }
});

// move to function to make it reusable (ex. after mod is installed)
async function load_browser() {
    const store = loadStorage();
    inPath = await store.get('settings-tcoaal');
    invoke('installed_mods', { inPath });
}

// on page load see what mods are installed
window.addEventListener('load', async () => {
    load_browser();
});

// when a mod is installed
listen('mod-install', async (event) => {
    // branch based on response
    switch (event.payload) {
        case "error_game_path":
            set_status('Please set your TCOAAL game path in settings!');
            break;
        case "error_modloader":
            set_status('Please install the Tomb modloader first, or use the Mod Pack page to do it all for you!');
            break;
        case "error_connection":
            set_status('Failed to connect to the mod\'s host..');
            break;
        case "error_file_open":
            set_status('Failed to open the mod file..');
            break;
        case "error_hash_mismatch":
            set_status('Failed to verify the hash of the mod file..');
            break;
        default:
            // success
            set_status('Mod installed successfully!');
            break
    }
    // reload the browser
    load_browser();
});

// update what mods are already installed
listen('installed-mods', async (event) => {
    if (event.payload === 'error_modloader') {
        // make installed_cache an empty object
        installed_cache = {};
    } else {
        // simplify the installed mods data for easier searching
        installed_cache = event.payload.reduce((acc, mod) => {
            const { id, version, status } = mod.modjson;
            acc[id] = {
                path: mod.folder,
                version: version,
                status: status
            };
            return acc;
        }, {});
    }
    // download + build repo (regardless of installed status)
    invoke('mod_ready', { inPath });
    await download_repo(); // avoid redownloading
    await download_foreign(); // avoid redownloading
    combine_jsons(); // build them together (bleh, they need to just use tomb..)
    build_search_cache(); // build the search cache
    build_repo('name', 'all');
    // print all json
    console.log('Installed Mods:', installed_cache);
    console.log('Combined Data:', combined_data);
    console.log('Search Cache:', search_cache);
});

// third-party mod warning
function notifyForeign() {
    Swal.fire({
        title: 'This mod is not hosted on the official Llamaawa.re repository.',
        text: 'Please be cautious when downloading and installing it. You may inspect the code yourself if you want. We do not vet third-party mods or gurantee their safety. This mod will automatically be translated to Tomb, so it may take more time to install or it may break. The developer of this mod may convert their mod to Tomb Modloader if they wish - feel free to ask them to so that TCOAAL modding can be more accessible!',
        closeOnConfirm: true,
        reverseButtons: true,
        confirmButtonText: 'I Understood',
        confirmButtonColor: "var(--main-colour)",
    });
}

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
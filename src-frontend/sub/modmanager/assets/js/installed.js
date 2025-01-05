let installed_cache = null;
let inPath = null;

// open folder
function openFolder(inPath) {
    console.log('Opening folder:', inPath); 
    invoke('open_folder', { inPath });
}

// pwetty.. (but optional!)
function rainbowify(text, offset) {
    const colors = ['#CC4C4C', '#CC744C', '#CC9F4C', '#4CCC4C', '#4C88CC', '#774CCC', '#CC4C88'];
    let rainbowText = '';
    for (let i = 0; i < text.length; i++) {
        const colorIndex = (i + offset) % colors.length;
        rainbowText += `<span style="color:${colors[colorIndex]}">${text[i]}</span>`;
    }
    return rainbowText;
}

// put the json into an html list
async function build_list() {
    // need to see if animations are enabled..
    const store = loadStorage();
    const animations = await store.get('settings-animations');
    // go through the data and populate the html
    const container = document.querySelector(".mods-container");
    container.innerHTML = "";
    let added_mods = 0;
    installed_cache.forEach(entry => {
        // check if there is a matching entry in the repo
        let update_available = false;
        if (combined_status) {
            const repoEntry = combined_data.find(repoEntry => repoEntry.modJson.id === entry.modjson.id);
            if (repoEntry) {
                // see if there is nau pdate
                if (repoEntry.modJson.version !== entry.modjson.version) {
                    update_available = true;
                }
                // replace the tags with the repo tags, since they aren't stored locally
                entry.modjson.tags = repoEntry.data.tags;
            }
        }
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
        let nameHTML = `
        ${entry.modjson.name || 'Unnamed Mod'}
        <span class="mod-author">(by ${entry.modjson.authors ? entry.modjson.authors.join(', ') : 'Unknown Author'}) [v${entry.modjson.version || '0.0'}]</span>
        `;
        // open local folder
        entry.folder = entry.folder.replace(/\\/g, '/').replace(/\/+/g, '/');
        nameHTML += `
                <a onclick="openFolder('${entry.folder}')" class="mod-open-link-symbol hvr-grow" title="View Source">📂</a>
            `;
        nameHeading.innerHTML = nameHTML;
        const description = document.createElement('p');
        description.classList.add('mod-description');
        description.textContent = entry.modjson.description || 'No description provided';
        // installed notice 
        const installedText = document.createElement('span');
        installedText.classList.add('mod-installed-text');
        if (entry.modjson.id !== 'tomb') {
            if (entry.modjson.status) {
                installedText.textContent = ` You have version ${entry.modjson.version} installed.`;
            } else {
                installedText.textContent = ` This mod is disabled. You can enable it again by hitting the button to the right.`;
            } 
        } else {
                installedText.textContent = `. Tomb can be updated and managed in the Mod Loader 🪦 tab.`;
        }
        description.appendChild(installedText);
        if (update_available && entry.modjson.status) {
            const updateAvailable = document.createElement('span');
            if (animations) {
                let updateRainbow = 'update';
                let rainbowOffset = 0;
                setInterval(() => {
                    let rainbowContainer = rainbowify(updateRainbow, rainbowOffset);
                    rainbowOffset = (rainbowOffset - 1 + updateRainbow.length) % updateRainbow.length;
                    updateAvailable.innerHTML = ' By the way, there is an ' + rainbowContainer + ' available!';
                }, 150); 
            } else {
                updateAvailable.innerHTML = ' By the way, there is an update available!';
            }    
            updateAvailable.classList.add('mod-installed-text');
            description.appendChild(updateAvailable);
        }
        detailsDiv.appendChild(nameHeading);
        detailsDiv.appendChild(description);
        // actions
        const actionsDiv = document.createElement('div');
        actionsDiv.classList.add('mod-actions');
        if (update_available && entry.modjson.status) {
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
                const sanitizedName = modData.name.replace(/[^a-zA-Z0-9]/g, '_');
                const modJson = combined_data.find(entry => entry.data.id === initialData.id).modJson;
                invoke('install_mod', { inPath, modPath, modHash, modTags, sanitizedName, modJson });
            });
        }
        if (entry.modjson.id !== 'tomb') {
            if (entry.modjson.status) {
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
                    console.log('Disabling mod:', entry.modjson.name);
                    const inPath = entry.folder;
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
                    console.log('Enabling mod:', entry.modjson.name);
                    const inPath = entry.folder;
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
            // add tooltip
            tippy(deleteIcon, {
                content: 'Delete the mod!',
                animation: 'perspective-subtle',
                placement: 'left',
                theme: 'burial'
            });
            // on delete click
            deleteIcon.addEventListener('click', async () => {
                console.log('Deleting mod:', entry.modjson.name);
                const modPath = entry.folder;
                invoke('uninstall_mod', { modPath });
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
        (entry.modjson.tags || ['No Tags Yet']).forEach(tag => {
            const tagSpan = document.createElement('span');
            tagSpan.classList.add('mod-tag', 'hvr-shrink');
            let tag_display = filter_tags(tag);
            tagSpan.textContent = tag_display;
            tagsDiv.appendChild(tagSpan);
        });
        secondRow.appendChild(tagsDiv);
        // put it all together
        modEntry.appendChild(firstRow);
        modEntry.appendChild(secondRow);
        container.appendChild(modEntry);
        // finally, add a filter if the mod is disabled
        if (!entry.modjson.status) {
            modEntry.classList.add('disabled-mod');
        }
        added_mods++;
    });
    if (added_mods == 0) {
        //<div class="loading">Loading the mod repository<span class="dots"></span></div>
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
    build_list();
});

// add a click event to "refresh-mods" button
document.querySelector('#refresh-mods').addEventListener('click', async () => {
    load_installed();
    set_status("Mods refreshed!");
});

// listen for refresh-mods
listen('refresh-mods', async () => {
    load_installed();
    set_status("Mods refreshed!");
});


// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#update-mods', {
        content: 'Download the latest version of all your mods - this may cause breaking changes!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
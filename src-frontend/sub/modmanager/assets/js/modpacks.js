let modpack_data = [];
let modpack_status = null;

// on button click, gather required mod downloads to send..
function install_modpack(inPath, modpack, packName) {
    // for each mod in the modpack, we need to find that mod in the combined_data
    let modEntries = [];
    modpack.mods.forEach(modId => {
        const modEntry = combined_data.find(entry => entry.data.id === modId);
        modEntries.push(modEntry);
    });
    let modpackMods = [];
    modEntries.forEach(modEntry => {
        // build the info we need for each mod in the modpack
        let mod_name = modEntry.data.id;
        let mod_sha256 = modEntry.data.sha256 || 'unknown_hash';
        let mod_tags = modEntry.data.tags || ['No Tags Yet'];
        let mod_url = modEntry.data.url;
        let modPackMod = {
            name: mod_name,
            sha256: mod_sha256,
            tags: mod_tags,
            modJson: modEntry.modJson,
            modUrl: mod_url
        };
        modpackMods.push(modPackMod);
    });
    // set the modpack.mods to the modpackMods
    modpack.mods = modpackMods;
    // add a value to modpack.name  
    modpack.name = packName;
    // call the back-end
    invoke('install_modpack', { inPath, modpackEntry: modpack });
}

// build the modpack repository
function build_modpack_repo() {
    // populate the modpacks ui
    const container = document.querySelector(".mods-container");
    container.innerHTML = "";
    Object.entries(modpack_data).forEach(([packName, pack]) => {
        // create the entry
        const modpackEntry = document.createElement('div');
        modpackEntry.classList.add('modpack-entry');
        const firstRow = document.createElement('div');
        firstRow.classList.add('modpack-row');
        // modpack icon
        const iconDiv = document.createElement('div');
        iconDiv.classList.add('modpack-icon');
        const iconImg = document.createElement('img');
        iconImg.src = 'assets/img/default.png';
        if (pack.icon !== null 
            && pack.icon !== undefined 
            && pack.icon !== 'default') {
            iconImg.src = pack.icon;
        }
        iconImg.alt = 'Modpack Icon';
        iconImg.classList.add('modpack-provided-icon', 'hvr-shrink');
        iconDiv.appendChild(iconImg);
        // modpack details
        const detailsDiv = document.createElement('div');
        detailsDiv.classList.add('modpack-details');
        const nameHeading = document.createElement('h3');
        nameHeading.classList.add('modpack-name');
        nameHeading.textContent = `${packName}`;
        const nameTimestamp = document.createElement('span');
        nameTimestamp.classList.add('modpack-subtitle');
        nameTimestamp.textContent = `(Updated: ${new Date(pack.lastUpdate).toLocaleDateString()})`;
        nameHeading.appendChild(nameTimestamp);
        const modCount = Object.keys(pack.mods).length;
        const description = document.createElement('p');
        description.classList.add('modpack-description');
        description.textContent = `${pack.description} ${modCount} mod${modCount > 1 ? 's' : ''} included in this pack - click to expand.`;
        detailsDiv.appendChild(nameHeading);
        detailsDiv.appendChild(description);
        // download the modpack
        const actionsDiv = document.createElement('div');
        actionsDiv.classList.add('modpack-actions');
        const installButton = document.createElement('img');
        installButton.src = 'assets/img/download.png';
        installButton.classList.add('modpack-download-icon', 'hvr-shrink');
        installButton.addEventListener('click', async () => {
            let inPath = await loadStorage().get('settings-tcoaal');
            install_modpack(inPath, pack, packName);
        });
        actionsDiv.appendChild(installButton);
        // build the first row
        firstRow.appendChild(iconDiv);
        firstRow.appendChild(detailsDiv);
        firstRow.appendChild(actionsDiv);
        // expandable mods list
        const modsList = document.createElement('div');
        modsList.classList.add('modpack-mods-list', 'hidden');
        pack.mods.forEach(modId => {
            const modEntry = document.createElement('p');
            modEntry.textContent = `• ${modId}`;
            modEntry.classList.add('modpack-mods-row');
            modsList.appendChild(modEntry);
        });
        // toggle mods list
        actionsDiv.addEventListener('click', (event) => {
            event.stopPropagation(); 
        });
        firstRow.addEventListener('click', () => {
            modsList.classList.toggle('hidden');
        });
        // add rows
        modpackEntry.appendChild(firstRow);
        modpackEntry.appendChild(modsList);
        container.appendChild(modpackEntry);
    });
}

// On page load, initialize the modpack list
window.addEventListener('load', async () => {
    // build the mods list first
    await download_repo(); 
    await download_foreign(); 
    combine_jsons();
    // load  the modpacks
    let api_server = await loadStorage().get('config-api-server');
    let modpacks_url = `${api_server}/modpacks.json`;
    modpack_data = await fetch(modpacks_url).then(response => response.json());
    console.log(modpack_data);
    modpack_status = (modpack_data !== null);
    // build the list
    build_modpack_repo();
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
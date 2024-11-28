let installed_cache = null;
let inPath = null;

// open folder
function openFolder(inPath) {
    console.log('Opening folder:', inPath); 
    invoke('open_folder', { inPath });
}

// filter tag names
function filter_tags(tag) {
    switch (tag) {
        case "gen-ai":
            return "Generative AI";
        default:
            return tag;
    }
}

// put the json into an html list
function build_list() {
    // go through the data and populate the html
    const container = document.querySelector(".mods-container");
    container.innerHTML = "";
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
        installedText.textContent = ` You have version ${entry.modjson.version} installed.`;
        description.appendChild(installedText);
        detailsDiv.appendChild(nameHeading);
        detailsDiv.appendChild(description);
        // actions
        const actionsDiv = document.createElement('div');
        actionsDiv.classList.add('mod-actions');
        const deleteIcon = document.createElement('img');
        deleteIcon.src = 'assets/img/delete.png';
        deleteIcon.alt = 'Delete Button';
        deleteIcon.classList.add('mod-download-icon', 'hvr-shrink');
        actionsDiv.appendChild(deleteIcon);
        // on delete click
        deleteIcon.addEventListener('click', async () => {
            console.log('Deleting mod:', entry.modjson.name);
            const modPath = entry.folder;
            invoke('uninstall_mod', { modPath });
        });
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
        added_mods++;
    });
    if (added_mods == 0) {
        //<div class="loading">Loading the mod repository<span class="dots"></span></div>
        const noMods = document.createElement('div');
        noMods.classList.add('loading');
        noMods.textContent = 'No mods found for this search criteria.. maybe yours can be the first?';
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

// when a mod is uninstalled
listen('mod-uninstall', async (event) => {
    // reload the browser
    load_browser();
});

// update what mods are already installed
listen('installed-mods', async (event) => {
    installed_cache = event.payload;
    console.log(installed_cache);
    build_list();
});

// add a click event to "refresh-mods" button
document.querySelector('#refresh-mods').addEventListener('click', async () => {
    load_installed();
    set_status("Mods refreshed!");
});
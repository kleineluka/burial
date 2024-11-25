// constant (for now at least)
const repo = "https://llamawa.re/repo.json";
let repo_data = null;
let repo_status = false;
let search_cache = null;
let fuse = null;

// filter tag names
function filter_tags(tag) {
    switch (tag) {
        case "gen-ai":
            return "Generative AI";
        default:
            return tag;
    }
}

// sort the data by modJson.name alphabetically
function sort_alphabetically() {
    repo_data.sort((a, b) => {
        const a_name = a.modJson.name || 'Unnamed Mod';
        const b_name = b.modJson.name || 'Unnamed Mod';
        return a_name.localeCompare(b_name);
    }); 
}

// sort the data by data.lastUpdate (newest dates first)
function sort_updated() {
    repo_data.sort((a, b) => {
        const dateA = new Date(a.data?.lastUpdate || 0);
        const dateB = new Date(b.data?.lastUpdate || 0);
        return dateB - dateA; 
    });
}

// download json into a structured object
async function download_repo() {
    // gather the data
    const response = await fetch(repo);
    if (!response.ok) {
        repo_status = false;
        console.error("Failed to fetch repository data");
        return;
    }
    const data = await response.json();
    if (!data) {
        repo_status = false;
        console.error("Failed to parse repository data");
        return;
    }
    repo_data = data;
    repo_status = true;  
    // build search cache
    search_cache = {};
    repo_data.forEach(entry => {
        search_cache[entry.data.id] = true;
    });
    const options = {
        keys: ["modJson.name"], 
        threshold: 0.1
    };
    fuse = new Fuse(repo_data, options);
}

// download json into a structured object
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
    repo_data.forEach(entry => {
        // get mod data
        const initialData = entry.data || {};
        const modData = entry.modJson || {};
        // first we need to get the tags and see if we are filtering for anything
        if (filter_kind && filter_kind !== 'all') {
            if (!initialData.tags || !initialData.tags.includes(filter_kind)) {
                return;
            }
        }
        // see if the search query is in the cache
        if (!search_cache[initialData.id]) return;
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
        iconImg.src = modData.icon || 'assets/img/default.png'; 
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
        nameHeading.innerHTML = nameHTML;
        const description = document.createElement('p');
        description.classList.add('mod-description');
        description.textContent = modData.description || 'No description provided';
        detailsDiv.appendChild(nameHeading);
        detailsDiv.appendChild(description);
        // actions
        const actionsDiv = document.createElement('div');
        actionsDiv.classList.add('mod-actions');
        const downloadIcon = document.createElement('img');
        downloadIcon.src = 'assets/img/download.png';
        downloadIcon.alt = 'Download Button';
        downloadIcon.classList.add('mod-download-icon', 'hvr-shrink');
        actionsDiv.appendChild(downloadIcon);
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
        secondRow.appendChild(tagsDiv);
        // put it all together
        modEntry.appendChild(firstRow);
        modEntry.appendChild(secondRow);
        container.appendChild(modEntry);
    });
}

// on sort dropdown update 
const sortDropdown = document.querySelector('#sortDropdown');
sortDropdown.addEventListener('change', async () => {
    const sortKind = sortDropdown.value;
    const filterKind = document.querySelector('#filterDropdown').value;
    build_repo(sortKind, filterKind); 
});

// on filter update
const filterDropdown = document.querySelector('#filterDropdown');
filterDropdown.addEventListener('change', async () => {
    const sortKind = document.querySelector('#sortDropdown').value;
    const filterKind = filterDropdown.value;
    const searchQuery = document.querySelector('#searchBar').value;
    build_repo(sortKind, filterKind, searchQuery); 
});

// on search update
const searchBar = document.querySelector('#searchBar');
searchBar.addEventListener('input', async () => {
    if (!searchBar.value || searchBar.value === '') {
        repo_data.forEach(entry => {
            search_cache[entry.data.id] = true;
        });
    } else {
        const query = searchBar.value;
        const results = fuse.search(query);
        const ids = results.map(result => result.item.data.id);
        repo_data.forEach(entry => {
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

// on page load, fetch the repository
window.addEventListener('load', async () => {
    await download_repo(); // avoid redownloading
    build_repo('name', 'all'); 
});
// versioning numbers
function inputVersion(input) {
    // numbers and periods ONLY!! start with number + only one period at a time
    input.value = input.value.replace(/[^0-9.]/g, '');
    input.value = input.value.replace(/(\.\.)/g, '.'); 
}

// add/remove authors
document.getElementById("add-author").addEventListener("click", function () {
    // create container + new seconddary row
    const container = document.getElementById("authors-container");
    let newRow = document.createElement("div");
    newRow.classList.add("second-row");
    // add the html structure + append
    newRow.innerHTML = `
            <p><!-- Empty! --></p>
            <input type="text" class="file-path-input input-author" value="">
            <button class="browse-button hvr-shrink remove-author">-</button>
        `;
    container.appendChild(newRow);
    newRow.querySelector(".remove-author").addEventListener("click", function () {
        newRow.remove();
    });
});

// add/remove mod dependencies
document.getElementById("add-mod-dep").addEventListener("click", function () {
    // prep: get container, create row
    const container = document.getElementById("mod-dep-container");
    let newRow = document.createElement("div");
    newRow.classList.add("double-row");
    newRow.classList.add("second-row");
    // add the row + listener to remove the row
    newRow.innerHTML = `
            <p><!-- Empty --></p>
            <input class="file-path-input input-mod-id" placeholder="Mod ID" value="">
            <input class="file-path-input input-mod-version" placeholder="Mod Version" value="">
            <button class="browse-button hvr-shrink remove-mod-dep">-</button>
        `;
    container.appendChild(newRow);
    newRow.querySelector(".remove-mod-dep").addEventListener("click", function () {
        newRow.remove();
    });
});

// add/remove files
document.getElementById("add-file").addEventListener("click", function () {
    // prep: get option, get container, create row
    const selectedValue = document.getElementById("dropdown-menu-file").value;
    const container = document.getElementById("files-container");
    let newRow = document.createElement("div");
    // branch based on what we are making
    switch (selectedValue) {
        case "assets":
            newRow.innerHTML = `
                    <div class="second-row" data-type="assets">
                        <p>Asset:</p>
                        <input class="file-path-input input-files-assets" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "imageDeltas":
            newRow.innerHTML = `
                    <div class="second-row" data-type="imageDeltas">
                        <p>Image Delta:</p>
                        <input class="file-path-input input-files-imagedeltas" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "dataDeltas":
            newRow.innerHTML = `
                    <div class="second-row" data-type="dataDeltas">
                        <p>Data Delta:</p>
                        <input class="file-path-input input-files-datadeltas" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "plugins":
            newRow.innerHTML = `
                    <div class="second-row" data-type="plugins">
                        <p>Plugin:</p>
                        <input class="file-path-input input-files-plugins" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "languages":
            newRow.innerHTML = `
                    <div class="second-row" data-type="languages">
                        <p>Language:</p>
                        <input class="file-path-input input-files-languages" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "inject":
            newRow.innerHTML = `
                    <div class="double-row" data-type="inject">
                        <p>Inject:</p>
                        <input class="file-path-input input-files-inject-file" placeholder="File to Inject" value="">
                        <input class="file-path-input input-files-inject-at" placeholder="Inject At" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        default:
            break;
    }
    // add the row + listener to remove the row
    container.appendChild(newRow);
    newRow.querySelector(".remove-file").addEventListener("click", function () {
        newRow.remove();
    });
});

// load button listener
document.getElementById("load-modjson").addEventListener("click", function () {
    const inPath = document.getElementById("input-modjson").value;
    invoke("load_modjson", { inPath });
});

// save button listener
document.getElementById("save-modjson").addEventListener("click", function () {
    const inPath = document.getElementById("input-modjson").value;
    const modjson = buildModjson();
    invoke ("save_modjson", { inPath, modjson });
});

// refresh button listener
document.getElementById("clear-modjson").addEventListener("click", function () {
    clearModjson();
});

// example button listener
document.getElementById("example-modjson").addEventListener("click", function () {
    fetch('/data/rules/modjson/example.json')
        .then(response => response.json())
        .then(data => {
            clearModjson();
            loadModjson(data);
        });
});

// build modjson
function buildModjson() {
    // build what will be the json
    const modData = {
        id: document.getElementById('input-id').value.trim(),
        name: document.getElementById('input-name').value.trim(),
        authors: [],
        description: document.getElementById('input-description').value.trim(),
        version: document.getElementById('input-version').value.trim(),
        spec: document.getElementById('input-spec').value.trim() || undefined, // optional
        gameDep: document.getElementById('input-dep-game').value.trim(),
        specDep: document.getElementById('input-dep-spec').value.trim(),
        modDeps: {},
        files: {}
    };
    // ensure all fields are filled in (except for spec)
    if (!modData.id || !modData.name || !modData.description || !modData.version || !modData.gameDep) {
        Swal.fire({
            title: 'Hey, listen!',
            text: 'Please fill in all of the required fields!',
        });
        return
    }
    // add authors, if any
    let has_authors = true;
    const authorInputs = document.querySelectorAll('.input-author');
    if (authorInputs.length === 1 && !authorInputs[0].value) {
        has_authors = false;
    } else {
        authorInputs.forEach(authorInput => {
            const authorValue = authorInput.value.trim();
            if (authorValue) {
                modData.authors.push(authorValue);
            }
        });
    }
    // add mod dependencies, if any
    let has_mod_deps = true;
    const modDepRows = document.querySelectorAll('#mod-dep-container .double-row');
    if (modDepRows.length === 1 && !modDepRows[0].querySelector('.input-mod-id').value) {
        has_mod_deps = false;
    } else {
        modDepRows.forEach(row => {
            const modID = row.querySelector('.input-mod-id').value.trim();
            const modVersion = row.querySelector('.input-mod-version').value.trim();
            if (modID && modVersion) {
                // it should look like: { modID: modVersion }, and they should all be in the same { }
                modData.modDeps[modID] = modVersion;
            }
        });
    }
    // collect files
    const fileRows = document.querySelectorAll('#files-container .second-row, #files-container .double-row');
    let has_files = true;
    if (fileRows == 0) {
        has_files = false;
    } else {
        fileRows.forEach(row => {
            const fileType = row.dataset.type; 
            switch (fileType) {
                case 'assets':
                case 'imageDeltas':
                case 'dataDeltas':
                case 'plugins':
                case 'languages':
                    const filePath = row.querySelector('.file-path-input').value.trim();
                    if (filePath) {
                        if (!modData.files[fileType]) { 
                            modData.files[fileType] = [];
                        }
                        modData.files[fileType].push(filePath);
                    }
                    break;
                case 'inject':
                    const injectFile = row.querySelector('.input-files-inject-file').value.trim();
                    const injectAt = row.querySelector('.input-files-inject-at').value.trim();
                    if (injectFile && injectAt) {
                        if (!modData.files.inject) {
                            modData.files.inject = [];
                        }
                        modData.files.inject.push({
                            file: injectFile,
                            at: injectAt
                        });
                    }
                    break;
                default:
                    break;
            }
        });
    }
    // remove undefined fields
    Object.keys(modData).forEach(key => {
        if (modData[key] === undefined) {
            delete modData[key];
        }
    });
    const modDataJson = JSON.stringify(modData, null, 2);
    return modDataJson;
}

// load modjson
function loadModjson(modData) {
    // individual fields
    document.getElementById('input-id').value = modData.id || '';
    document.getElementById('input-name').value = modData.name || '';
    document.getElementById('input-description').value = modData.description || '';
    document.getElementById('input-version').value = modData.version || '';
    document.getElementById('input-spec').value = modData.spec || '';
    document.getElementById('input-dep-game').value = modData.dependencies.game || '';
    document.getElementById('input-dep-spec').value = modData.dependencies.spec || '';
    // dynamically added fields
    const authorsContainer = document.getElementById('authors-container');
    const modDepContainer = document.getElementById('mod-dep-container');
    const filesContainer = document.getElementById('files-container');
    // fill in the basic fields
    if (modData.authors.length > 0) {
        let firstAuthor = true;
        modData.authors.forEach(author => {
            // the first author goes in the main input in first-row
            if (firstAuthor) {
                document.getElementById('input-author').value = author;
                firstAuthor = false;
            } else {
                // the rest of the authors go in the secondary rows
                let newRow = document.createElement('div');
                newRow.classList.add('second-row');
                newRow.innerHTML = `
                    <p><!-- Empty! --></p>
                    <input type="text" class="file-path-input input-author" value="${author}">
                    <button class="browse-button hvr-shrink remove-author">-</button>
                `;
                authorsContainer.appendChild(newRow);
                newRow.querySelector('.remove-author').addEventListener('click', function() {
                    newRow.remove();
                });
            }
        });
    }
    // fill in the mod dependencies
    if(Object.keys(modData.dependencies.mods).length > 0) {
        let firstModDep = true;
        Object.entries(modData.dependencies.mods).forEach(([modId, modVersion]) => {
            if (firstModDep) {
                document.getElementById('input-mod-id').value = modId;
                document.getElementById('input-mod-version').value = modVersion;
                firstModDep = false;
            } else {
                let newRow = document.createElement('div');
                newRow.classList.add('double-row');
                newRow.classList.add("second-row");
                newRow.innerHTML = `
                <p><!-- Empty --></p>
                <input class="file-path-input input-mod-id" placeholder="Mod ID" value="${modId}">
                <input class="file-path-input input-mod-version" placeholder="Mod Version" value="${modVersion}">
                <button class="browse-button hvr-shrink remove-mod-dep">-</button>
            `;
                modDepContainer.appendChild(newRow);
                newRow.querySelector('.remove-mod-dep').addEventListener('click', function () {
                    newRow.remove();
                });
            }
        });
    }
    // fill in the files
    if (Object.keys(modData.files).length > 0) {
        Object.entries(modData.files).forEach(([fileType, files]) => {
            files.forEach(file => {
                let newRow = document.createElement('div');
                switch (fileType) {
                    case 'assets':
                        newRow.innerHTML = `
                            <div class="second-row" data-type="assets">
                                <p>Asset:</p>
                                <input class="file-path-input input-files-assets" value="${file}">
                                <button class="browse-button hvr-shrink remove-file">-</button>
                            </div>
                        `;
                        break;
                    case 'imageDeltas':
                        newRow.innerHTML = `
                            <div class="second-row
                            " data-type="imageDeltas">
                                <p>Image Delta:</p>
                                <input class="file-path-input input-files-imagedeltas" value="${file}">
                                <button class="browse-button hvr-shrink remove-file">-</button>
                            </div>
                        `;
                        break;
                    case 'dataDeltas':
                        newRow.innerHTML = `
                            <div class="second-row
                            " data-type="dataDeltas">
                                <p>Data Delta:</p>
                                <input class="file-path-input input-files-datadeltas" value="${file}">
                                <button class="browse-button hvr-shrink remove-file">-</button>
                            </div>
                        `;
                        break;
                    case 'plugins':
                        newRow.innerHTML = `
                            <div class="second-row
                            " data-type="plugins">
                                <p>Plugin:</p>
                                <input class="file-path-input input-files-plugins" value="${file}">
                                <button class="browse-button hvr-shrink remove-file">-</button>
                            </div>
                        `;
                        break;
                    case 'languages':
                        newRow.innerHTML = `
                            <div class="second-row
                            " data-type="languages">
                                <p>Language:</p>
                                <input class="file-path-input input-files-languages" value="${file}">
                                <button class="browse-button hvr-shrink remove-file">-</button>
                            </div>
                        `;
                        break;
                    case 'inject':
                        newRow.innerHTML = `
                            <div class="double-row
                            " data-type="inject">
                                <p>Inject:</p>
                                <input class="file-path-input input-files-inject-file" placeholder="File to Inject" value="${file.file}">
                                <input class="file-path-input input-files-inject-at" placeholder="Inject At" value="${file.at}">
                                <button class="browse-button hvr-shrink remove-file">-</button>
                            </div>
                        `;
                        break;
                    default:
                        break;
                }
                filesContainer.appendChild(newRow);
                newRow.querySelector('.remove-file').addEventListener('click', function () {
                    newRow.remove();
                });
            });
        });
    }
}

// clear modjson
function clearModjson() {
    // individual fields
    document.getElementById('input-id').value = '';
    document.getElementById('input-name').value = '';
    document.getElementById('input-description').value = '';
    document.getElementById('input-version').value = '';
    document.getElementById('input-spec').value = '';
    document.getElementById('input-dep-game').value = '';
    document.getElementById('input-dep-spec').value = '';
    // dynamically added fields
    const authorsContainer = document.getElementById('authors-container');
    const modDepContainer = document.getElementById('mod-dep-container');
    const filesContainer = document.getElementById('files-container');
    // clear authors (set first row to empty and remove all other rows)
    document.getElementById('input-author').value = '';
    const authorRows = authorsContainer.querySelectorAll('.second-row');
    authorRows.forEach(row => {
        row.remove();
    });
    // clear mod dependencies (set first row to empty and remove all other rows)
    document.getElementById('input-mod-id').value = '';
    document.getElementById('input-mod-version').value = '';
    const modDepRows = modDepContainer.querySelectorAll('.second-row');
    modDepRows.forEach(row => {
        row.remove();
    });
    // clear files (remove all rows)
    const fileRows = filesContainer.querySelectorAll('.second-row, .double-row');
    fileRows.forEach(row => {
        row.remove();
    });
    // update scroll position of the scrollable container 
    const scrollableContainer = document.querySelector('.scrollable-area');
    scrollableContainer.scrollTop = 0;
}

// listen for the json file loaded
listen("load-modjson", (event) => {
    const modData = JSON.parse(event.payload);
    loadModjson(modData);
});
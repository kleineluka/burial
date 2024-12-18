// calculate the sha256 hash of a zip file
async function hashZipFile(file) {
    const buffer = await file.arrayBuffer();
    const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');
    return hashHex;
}

// load button listener
document.getElementById("load-repojson").addEventListener("click", function () {
    const inPath = document.getElementById("input-repojson").value;
    invoke("load_repojson", { inPath });
});

// save button listener
document.getElementById("save-repojson").addEventListener("click", function () {
    const inPath = document.getElementById("input-repojson").value;
    const repojson = buildRepojson();
    invoke("save_repojson", { inPath, repojson });
});

// refresh button listener
document.getElementById("clear-repojson").addEventListener("click", function () {
    clearRepojson();
});

// example button listener
document.getElementById("example-repojson").addEventListener("click", function () {
    fetch('/data/rules/repojson/example.json')
        .then(response => response.json())
        .then(data => {
            clearRepojson();
            loadRepojson(data);
        });
});

// listen for click for set date time
document.getElementById("set-date").addEventListener("click", function () {
    const date = new Date();
    document.getElementById("input-date").value = date.toISOString();
});

// listen for click on set-sha256 button
document.getElementById("set-sha256").addEventListener("click", function () {
    // open sweetalert2 with file input
    Swal.fire({
        title: 'Select Your Mod',
        text: 'No files are uploaded and the SHA-256 hash is calculated locally. Of course, you can also use other tools to calculate it on your own!',
        input: 'file',
        showCancelButton: true,
        confirmButtonText: "Continue",
        reverseButtons: true,
        confirmButtonColor: "var(--main-colour)",
        inputAttributes: {
            accept: '.zip',
            'aria-label': ' Please select the .zip file!'
        }
    }).then((result) => {
        if (result.value) {
            set_status("Calculating hash..");
            const file = result.value;
            hashZipFile(file).then((hash) => {
                document.getElementById("input-sha256").value = hash;
                clear_status();
            });
        } else {
            set_status("No mod zip selected!");
        }
    });
});

// add tags
document.getElementById("add-tag").addEventListener("click", function () {
    // prep: get container, create row
    const selectedValue = document.getElementById("dropdown-menu-tags").value;
    const container = document.getElementById("tags-container");
    let newRow = document.createElement("div");
    // add the row with the filled in value
    newRow.innerHTML = `<div class="second-row" data-type="tags">
                            <p></p>
                            <input class="file-path-input" value="${selectedValue}">
                            <button class="browse-button hvr-shrink remove-file">-</button>
                        </div>`;
    // add row to container
    container.appendChild(newRow);
    // listen for click on remove button
    newRow.querySelector(".remove-file").addEventListener("click", function () {
        newRow.remove();
    });
});

// build repojson
function buildRepojson() {
    // build what will be the json
    const repoData = {
        lsatUpdate: document.getElementById('input-date').value.trim(),
        url: document.getElementById('input-url').value.trim(),
        source: document.getElementById('input-source').value.trim() || undefined, // optional
        sha256: document.getElementById('input-sha256').value.trim(),
        tags: [], // optional
    };
    // ensure all fields are filled in (except for spec)
    if (!repoData.lsatUpdate || !repoData.url || !repoData.sha256) {
        Swal.fire({
            title: 'Hey, listen!',
            text: 'Please fill in all of the required fields!',
        });
        return
    }
    // get all tags
    const tags = document.querySelectorAll('div[data-type="tags"]');
    tags.forEach(tag => {
        repoData.tags.push(tag.querySelector('input').value);
    });
    // if there are no tags, remove the tags key entirely
    if (repoData.tags.length === 0) {
        delete repoData.tags;
    }
    return repoData;
}

// load repojson
function loadRepojson(data) {
    // load the data into the form
    document.getElementById('input-date').value = data.lsatUpdate;
    document.getElementById('input-url').value = data.url;
    document.getElementById('input-source').value = data.source || "";
    document.getElementById('input-sha256').value = data.sha256;
    // add tags
    data.tags.forEach(tag => {
        // prep: get container, create row
        const container = document.getElementById("tags-container");
        let newRow = document.createElement("div");
        newRow.className = "tag-container"; // for easy removal
        // add the row with the filled in value
        newRow.innerHTML = `<div class="second-row" data-type="tags">
                                <p></p>
                                <input class="file-path-input" value="${tag}">
                                <button class="browse-button hvr-shrink remove-file">-</button>
                            </div>`;
        // add row to container
        container.appendChild(newRow);
        // listen for click on remove button
        newRow.querySelector(".remove-file").addEventListener("click", function () {
            newRow.remove();
        });
    });
}

// clear repojson
function clearRepojson() {
    // clear the form
    document.getElementById('input-date').value = "";
    document.getElementById('input-url').value = "";
    document.getElementById('input-source').value = "";
    document.getElementById('input-sha256').value = "";
    // clear tags
    const tags = document.querySelectorAll('.tag-container');
    tags.forEach(tag => {
        tag.remove();
    });
}

// listen for the json file loaded
listen("load-repojson", (event) => {
    const repoData = JSON.parse(event.payload);
    loadRepojson(repoData);
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#clear-repojson', {
        content: 'This will reset all entered data by clearing all fields - make sure you save first if needed!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#example-repojson', {
        content: 'This will reset all entered data by entering in example data - make sure you save first if needed!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#last-updated-label', {
        content: 'When you last updated your mod - must be in ISO format. Just use the button to set it!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#set-date', {
        content: 'Automatically get the current timestamp in ISO format and set it in the repo json.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#zip-url-label', {
        content: 'The URL to where your mod is hosted (ex. Github, Codeberg) for downloading. Please note that it must be a direct link to the zip file.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#source-url-label', {
        content: 'The URL to where your source code is hosted (ex. Github, Codeberg). This is optional, but recommended!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#sha-256-label', {
        content: 'The SHA-256 hash of your mod zip file. You can use the button to calculate it! This is to ensure that the mod is not tampered with and isn\'t corrupted.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#tags-label', {
        content: 'Add tags to describe what your mod does and to help players find it. You can add as many as you want!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
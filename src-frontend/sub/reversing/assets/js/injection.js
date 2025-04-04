// populate + update dropdowns
let injectionData = {};
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/injection.json')
        .then(response => response.json())
        .then(data => {
            injectionData = data;
            const dropdownFile = document.getElementById('dropdown-menu-file');
            const dropdownLocation = document.getElementById('dropdown-menu-location');
            // populate the 'Inject To File' dropdown
            for (const fileName in data) {
                const option = document.createElement('option');
                option.value = fileName;
                option.textContent = fileName;
                dropdownFile.appendChild(option);
            }
            // update the 'Inject In File' dropdown when a file is selected
            dropdownFile.addEventListener('change', () => {
                dropdownLocation.innerHTML = '';
                const selectedFile = dropdownFile.value;
                const locations = data[selectedFile];
                for (const location in locations) {
                    const option = document.createElement('option');
                    option.value = location;
                    option.textContent = location;
                    dropdownLocation.appendChild(option);
                }
            });
            // trigger the change event on page load to populate the second dropdown
            dropdownFile.dispatchEvent(new Event('change'));
        })
        .catch(error => console.error('Error fetching Injection List JSON:', error));
});

// backup button
function backupButton() {
    // get the game path
    const gamePath = document.getElementById('tcoaal-path').value;
    // get selected file + where it's at locally
    const selectedFile = document.getElementById('dropdown-menu-file').value;
    const selectedLocation = document.getElementById('dropdown-menu-location').value;
    const locationData = injectionData[selectedFile][selectedLocation];
    // send to backend
    invoke ("injection_backup", { gamePath, inPath: locationData.file });
}

// open file button
function openFileButton() {
    // get the game path
    const gamePath = document.getElementById('tcoaal-path').value;
    // if empty, return error
    if (gamePath === '') {
        Swal.fire({
            icon: "error",
            title: "Set your game path to open the file!",
            showConfirmButton: true
        });
        return;
    }
    // get selected file + where it's at locally
    const selectedFile = document.getElementById('dropdown-menu-file').value;
    const selectedLocation = document.getElementById('dropdown-menu-location').value;
    const locationData = injectionData[selectedFile][selectedLocation];
    // send to backend
    invoke ("injection_open_file", { gamePath, inPath: locationData.file });
}

// open folder button
function openFolderButton() {
    // get the game path
    const gamePath = document.getElementById('tcoaal-path').value;
    // if empty, return error
    if (gamePath === '') {
        Swal.fire({
            icon: "error",
            title: "Set your game path to open the folder!",
            showConfirmButton: true
        });
        return;
    }
    // get selected file + where it's at locally
    const selectedFile = document.getElementById('dropdown-menu-file').value;
    const selectedLocation = document.getElementById('dropdown-menu-location').value;
    const locationData = injectionData[selectedFile][selectedLocation];
    // send to backend
    invoke("injection_open_folder", { gamePath, inPath: locationData.file });
}

// inject button
function injectButton() {
    // get the game path
    const gamePath = document.getElementById('tcoaal-path').value;
    const codePath = document.getElementById('code-path').value;
    // get selected file + where it's at locally
    const selectedFile = document.getElementById('dropdown-menu-file').value;
    const selectedLocation = document.getElementById('dropdown-menu-location').value;
    const locationData = injectionData[selectedFile][selectedLocation];
    // get the .location property and then fetch the json at rules/injection/.location.json
    fetch(`/data/rules/injection/${locationData.location}`)
        .then(response => response.json())
        .then(data => {
            // send data to backend
            const inPath = locationData.file;
            const before = data.before;
            const after = data.after;
            const indentation = data.indentation;
            invoke("injection_save", { gamePath, inPath, before, after, codePath, indentation });
        })
        .catch(error => console.error('Error fetching Injection Rules JSON:', error));
}

// preview file button
function previewButton() {
    // get the game path
    const gamePath = document.getElementById('tcoaal-path').value;
    const codePath = document.getElementById('code-path').value;
    // get selected file + where it's at locally
    const selectedFile = document.getElementById('dropdown-menu-file').value;
    const selectedLocation = document.getElementById('dropdown-menu-location').value;
    const locationData = injectionData[selectedFile][selectedLocation];
    // get the .location property and then fetch the json at rules/injection/.location.json
    console.log(locationData.location);
    fetch(`/data/rules/injection/${locationData.location}`)
        .then(response => response.json())
        .then(data => {
            // send data to backend
            const inPath = locationData.file;
            const before = data.before;
            const after = data.after;
            const indentation = data.indentation;
            invoke ("injection_preview", { gamePath, inPath, before, after, codePath, indentation });
            document.getElementById('preview-information').textContent = `Previewing Injection in ${selectedFile} at ${selectedLocation}`;
        })
        .catch(error => console.error('Error fetching Injection Rules JSON:', error));
}

// listen to display the preview
listen('preview', (event) => {
    // add hidden to id sub-main and remove from sub-preview
    document.getElementById('sub-main').classList.add('hidden');
    document.getElementById('sub-preview').classList.remove('hidden');
    // set the preview code (textarea-preview)
    document.getElementById('textarea-preview').textContent = event.payload;
});

// go back from preview
function previewBackButton() {
    // remove hidden from id sub-main and add to sub-preview
    document.getElementById('sub-main').classList.remove('hidden');
    document.getElementById('sub-preview').classList.add('hidden');
    // clear the preview code
    document.getElementById('textarea-preview').textContent = 'Loading preview..';
}

// copy preview code
function previewCopyButton() {
    // get the preview code
    const previewCode = document.getElementById('textarea-preview');
    // copy the code to the clipboard
    navigator.clipboard.writeText(previewCode.value);
    set_status('Copied to clipboard!');
}

// code path browse
document.getElementById('browse-button-code').addEventListener('click', (event) => {
    invoke('file_dialog', { emitEvent: 'selected-code-file', fileType: 'all' });
});

listen('selected-code-file', (event) => {
    document.getElementById('code-path').value = event.payload;
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#backup-button', {
        content: 'Make a back up of the target file to return to after injecting.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#preview-button', {
        content: 'Preview what the final injection will look like.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#open-file-button', {
        content: 'Open the target file that is going to be injected.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#open-folder-button', {
        content: 'Open the folder containing the target file.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#inject-button', {
        content: 'Inject the code into the target file. You may want to back the file up first!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#inject-file-label', {
        content: 'What TCOAAL file to put your code into.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#inject-point-label', {
        content: 'Where in the file to put your code.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#code-path',
        {
            content: 'Path to the code you want to inject (like a .js file).',
            animation: 'perspective-subtle',
            placement: 'top',
            theme: 'burial'
        });
});
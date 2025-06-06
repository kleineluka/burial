// determine if finding a file or folder and launch dialog
document.getElementById('browse-button-in').addEventListener('click', (event) => {
    var emitEvent = 'selected-input-folder';
    if (document.getElementById('select-file').classList.contains('selected')) {
        var fileType = 'all';
        window.__TAURI__.invoke('file_dialog', { emitEvent, fileType });
    } else {
        window.__TAURI__.invoke('folder_dialog', { emitEvent });
    }
});

// listen to put the selected data back in the input path
document.addEventListener('DOMContentLoaded', () => {
    listen('selected-input-folder', (event) => {
        document.getElementById('input-path').value = event.payload;
    });
});

// do the encryption
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('encrypt-button').addEventListener('click', () => {
        // get if select-file or select-folder is selected
        const folderButton = document.getElementById('select-folder');
        const fileButton = document.getElementById('select-file');
        let pathKind = (folderButton.classList.contains('selected')) ? 'folder' : 'file';
        // check if we are decrypting the whole file or just the original byte positions
        const wholeButton = document.getElementById('select-byte-whole');
        const originalButton = document.getElementById('select-byte-original');
        let advancedPositions = (originalButton.classList.contains('selected')) ? true : false;
        // get paths and send to rust
        const inPath = document.getElementById('input-path').value;
        const outPath = document.getElementById('output-path').value;
        invoke('encrypt', { pathKind, inPath, outPath, advancedPositions });
    });
});

// switch between encrypting a single file and decrypting a folder
document.addEventListener('DOMContentLoaded', () => {
    const folderButton = document.getElementById('select-folder');
    const fileButton = document.getElementById('select-file');
    folderButton.addEventListener('click', () => {
        folderButton.classList.add('selected');
        fileButton.classList.remove('selected');
    });
    fileButton.addEventListener('click', () => {
        fileButton.classList.add('selected');
        folderButton.classList.remove('selected');
    });
});

// show advanced settings
document.getElementById('advanced-settings-toggle').addEventListener('click', function () {
    var advancedContents = document.getElementsByClassName('advanced-settings-contents')[0];
    advancedContents.classList.toggle('expanded');
});

// switch between encrypting the whole file or just the original byte positions
document.addEventListener('DOMContentLoaded', () => {
    const wholeButton = document.getElementById('select-byte-whole');
    const originalButton = document.getElementById('select-byte-original');
    wholeButton.addEventListener('click', () => {
        wholeButton.classList.add('selected');
        originalButton.classList.remove('selected');
    });
    originalButton.addEventListener('click', () => {
        originalButton.classList.add('selected');
        wholeButton.classList.remove('selected');
    });
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();s
    tippy('#select-byte-whole', {
        content: 'Encrypt the whole file (suggested)',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#select-byte-original', {
        content: 'Encrypt only a specific part of the file depending on extension (like tcoaal)',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});

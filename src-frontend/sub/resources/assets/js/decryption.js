// listen for button click to move to index.html on back
document.getElementById('browse-button-in').addEventListener('click', (event) => {
    var emitEvent = 'selected-input-folder';
    if (document.getElementById('select-file').classList.contains('selected')) {
        var fileType = 'k9a';
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

// do the decryption
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('decrypt-button').addEventListener('click', () => {
        // get if select-file or select-folder is selected
        const folderButton = document.getElementById('select-folder');
        const fileButton = document.getElementById('select-file');
        let pathKind = (folderButton.classList.contains('selected')) ? 'folder' : 'file';
        // get paths and send to rust
        const inPath = document.getElementById('input-path').value;
        const outPath = document.getElementById('output-path').value;
        invoke('decrypt', { pathKind, inPath, outPath });
    });
});

// switch between decrypting a single file and decrypting a folder
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

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});

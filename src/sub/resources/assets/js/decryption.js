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

document.getElementById('browse-button-out').addEventListener('click', (event) => {
    var emitEvent = 'selected-output-folder';
    window.__TAURI__.invoke('folder_dialog', { emitEvent });
});

// listen to put the selected data back in the input path
document.addEventListener('DOMContentLoaded', () => {
    listen('selected-input-folder', (event) => {
        document.getElementById('input-path').value = event.payload;
    });
});

// listen to put the selected data back in the output path
document.addEventListener('DOMContentLoaded', () => {
    listen('selected-output-folder', (event) => {
        document.getElementById('output-path').value = event.payload;
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

// listen for updates to the log
listen('status', (event) => {
    const logElement = document.getElementById('status');
    logElement.innerHTML = event.payload;
});

// listen for errors
listen('error', (event) => {
    const error = event.payload;
    Swal.fire({
        icon: "error",
        title: error,
        showConfirmButton: true
    });
});

// listen for success mesasges
listen('success', (event) => {
    const success = event.payload;
    Swal.fire({
        icon: "success",
        title: success,
        showConfirmButton: true
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
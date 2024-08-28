// command tauri to open a file browser
async function select_folder() {
    await window.__TAURI__.invoke('folderwalk', {});
}

// listen for button click to move to index.html on back
document.getElementById('browse-button-in').addEventListener('click', (event) => {
    select_folder();
});

// listen to put the selected data back in the path
document.addEventListener('DOMContentLoaded', () => {
    listen('selected-folder', (event) => {
        document.querySelector('.file-path-input').value = event.payload;
    });
});

// do the decryption
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('decrypt-button').addEventListener('click', () => {
        // get if select-file or select-folder is selected
        const folderButton = document.getElementById('select-folder');
        const fileButton = document.getElementById('select-file');
        let pathKind = (folderButton.classList.contains('selected')) ? 'folder' : 'file';
        // get the input path
        const inPath = document.getElementById('input-path').value;
        // get the output path
        const outPath = document.getElementById('output-path').value;
        // show logs
        const showLogs = true;
        // send to rust
        invoke('decrypt', { pathKind, inPath, outPath, showLogs });
    });
});

// listen for updates to the log
listen('log', (event) => {
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
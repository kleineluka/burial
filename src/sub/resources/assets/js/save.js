// get save files on load
document.addEventListener('DOMContentLoaded', () => {
    setTimeout(() => {
        invoke('find_saves', {});
    }, 1000);
});

// listen for save files loaded and put into dropdown
listen('load-saves', (event) => {
    // fill to all dropdowns
    const fileNames = event.payload.split(',');
    const dropdownMain = document.getElementById('dropdown-menu-save-main');
    const dropdownBackup = document.getElementById('dropdown-menu-save-backup');
    dropdownMain.innerHTML = '';
    dropdownBackup.innerHTML = '';
    fileNames.forEach((fileName) => {
        const optionMain = document.createElement('option');
        optionMain.value = fileName;
        optionMain.textContent = fileName;
        dropdownMain.appendChild(optionMain);
        const optionBackup = document.createElement('option');
        optionBackup.value = fileName;
        optionBackup.textContent = fileName;
        dropdownBackup.appendChild(optionBackup);
    });
});

// open saves folder
document.getElementById('open-button').addEventListener('click', (event) => {
    invoke('open_saves', {});   
});

// backup menu change
document.getElementById('navigate-backup-button').addEventListener('click', (event) => {
    // hide other sub menus
    document.getElementById('sub-main').classList.add('hidden');
    document.getElementById('sub-edit').classList.add('hidden');
    // show backup menu
    document.getElementById('sub-backup').classList.remove('hidden');
});

// do backup on all
document.getElementById('backup-button').addEventListener('click', (event) => {
    let backupPath = document.getElementById('input-backup-out').value;
    if (backupPath === '') {
        Swal.fire({
            icon: 'error',
            title: 'Error',
            text: 'Please enter a backup path.',
        });
    }
    invoke('backup', {backupPath});
});

// edit menu change
document.getElementById('navigate-edit-button').addEventListener('click', (event) => {
    // get selected save file name
    let saveName = document.getElementById('dropdown-menu-save-main').value;
    // hide other sub menus
    document.getElementById('sub-main').classList.add('hidden');
    document.getElementById('sub-backup').classList.add('hidden');
    // show edit menu
    document.getElementById('sub-edit').classList.remove('hidden');
    // call rust to get save file
    invoke('read_save', { saveName });
});

// listen for when the save was read
let saveContent = '';
listen('load-save', (event) => {
    saveContent = event.payload;
    document.getElementById('textarea-save').value = saveContent;
});

// copy the contents of the text area to clipboard
document.getElementById('edit-copy-button').addEventListener('click', (event) => {
    let textAreaContent = document.getElementById('textarea-save').value;
    navigator.clipboard.writeText(textAreaContent);
    set_status('Copied to clipboard!');
});

// paste from clipboard to text area
document.getElementById('edit-paste-button').addEventListener('click', (event) => {
    navigator.clipboard.readText().then((clipText) => {
        document.getElementById('textarea-save').value = clipText;
        set_status('Pasted from clipboard!');
    });
});

// add to all "back-button" classes
document.querySelectorAll('.back-button').forEach((element) => {
    element.addEventListener('click', (event) => {
        // hide sub menus
        document.getElementById('sub-backup').classList.add('hidden');
        document.getElementById('sub-edit').classList.add('hidden');
        // show main menu
        document.getElementById('sub-main').classList.remove('hidden');
    });
});
// switch between horizontal navbars
document.addEventListener('DOMContentLoaded', () => {
    const navOptions = document.querySelectorAll('.page-navbar-option');
    const subContainers = document.querySelectorAll('.page-container');
    navOptions.forEach(option => {
        option.addEventListener('click', (event) => {
            event.preventDefault();
            // clear current selection
            navOptions.forEach(nav => nav.classList.remove('selected'));
            subContainers.forEach(container => container.classList.add('hidden-container'));
            // show what was selected
            option.classList.add('selected');
            const id = option.id;
            const subContainer = document.getElementById(`sub-${id}`);
            if (subContainer) {
                subContainer.classList.remove('hidden-container');
            }
        });
    });
});

// get save files on load
document.addEventListener('DOMContentLoaded', () => {
    setTimeout(() => {
        invoke('find_saves', {});
    }, 1000);
});

// listen for save files loaded and put into dropdown
listen('load-saves', (event) => {
    const fileNames = event.payload.split(',');
    const dropdownMain = document.getElementById('dropdown-menu-save-main');
    dropdownMain.innerHTML = '';
    fileNames.forEach((fileName) => {
        const optionMain = document.createElement('option');
        optionMain.value = fileName;
        optionMain.textContent = fileName;
        dropdownMain.appendChild(optionMain);
    });
});

// open saves folder
document.getElementById('open-button').addEventListener('click', (event) => {
    invoke('open_saves', {});   
});

// copy save file
document.getElementById('copy-button').addEventListener('click', (event) => {
    let saveName = document.getElementById('dropdown-menu-save-main').value;
    invoke('copy_save', { saveName });
});

// delete all save files
document.getElementById('delete-all-button').addEventListener('click', (event) => {
    Swal.fire({
        title: "Are you sure?",
        text: "Deleting all saves is not reversible. Are you sure you want to continue?",
        type: "warning",
        showCancelButton: true,
        confirmButtonText: "Delete All Saves",
        closeOnConfirm: true,
        reverseButtons: true,
        confirmButtonColor: "var(--main-colour)",
    }).then((result) => {
        if (result.isConfirmed) {
            invoke('delete_all', {  });
        }
    });
});

// delete auto save files
document.getElementById('delete-auto-button').addEventListener('click', (event) => {
    Swal.fire({
        title: "Are you sure?",
        text: "Deleting automatic saves is not reversible. Are you sure you want to continue?",
        type: "warning",
        showCancelButton: true,
        confirmButtonText: "Delete Auto Saves",
        closeOnConfirm: true,
        reverseButtons: true,
        confirmButtonColor: "var(--main-colour)",
    }).then((result) => {
        if (result.isConfirmed) {
            invoke('delete_auto', {});
        }
    });
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
    invoke('backup_saves', { backupPath });
});

// edit menu change
document.getElementById('navigate-edit-button').addEventListener('click', (event) => {
    // get selected save file name
    let saveName = document.getElementById('dropdown-menu-save-main').value;
    // show / hide
    document.getElementById('edit-sub-main').classList.add('hidden-container');
    document.getElementById('edit-sub-edit').classList.remove('hidden-container');
    document.getElementById('navbar-main').classList.add('hidden-container');
    document.getElementById('navbar-edit').classList.remove('hidden-container');
    // call rust to get save file
    invoke('read_save', { saveName });
});

// save the save
document.getElementById('edit-save-button').addEventListener('click', (event) => {
    console.log('edit-save-button clicked!');
    // get the contents of the save file (document.getElementById('textarea-save').value)
    let saveName = document.getElementById('dropdown-menu-save-main').value;
    let saveData = document.getElementById('textarea-save').value;
    // call rust to save file
    invoke('write_save', { saveName, saveData });
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

// listen to put the selected data back in the backup path
document.getElementById('browse-button-out').addEventListener('click', (event) => {
    var emitEvent = 'selected-output-folder';
    window.__TAURI__.invoke('folder_dialog', { emitEvent });
});

document.addEventListener('DOMContentLoaded', () => {
    listen('selected-output-folder', (event) => {
        document.getElementById('input-backup-out').value = event.payload;
    });
});

// revert edited save in editor
function revertSave() {
    Swal.fire({
        title: "Are you sure?",
        text: "Reverting the save will discard all changes. Are you sure you want to continue?",
        type: "warning",
        showCancelButton: true,
        confirmButtonText: "Revert Save",
        closeOnConfirm: true,
        reverseButtons: true,
        confirmButtonColor: "var(--main-colour)",
    }).then((result) => {
        if (result.isConfirmed) {
            document.getElementById('textarea-save').value = saveContent;
            set_status('Reverted save!');
        }
    });
}

// exit editor
function exitEditor() {
    document.getElementById('edit-sub-main').classList.remove('hidden-container');
    document.getElementById('navbar-main').classList.remove('hidden-container');
    document.getElementById('edit-sub-edit').classList.add('hidden-container');
    document.getElementById('navbar-edit').classList.add('hidden-container');
}

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#open-button', {
        content: 'Open the folder where the saves are in your file explorer.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#navigate-edit-button', {
        content: 'Open a text editor inside of Burial with the save decrypted.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#copy-button', {
        content: 'Make a copy of the selected save.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#delete-all-button', {
        content: 'This will delete all of your saves!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#delete-auto-button', {
        content: 'This will delete your automatic saves!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#input-backup-out', {
        content: 'Where to back up your saves to.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
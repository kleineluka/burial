// convert bytes to mb
function bytesToMB(bytes) {
    return (bytes / 1024 / 1024).toFixed(2);
}

// create a backup
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('backup-make').addEventListener('click', () => {
        // get the tcoaal path and optionally if using a custom name 
        const inPath = document.getElementById('tcoaal-path').value;
        let inName = document.getElementById('backup-name').value;
        if (!inName || inName === '') {
            inName = 'null';
        }
        // send to rust
        invoke('create_backup', { inPath, inName });
    });
});

// get list of backups
function updateBackupsList(csv) {
    // clear container and see if empty
    const container = document.querySelector(".backups-list-contents");
    container.innerHTML = '';
    if (csv === "null") {
        const backupEntry = document.createElement("div");
        backupEntry.className = "backup-entry";
        const noBackupsMessage = document.createElement("div");
        noBackupsMessage.textContent = "No backups available.";
        backupEntry.appendChild(noBackupsMessage);
        container.appendChild(backupEntry);
        return;
    }
    // split csv and go through
    const backups_names = csv.split('|')[0];
    const backups_disk_space = csv.split('|')[1];
    const backups = backups_names.split(',');
    const disk_space = backups_disk_space.split(',');
    backups.forEach(backup => {
        // build all the components
        const backupEntry = document.createElement("div");
        backupEntry.className = "backup-entry";
        const backupPath = document.createElement("span");
        backupPath.className = "backup-path";
        const disk_space_mb = bytesToMB(disk_space[backups.indexOf(backup)]);
        backupPath.textContent = `${backup} (${disk_space_mb} MB)`;
        const backupButtons = document.createElement("div");
        backupButtons.className = "backup-buttons";
        const restoreButton = document.createElement("button");
        restoreButton.className = "backup-button restore";
        restoreButton.textContent = "Restore";
        const deleteButton = document.createElement("button");
        deleteButton.className = "backup-button delete";
        deleteButton.textContent = "Delete";
        // add all the components of the entry
        backupButtons.appendChild(restoreButton);
        backupButtons.appendChild(deleteButton);
        backupEntry.appendChild(backupPath);
        backupEntry.appendChild(backupButtons);
        // and add the entry to the container
        container.appendChild(backupEntry);
        // add event listeners to the buttons
        restoreButton.addEventListener('click', () => {
            Swal.fire({
                title: "Are you sure?",
                text: "Restoring a backup will overwrite your current TCOAAL game folder. This action cannot be undone.",
                type: "warning",
                showCancelButton: true,
                confirmButtonText: "Restore Backup",
                closeOnConfirm: true,
                reverseButtons: true,
                confirmButtonColor: "#F595B2",
            }).then((result) => {
                if (result.isConfirmed) {
                    let inPath = document.getElementById('tcoaal-path').value;
                    invoke('restore_backup', { inPath, inName: backup });
                } 
            });
        });
        deleteButton.addEventListener('click', () => {
            invoke('delete_backup', { inName: backup });
        });
    });
}

listen('reload-backups', (event) => {
    const container = document.querySelector(".backups-list-contents");
    container.innerHTML = '';
    const loading = document.createElement("div");
    loading.innerHTML = "Loading backups..";
    container.appendChild(loading);
    setTimeout(() => {
        invoke('get_backups', {});
    }, 2000);
});

listen('backups', (event) => {
    updateBackupsList(event.payload);
});

// load backups on page load
document.addEventListener('DOMContentLoaded', () => {
    // wait 2 seconds, need better way to do this
    setTimeout(() => {
        invoke('get_backups', {});
    }, 2000);
});

// delete all backups
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('backup-clean').addEventListener('click', () => {
        invoke('clean_backups', {});
    });
});

// open backups folder
function openBackups() {
    invoke('open_backups', {});
}

// show advanced settings
function advancedSettings() {
    var advancedContents = document.getElementsByClassName('advanced-settings-contents')[0];
    advancedContents.classList.toggle('expanded');
}
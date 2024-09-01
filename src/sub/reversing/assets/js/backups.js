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

// show optional settings
document.getElementById('optional-settings-toggle').addEventListener('click', function () {
    var advancedContents = document.getElementsByClassName('optional-settings-contents')[0];
    advancedContents.classList.toggle('expanded');
});

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
    const container = document.querySelector(".backups-list-kind-container");
    container.innerHTML = '';
    if (csv === "null") {
        const noBackupsMessage = document.createElement("div");
        noBackupsMessage.textContent = "No backups available.";
        container.appendChild(noBackupsMessage);
        return;
    }
    // split csv and go through
    const backups = csv.split(',');
    backups.forEach(backup => {
        // build all the components
        const backupEntry = document.createElement("div");
        backupEntry.className = "backup-entry";
        const backupPath = document.createElement("span");
        backupPath.className = "backup-path";
        backupPath.textContent = backup;
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
                    console.log('Confirmed');
                } 
            });
        });
        deleteButton.addEventListener('click', () => {
            invoke('delete_backup', { inName: backup });
        });
    });
}

listen('reload-backups', (event) => {
    const container = document.querySelector(".backups-list-kind-container");
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
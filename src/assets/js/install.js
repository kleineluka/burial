/* Listen to which install type is currently selected */
document.addEventListener('DOMContentLoaded', () => {
    const copyGameButton = document.getElementById('copy-game');
    const patchGameButton = document.getElementById('patch-game');

    copyGameButton.addEventListener('click', () => {
        copyGameButton.classList.add('selected');
        patchGameButton.classList.remove('selected');
    });

    patchGameButton.addEventListener('click', () => {
        patchGameButton.classList.add('selected');
        copyGameButton.classList.remove('selected');
    });
});

/* Listen for the install button click */
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('install-button').addEventListener('click', () => {
        // get the selected install type
        const selectedButton = document.querySelector('.install-type-button.selected');
        let installType = selectedButton ? selectedButton.textContent : '';
        installType = (installType === 'Copy Game') ? 'copy' : 'patch';
        // update the interface
        document.getElementById('log-title').textContent = `The mod is.. being installed..`;
        document.getElementById('prompt-div').style.display = 'none';
        document.getElementById('log-div').style.display = 'block';
        // send to rust
        invoke('install_mod', { installType });
    });
});

/* Listen for update logs */
document.addEventListener('DOMContentLoaded', () => {
    listen('log-update', (event) => {
        // get the log area
        const logList = document.getElementById('log-list');
        const listItem = document.createElement('li');
        // check if we need to update the last one
        const existingListItem = logList.lastElementChild;
        if (existingListItem) {
            const existingEmoji = existingListItem.querySelector('span');
            existingEmoji.textContent = '🎉';
        }
        // truncate the list if it's too long
        if (logList.children.length >= 6) {
            logList.removeChild(logList.firstElementChild);
        }
        // insert the new update
        listItem.className = 'list-item';
        listItem.innerHTML = `<span>⚒️</span> ${event.payload}`;
        logList.appendChild(listItem);
    });
});
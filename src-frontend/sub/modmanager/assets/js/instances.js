// on load, fetch from store
document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const inPath = await store.get('settings-tcoaal');
    invoke('load_instances', { inPath });
});

// format the "last played"
function formatLastPlayed(lastPlayed) {
    // if 0, then never played
    if (lastPlayed === 0) {
        return 'Never';
    }
    return lastPlayed + ' Days Ago';
}

// when the active instance is updated, it needs to be reflected
listen('active-instance', (event) => {
    // first, update the storage
    const store = loadStorage();
    store.set('state-game-instance', event.payload);
    // and then update the dropdown
    const dropdown = document.getElementById('dropdown-menu-current-instance');
    dropdown.value = event.payload;
});

// wait for a response to fill in the instances
listen('instances-loaded', async (event) => {
    const store = loadStorage();
    const data = JSON.parse(event.payload);
    const instances = data.instances;
    const instanceContainer = document.querySelector('.instance-container'); 
    // clear then make instances
    instanceContainer.innerHTML = '';
    instances.forEach(instance => {
        // add elements of each instance's html
        const instanceDiv = document.createElement('div');
        instanceDiv.classList.add('instance');
        const headerDiv = document.createElement('div');
        headerDiv.classList.add('instance-header');
        const nameSectionDiv = document.createElement('div');
        nameSectionDiv.classList.add('instance-name-section');
        const nameP = document.createElement('p');
        nameP.classList.add('instance-name');
        nameP.textContent = instance.name;
        const dateP = document.createElement('p');
        dateP.classList.add('instance-date');
        dateP.textContent = `(Made ${instance.date_created})`;
        nameSectionDiv.appendChild(nameP);
        nameSectionDiv.appendChild(dateP);
        const buttonsDiv = document.createElement('div');
        buttonsDiv.classList.add('instance-buttons');
        // active buttons with listeners attached to them
        const renameBtn = document.createElement('button');
        renameBtn.classList.add('instance-btn', 'hvr-grow', 'rename-btn');
        renameBtn.title = 'Rename this instance!';
        renameBtn.textContent = '✏️';
        renameBtn.addEventListener('click', async () => {
            const newName = await Swal.fire({
                title: 'Rename Instance',
                input: 'text',
                inputLabel: 'Whatcha wanna name it?',
                inputValue: instance.name,
                inputPlaceholder: 'Enter a new name',
                showCancelButton: true,
                confirmButtonText: 'Rename',
                confirmButtonColor: 'var(--main-colour)',
                cancelButtonText: 'Cancel',
                inputValidator: (value) => {
                    if (!value) {
                        return 'You need to write something!';
                    }
                }
            });
            if (newName.isConfirmed) {
                const inPath = await store.get('settings-tcoaal');
                invoke('rename_instance', { inPath, oldName: instance.name, newName: newName.value });
            }
        });
        const duplicateBtn = document.createElement('button');
        duplicateBtn.classList.add('instance-btn', 'hvr-grow', 'info-btn');
        duplicateBtn.title = 'Clone this instance!';
        duplicateBtn.textContent = '♻️';
        duplicateBtn.addEventListener('click', async () => {
            const newName = await Swal.fire({
                title: 'Clone Instance',
                input: 'text',
                inputLabel: 'Whatcha wanna name it?',
                inputValue: instance.name + ' Clone',
                inputPlaceholder: 'Enter a new name',
                showCancelButton: true,
                confirmButtonText: 'Duplicate',
                confirmButtonColor: 'var(--main-colour)',
                cancelButtonText: 'Cancel',
                inputValidator: (value) => {
                    if (!value) {
                        return 'You need to write something!';
                    }
                }
            });
            if (newName.isConfirmed) {
                const inPath = await store.get('settings-tcoaal');
                invoke('clone_instance', { inPath, oldInstance: instance.name, newInstance: newName.value });
            }
        });
        const deleteBtn = document.createElement('button');
        deleteBtn.classList.add('instance-btn', 'hvr-grow', 'delete-btn');
        deleteBtn.title = 'Delete this instance!';
        deleteBtn.textContent = '🗑️';
        buttonsDiv.append(renameBtn, duplicateBtn, deleteBtn);
        headerDiv.append(nameSectionDiv, buttonsDiv);
        const statsDiv = document.createElement('div');
        statsDiv.classList.add('instance-stats');
        const indexKindP = document.createElement('p');
        indexKindP.classList.add('hvr-shrink');
        indexKindP.textContent = instance.index_kind;
        statsDiv.appendChild(indexKindP);
        // conditionally add mod count
        if (instance.index_kind !== 'Vanilla Game 🍦' && instance.mod_count > 0) {
            const modCountP = document.createElement('p');
            modCountP.classList.add('hvr-shrink');
            modCountP.textContent = `${instance.mod_count} Mods`;
            statsDiv.appendChild(modCountP);
        }
        const lastPlayedP = document.createElement('p');
        lastPlayedP.classList.add('hvr-shrink');
        lastPlayedP.textContent = `Last Played ${formatLastPlayed(instance.last_played)}`;
        statsDiv.appendChild(lastPlayedP);
        const gameVersionP = document.createElement('p');
        gameVersionP.classList.add('hvr-shrink');
        gameVersionP.textContent = `Game Version ${instance.game_version}`;
        // add it all back together
        statsDiv.appendChild(gameVersionP);
        instanceDiv.append(headerDiv, statsDiv);
        instanceContainer.appendChild(instanceDiv);
    });
    // fill in all the names to the dropdown
    const dropdown = document.getElementById('dropdown-menu-current-instance');
    dropdown.innerHTML = '';
    instances.forEach(instance => {
        const option = document.createElement('option');
        option.value = instance.name;
        option.textContent = instance.name;
        dropdown.appendChild(option);
    });
    // set the active instance
    const activeInstance = await store.get('state-game-instance');
    dropdown.value = activeInstance;
    // see if the active instance exists
    const found = instances.find(instance => instance.name === activeInstance);
    if (!found) {
        Swal.fire({
            title: "Heads up!",
            text: "It looks like your active instance isn't saved to your instances folder. You may want to pick another one or make a new one..",
            toast: true,
            position: "bottom-right",
            showConfirmButton: true,
            confirmButtonText: "Got it!",
            confirmButtonColor: 'var(--main-colour)',
            timer: 20000,
        });
    }
});

// if the user doesn't have a valid game, then it gets an error..
listen('instances-errored', (event) => {
    // clear loading
    const instanceContainer = document.querySelector('.instance-container');
    instanceContainer.innerHTML = '';
    // branch based on kind of error (for now only gamepath)
    if (event.payload === 'gamepath') {
        const instanceInfo = document.createElement('div');
        instanceInfo.classList.add('instance-info');
        instanceInfo.append("Instances could not be loaded because your game path is not set. Please set your game path in the ");
        const settingsLink = document.createElement('a');
        settingsLink.href = '../../settings.html';
        settingsLink.textContent = 'Settings 🍪';
        const nowrapSpan = document.createElement('span');
        nowrapSpan.style.whiteSpace = 'nowrap';
        nowrapSpan.appendChild(settingsLink);
        instanceInfo.appendChild(nowrapSpan);
        instanceInfo.append("!");
        instanceContainer.appendChild(instanceInfo);
    }
});

listen('instances-error', (event) => {
    // match the error with the correct message
    let message = '';
    switch (event.payload) {
        case 'current-nonexistant':
            message = 'There was an error renaming the instance..';
            break;
        case 'old-nonexistant':
            message = 'There was an error cloning the instance..';
            break;
        case 'name-taken':
            message = 'There was an error deleting the instance..';
            break;
        case 'name-toolong':
            message = 'The name you entered was too long..';
            break;
        case 'error-empty':
            message = 'The name you entered was empty..';
            break;
    }
    Swal.fire({
        title: 'There was an issue!',
        text: message,
        confirmButtonText: 'Oki!'
    });
});
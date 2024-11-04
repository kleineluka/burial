// on load, fetch from store
document.addEventListener('DOMContentLoaded', async () => {
    // load settings to get the current cached game instance
    const store = loadStorage();
    const gameInstance = await store.get('state-game-instance');
    const dropdown = document.getElementById('dropdown-menu-current-instance');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = gameInstance;
    option.text = gameInstance;
    dropdown.appendChild(option);
    // load all instances
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

// wait for a response to fill in the instances
listen('instances-loaded', (event) => {
    const data = JSON.parse(event.payload); // Accessing the JSON data
    const instances = data.instances;
    const instanceContainer = document.querySelector('.instance-container'); // Assuming .instance-container wraps all instances

    // Clear any existing content before populating
    instanceContainer.innerHTML = '';

    instances.forEach(instance => {
        // Create main instance div
        const instanceDiv = document.createElement('div');
        instanceDiv.classList.add('instance');

        // Instance header section
        const headerDiv = document.createElement('div');
        headerDiv.classList.add('instance-header');

        // Instance name section
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

        // Buttons section
        const buttonsDiv = document.createElement('div');
        buttonsDiv.classList.add('instance-buttons');

        const renameBtn = document.createElement('button');
        renameBtn.classList.add('instance-btn', 'hvr-grow', 'rename-btn');
        renameBtn.title = 'Rename this instance!';
        renameBtn.textContent = '✏️';

        const duplicateBtn = document.createElement('button');
        duplicateBtn.classList.add('instance-btn', 'hvr-grow', 'info-btn');
        duplicateBtn.title = 'Duplicate this instance!';
        duplicateBtn.textContent = '♻️';

        const deleteBtn = document.createElement('button');
        deleteBtn.classList.add('instance-btn', 'hvr-grow', 'delete-btn');
        deleteBtn.title = 'Delete this instance!';
        deleteBtn.textContent = '🗑️';

        buttonsDiv.append(renameBtn, duplicateBtn, deleteBtn);
        headerDiv.append(nameSectionDiv, buttonsDiv);

        // Stats section
        const statsDiv = document.createElement('div');
        statsDiv.classList.add('instance-stats');

        // Add index kind
        const indexKindP = document.createElement('p');
        indexKindP.classList.add('hvr-shrink');
        indexKindP.textContent = instance.index_kind;
        statsDiv.appendChild(indexKindP);

        // Conditionally add mod count
        if (instance.index_kind !== 'Vanilla Game 🍦' && instance.mod_count > 0) {
            const modCountP = document.createElement('p');
            modCountP.classList.add('hvr-shrink');
            modCountP.textContent = `${instance.mod_count} Mods`;
            statsDiv.appendChild(modCountP);
        }

        // Add last played
        const lastPlayedP = document.createElement('p');
        lastPlayedP.classList.add('hvr-shrink');
        lastPlayedP.textContent = `Last Played ${formatLastPlayed(instance.last_played)}`;
        statsDiv.appendChild(lastPlayedP);

        // Add game version
        const gameVersionP = document.createElement('p');
        gameVersionP.classList.add('hvr-shrink');
        gameVersionP.textContent = `Game Version ${instance.game_version}`;
        statsDiv.appendChild(gameVersionP);

        // Combine header and stats, then add to main container
        instanceDiv.append(headerDiv, statsDiv);
        instanceContainer.appendChild(instanceDiv);
    });
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
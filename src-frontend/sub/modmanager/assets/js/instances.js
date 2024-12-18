// on load, fetch from store
document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const instances = await store.get('instances');
    if (!instances) {
        noInstances();
        return;
    }
});

// not using instances
function noInstances() {
    // clear out + add information
    const pageContainer = document.getElementById('instances-container');
    pageContainer.innerHTML = '';
    const instanceContainer = document.createElement('div');
    instanceContainer.classList.add('instance-container');
    const instanceInfo = document.createElement('div');
    instanceInfo.classList.add('instance-info');
    const message = document.createElement('div');
    message.classList.add('loading');
    const settingsLink = "<a href='../../settings'>Settings</a>";
    message.innerHTML = 'Instances are not enabled. This is a beta feature that allows you to seperate game files between different vanilla files, mods, and modpacks. Please enable it in ' + settingsLink + ' (with caution)!';
    instanceInfo.appendChild(message);
    instanceContainer.appendChild(instanceInfo);
    pageContainer.appendChild(instanceContainer);
    const dropdown = document.getElementById('dropdown-menu-current-instance');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = 'disabled';
    option.innerHTML = 'Disabled';
    dropdown.appendChild(option);
}

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
// on load, fetch from store
document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const gameInstance = await store.get('state-game-instance');
    const dropdown = document.getElementById('dropdown-menu-current-instance');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = gameInstance;
    option.text = gameInstance;
    dropdown.appendChild(option);
});
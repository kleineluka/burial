// use tauri
const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

// use wow.js (pretty animations!)
new WOW().init();

// disable right click (shh.. there is no web app in ba sing se)
document.addEventListener('contextmenu', event => event.preventDefault());

// navigate to a different page
async function navigatePage(page) {
    await window.__TAURI__.invoke('navigate', { page });
}

// manually set the status
function set_status(status) {
    const logElement = document.getElementById('status');
    logElement.innerHTML = status;
}

// load the persistant storage
function loadStorage() {
    let store;
    try {
        store = new Store('.cache.json');
        return store; 
    } catch (error) {
        // this is more for me than the user lol
        Swal.fire({
            title: "Development Error: You forgot to include the store.min.js file!",
            text: "Burial will continue to work, but local storage will not be functional on this page.",
            toast: true,
            position: "bottom-right",
            showConfirmButton: true,
            confirmButtonText: "You should report or fix this..",
            timer: 10000,
        });
        return null; 
    }
}

// on load, see if any settings need fetched and if so pass it
window.onload = async () => {
    // load the store 
    const store = loadStorage();
    // set the metadata
    const discord = document.getElementById('discord-link');
    const github = document.getElementById('github-link');
    if (discord) {
        const metadata_discord = await store.get('metadata-discord');
        discord.href = metadata_discord;
    }
    if (github) {
        const metadata_github = await store.get('metadata-github');
        github.href = metadata_github;
    }
    // set the user settings if necessary
    const setting_tcoaal = document.getElementById('tcoaal-path');
    const setting_output = document.getElementById('output-path');
    const setting_tcoaal_classes = document.getElementsByClassName('tcoaal-path');
    const require_settings = setting_tcoaal || setting_output || setting_tcoaal_classes.length > 0;
    if (require_settings) {
        if (setting_tcoaal) {
            const tcoaal = await store.get('settings-tcoaal');
            setting_tcoaal.value = tcoaal;
        }
        if (setting_output) {
            const output = await store.get('settings-output');
            setting_output.value = output;
        }
        if (setting_tcoaal_classes.length > 0) {
            const tcoaal = await store.get('settings-tcoaal');
            for (let i = 0; i < setting_tcoaal_classes.length; i++) {
                setting_tcoaal_classes[i].value = tcoaal;
            }
        }
    }
};

// listen for button click to navigate between pages 
document.addEventListener('DOMContentLoaded', () => {
    // get all elements with the class 'navigate-button'
    const navigateButtons = document.getElementsByClassName('navigate-button');
    // loop through the collection and add event listeners
    for (let i = 0; i < navigateButtons.length; i++) {
        navigateButtons[i].addEventListener('click', (event) => {
            let pageToNavigate = event.target.dataset.page;
            navigatePage(pageToNavigate);
        });
    }
});

// listeners for various updates and conditions
listen('status', (event) => {
    const logElement = document.getElementById('status');
    logElement.innerHTML = event.payload;
});

listen('error', (event) => {
    const error = event.payload;
    Swal.fire({
        icon: "error",
        title: error,
        showConfirmButton: true
    });
});

listen('success', (event) => {
    const success = event.payload;
    Swal.fire({
        icon: "success",
        title: success,
        showConfirmButton: true
    });
});
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
    const parent = logElement.parentElement;
    parent.style.display = "block";
    logElement.innerHTML = status;
}

// clear the status
function clear_status() {
    const logElement = document.getElementById('status');
    const parent = logElement.parentElement;
    parent.style.display = "none";
    logElement.innerHTML = '';
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
    // set the theme
    const theme = await store.get('settings-theme');
    if (theme) {
        // update sidebar icons
        document.documentElement.setAttribute('data-theme', theme);
        const imgs = document.querySelectorAll('img.sidebar-icon');
        imgs.forEach(img => {
            const newSrc = img.getAttribute(`data-${theme}`);
            if (newSrc) {
                img.src = newSrc;
            }
        });
    }
    // disable animations if necessary (boring!)
    const animations = await store.get('settings-animations');
    if (!animations) {
        // add disable-animations class to body
        document.body.classList.add('disable-animations');
        // remove hvr-<animation> from all classes that have one (ex. hvr-grow, hvr-shrink, etc - it can be any of them)
        const elements = document.querySelectorAll('[class*="hvr-"]');
        elements.forEach(element => {
            const classes = element.classList;
            for (let i = 0; i < classes.length; i++) {
                if (classes[i].includes('hvr-')) {
                    element.classList.remove(classes[i]);
                }
            }
        });
    }
    // set the metadata
    const discord = document.getElementById('discord-link');
    const github = document.getElementById('github-link');
    const attributes = document.getElementById('attributions-link');	
    if (discord) {
        const metadata_discord = await store.get('metadata-discord');
        discord.href = metadata_discord;
    }
    if (github) {
        const metadata_github = await store.get('metadata-github');
        github.href = metadata_github;
        if (attributes) {
            let metadata_attributes = metadata_github + '/blob/main/ATTRIBUTIONS.md';
            attributes.href = metadata_attributes;
        }
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
    // get all elements with the class 'navigate-button' + loop through the collection and add event listeners
    const navigateButtons = document.getElementsByClassName('navigate-button');
    for (let i = 0; i < navigateButtons.length; i++) {
        navigateButtons[i].addEventListener('click', (event) => {
            let pageToNavigate = event.target.dataset.page;
            navigatePage(pageToNavigate);
        });
    }
};

// listeners for various updates and conditions
listen('status', (event) => {
    const logElement = document.getElementById('status');
    const parent = logElement.parentElement;
    parent.style.display = "block";
    logElement.innerHTML = event.payload;
});

listen('status-clear', (event) => {
    const logElement = document.getElementById('status');
    const parent = logElement.parentElement;
    parent.style.display = "none";
    logElement.innerHTML = '';
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
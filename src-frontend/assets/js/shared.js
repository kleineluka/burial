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

// fetch a url with a timeout and error handling
const fetchWithTimeout = (url, options, timeout = 5000) => {
    return Promise.race([
        fetch(url, options),
        new Promise((_, reject) =>
            setTimeout(() => reject(new Error("Request timed out")), timeout)
        ),
    ]);
};

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

// determine if we need to skip tooltips
async function skipTooltips() {
    const store = loadStorage();
    const tooltips = await store.get('settings-tooltips');
    return !tooltips;
}

// default tooltips
function defaultTooltips() {
    // github link
    tippy('#github-link', {
        content: 'View the source code on GitHub',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    // discord link
    tippy('#discord-link', {
        content: 'Join the Discord server',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    // attributions link
    tippy('#attributions-link', {
        content: 'View the attributions for Burial!',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    // launch game button
    tippy('#launch-game', {
        content: 'Launch the game',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    // some defaults we may want to file in
    // if #input-path exists, add a tooltip
    let inputPath = document.getElementById('input-path');
    if (inputPath) {
        tippy('#input-path', {
            content: 'The input path for the file or folder',
            animation: 'perspective-subtle',
            placement: 'top',
            theme: 'burial'
        });
    }
    let inputTcoaal = document.getElementById('tcoaal-path');
    if (inputTcoaal) {
        tippy('#tcoaal-path', {
            content: 'The path to the TCOAAL folder',
            animation: 'perspective-subtle',
            placement: 'top',
            theme: 'burial'
        });
    }
    let outputPath = document.getElementById('output-path');
    if (outputPath) {
        tippy('#output-path', {
            content: 'The folder to export the selected resources to',
            animation: 'perspective-subtle',
            placement: 'top',
            theme: 'burial'
        });
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
    const launcher = document.getElementById('launch-game');
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
    if (launcher) {
        // add a click event listener to the launch game button, and if clicked, invoke the launch_game function
        launcher.addEventListener('click', async () => {
            const tcoaal = await store.get('settings-tcoaal');
            invoke('launch_game', { inPath: tcoaal });
        });
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
    // if tcoaal-path exists, add an event for clicking and listener (for the file dialog)
    let tcoaalPath = document.getElementById('tcoaal-path');
    if (tcoaalPath) {
        let browseButtonTcoaal = document.getElementById('browse-button-tcoaal');
        browseButtonTcoaal.addEventListener('click', (event) => {
            invoke('folder_dialog', { emitEvent: 'selected-input-folder' });
        });
        listen('selected-input-folder', (event) => {
            tcoaalPath.value = event.payload;
        });
    }
    // if output-path exists, add an event for clicking and listener (for the file dialog)
    let outputPath = document.getElementById('output-path');
    if (outputPath) {
        let browseButtonOut = document.getElementById('browse-button-out');
        browseButtonOut.addEventListener('click', (event) => {
            invoke('folder_dialog', { emitEvent: 'selected-output-folder' });
        });
        listen('selected-output-folder', (event) => {
            outputPath.value = event.payload;
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
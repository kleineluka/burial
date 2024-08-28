// use tauri
const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

// navigate to a different page
async function navigate_page(page) {
    await window.__TAURI__.invoke('navigate', { page });
}

// listen for button click to navigate between pages 
document.addEventListener('DOMContentLoaded', () => {
    // Get all elements with the class 'navigate-button'
    const navigateButtons = document.getElementsByClassName('navigate-button');

    // Loop through the collection and add event listeners
    for (let i = 0; i < navigateButtons.length; i++) {
        navigateButtons[i].addEventListener('click', (event) => {
            let pageToNavigate = event.target.dataset.page;
            navigate_page(pageToNavigate);
        });
    }
});

// use wow.js (pretty animations!)
new WOW().init();

// disable right click (shh.. there is no web app in ba sing se)
document.addEventListener('contextmenu', event => event.preventDefault());

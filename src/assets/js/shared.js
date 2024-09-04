// use tauri
const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

// navigate to a different page
async function navigate_page(page) {
    await window.__TAURI__.invoke('navigate', { page });
}

// listen for button click to navigate between pages 
document.addEventListener('DOMContentLoaded', () => {
    // get all elements with the class 'navigate-button'
    const navigateButtons = document.getElementsByClassName('navigate-button');
    // loop through the collection and add event listeners
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

// on load, set the current version + check for updates
document.addEventListener('DOMContentLoaded', () => {
    const version = document.getElementById('version');
    if (version) {
        invoke('get_version').then((res) => {
            version.innerText = res;
            // fetch the latest version + compare (if update button is present)
            if (!document.getElementById('update-button')) return;
            let latestVersion = 'nan';
            fetch('https://raw.githubusercontent.com/kleineluka/burial/main/api/version.txt')
                .then(response => response.text())
                .then(data => {
                    console.log(data);
                    latestVersion = data;
                    if (latestVersion !== 'nan') {
                        // compare version and latest (format x.x.x)
                        const currentVersionArray = res.split('.');
                        const latestVersionArray = latestVersion.split('.');
                        let updateAvailable = false;
                        for (let i = 0; i < currentVersionArray.length; i++) {
                            if (parseInt(currentVersionArray[i]) < parseInt(latestVersionArray[i])) {
                                updateAvailable = true;
                                console.log('update available');
                                break;
                            }
                        }
                        if (updateAvailable) {
                            const updateButton = document.getElementById('update-button');
                            updateButton.classList.remove('update-button-hide');
                        }
                    }
                });
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

// manually set the status
function set_status(status) {
    const logElement = document.getElementById('status');
    logElement.innerHTML = status;
}
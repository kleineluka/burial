// compare version numbers (returns true if remote is greater than local)
function compareVersions(local, remote) {
    const localParts = local.split('.').map(Number);
    const remoteParts = remote.split('.').map(Number);
    for (let i = 0; i < localParts.length; i++) {
        if (remoteParts[i] > localParts[i]) {
            return true;
        } else if (remoteParts[i] < localParts[i]) {
            return false;
        }
    }
    return false;
}

// see if it is the first run, overrides store (but fine for index)
document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const start_tutorial = await store.get('state-first-run');
    if (start_tutorial) {
        invoke('navigate', { page: 'sub/tutorial/index.html' });
    } else {
        // check the local and remote versions
        const localVersion = await store.get('state-local-version');
        const remoteVersion = await store.get('metadata-version');
        if (compareVersions(localVersion, remoteVersion)) {
            const updateButton = document.getElementById('update-button');
            updateButton.classList.remove('update-button-hide');
        }
    }
});

// open docs
function openDocs() {
    invoke('open_browser', { url: 'https://github.com/kleineluka/burial/wiki' });
}

// move the leyley icon around randomly, like the walking animation
const centerImage = document.querySelector('.center-image');

// generate a random number in a range
function getRandomPosition(max) {
    return Math.floor(Math.random() * max);
}

// update the x/y position
function moveImageRandomly() {
    const container = centerImage.parentElement;
    const randomX = getRandomPosition(5);
    const randomY = getRandomPosition(5);
    centerImage.style.transform = `translate(${randomX}px, ${randomY}px)`;
}

// move leyley around randomly on an interval
setInterval(moveImageRandomly, 100);

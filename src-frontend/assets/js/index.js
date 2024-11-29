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

// splash text
const splashTexts = [
    "Now with 100% more siblings!",
    "I saw you eat that cake!",
    "Mmm.. rubbish tomatoes!",
    "Never say never, my dear Andrew!",
    "We aren't like that, are we?",
    "*Eating sounds*",
    "Also try Omori!",
    "Deplorable!",
    "Deflowerable?",
    "No hussies allowed!",
    "Floozies-b-gone",
    "Doormat extraordinare",
    "Very not good!",
    "Cry about it!",
    "A VHS player. Cannot be eaten.",
    "Big words don't fit in your mouth.",
    "Oh, it's mom..",
    "Now unbanned in Australia!",
    "Maybe we can make a deal?",
    "Star us on Github! Wait, can I put ads here?",
    "Don't talk about her like that.",
    "Releasing: in decades",
    "Where are your robes?"
];

function updateSplashText() {
    const splashTextElement = document.getElementById("splash-text");
    const randomIndex = Math.floor(Math.random() * splashTexts.length);
    splashTextElement.textContent = splashTexts[randomIndex];
}


// see if it is the first run, overrides store (but fine for index)
document.addEventListener('DOMContentLoaded', async () => {
    // see if first run
    const store = loadStorage();
    const start_tutorial = await store.get('state-first-run');
    if (start_tutorial) {
        invoke('navigate', { page: 'sub/tutorial/index.html' });
    } else {
        // set the theme
        const theme = await store.get('settings-theme');
        if (theme) {
            const img = document.querySelector('img.center-image');
            if (img) {
                const newSrc = img.getAttribute(`data-${theme}`);
                if (newSrc) {
                    img.src = newSrc;
                }
            }
        }
        // check the local and remote versions
        const localVersion = await store.get('state-local-version');
        const remoteVersion = await store.get('metadata-version');
        if (compareVersions(localVersion, remoteVersion)) {
            const updateButton = document.getElementById('update-button');
            updateButton.classList.remove('update-button-hide');
        }
        // start splash text
        updateSplashText();
        setInterval(updateSplashText, 3000);
    }
});
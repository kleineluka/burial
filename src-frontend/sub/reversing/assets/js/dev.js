// on load, see if dev tools are enabled
document.addEventListener('DOMContentLoaded', async function () {
    const store = loadStorage();
    const inPath = await store.get('settings-tcoaal');
    if (inPath) invoke('dev_presences', { inPath });
});

listen('devtools', (event) => {
    document.getElementById('dropdown-menu-devtools').value = event.payload ? 'enabled' : 'disabled';
});

let devData = {};
async function loadDev() {
    const response = await fetch('/data/supported/dev.json');
    devData = await response.json();
}

async function loadCode(filePath) {
    const response = await fetch(filePath);
    return await response.text();
}

// Save when the user changes the devtool settings
document.getElementById('save-devtools').addEventListener('click', async function () {
    // get the tcoaal-path value
    var inPath = document.getElementById('tcoaal-path').value;
    // get value of dropdown-menu-devtools which either has enabled or disabled selected
    var devtools = document.getElementById('dropdown-menu-devtools').value;
    var codeToggle = devtools === 'enabled';
    // Wait for loadDev to finish loading the data
    await loadDev();
    // get the code and target from the devData
    var codePath = devData['devtools'].code;
    var injectedCode = await loadCode(codePath);
    var targetLine = devData['devtools'].target;
    var codeIndent = devData['devtools'].indent;
    // invoke
    invoke("toggle_devtools", { inPath, injectedCode, targetLine, codeToggle, codeIndent });
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#refresh-tcoaal-path', {
        content: 'Refresh your game path and whether developer tools are enabled or disabled.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#devtools-label', {
        content: 'Please note: this feature requires you to install the developer SDK! You can do so in Burial in the SDK tab.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#save-devtools', {
        content: 'This will edit your TCOAAL installation to enable or disable devtools.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
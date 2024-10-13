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

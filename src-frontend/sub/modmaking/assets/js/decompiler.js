// listen for click on id decompile-button
document.getElementById('decompile-button').addEventListener('click', async () => {
    // get the user's input
    var inPath = document.getElementById('tcoaal-path').value;
    var modPath = document.getElementById('mod-path').value;
    var outPath = document.getElementById('output-path').value;
    // call the backend
    invoke('decompile_mod', { inPath, modPath, outPath} );
});

// mod path browse
document.getElementById('browse-button-mod').addEventListener('click', (event) => {
    invoke('folder_dialog', { emitEvent: 'selected-mod-path' });
});

listen('selected-mod-path', (event) => {
    document.getElementById('mod-path').value = event.payload;
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
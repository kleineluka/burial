// only allow x.x.x in mod-version
document.getElementById('mod-version').addEventListener('input', function () {
    let value = document.getElementById('mod-version').value;
    let newValue = '';
    for (let i = 0; i < value.length; i++) {
        if (value[i].match(/[0-9.]/)) {
            newValue += value[i];
        }
    }
    document.getElementById('mod-version').value = newValue;
});

// listen for convert-button and call backend
document.getElementById('convert-button').addEventListener('click', async function () {
    var inPath = document.getElementById('mod-path').value;
    var gamePath = document.getElementById('tcoaal-path').value;
    var outPath = document.getElementById('output-path').value;
    var modName = document.getElementById('mod-name').value || 'my_mod';
    var modId = document.getElementById('mod-id').value || 'my_converted_mod';
    var modAuthors = document.getElementById('mod-author').value || 'burial_converted';
    var modDescription = document.getElementById('mod-description').value || 'Converted by Burial';
    var modVersion = document.getElementById('mod-version').value || '1.0.0';
    invoke('convert_mod', { inPath, gamePath, outPath, modName, modId, modAuthors, modDescription, modVersion });
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
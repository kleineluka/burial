// listen for dropdown change at dropdown-menu-direction
document.getElementById('dropdown-menu-direction').addEventListener('change', function() {
    var direction = document.getElementById('dropdown-menu-direction').value;
    if (direction === 'rpg') {
        document.getElementById('rpg-container').classList.remove('hidden');
        document.getElementById('mod-container').classList.add('hidden');
    } else if (direction === 'mod') {
        document.getElementById('rpg-container').classList.add('hidden');
        document.getElementById('mod-container').classList.remove('hidden');
    }
});

// only allow x.x.x in mod-version
document.getElementById('mod-version').addEventListener('input', function() {
    let value = document.getElementById('mod-version').value;
    let newValue = '';
    for (let i = 0; i < value.length; i++) {
        if (value[i].match(/[0-9.]/)) {
            newValue += value[i];
        }
    }
    document.getElementById('mod-version').value = newValue;
});

// convert game to project on backend
document.getElementById('project-button').addEventListener('click', function() {
    // get the in path
    var inPath = document.getElementById('tcoaal-path').value;
    var outPath = document.getElementById('output-path').value;
    invoke('export_rpg_project', { inPath, outPath });
});

// listen for click on project-button (call to export rpg)
document.getElementById('project-button').addEventListener('click', function() {
    // get the in path
    var inPath = document.getElementById('tcoaal-path').value;
    var outPath = document.getElementById('output-path').value;
    var projectName = document.getElementById('project-name').value;
    if (projectName === '') projectName = 'my_project';
    invoke('export_rpg_project', { inPath, outPath, projectName });
});

// listen for click on mod-button (call to export mod)
document.getElementById('mod-button').addEventListener('click', async function () {
    // get the in path
    var inPath = document.getElementById('rpg-project-path').value;
    var gamePath = document.getElementById('tcoaal-path-mod').value;
    var outPath = document.getElementById('mod-path').value;
    var folderName = document.getElementById('mod-folder').value;
    let modVersion = document.getElementById('mod-version').value;
    var autoZip = (document.getElementById('dropdown-auto-zip').value === 'true');
    if (folderName === '') folderName = 'my_project'; // default, not required
    if (modVersion === '') modVersion = '1.0.0'; // default, not required
    // see if we can get default mod properties from settings
    let store = loadStorage();
    let modName = (await store.get('settings-modname')) || 'My Mod';
    let modId = (await store.get('settings-modid')) || 'my_mod';
    let modAuthor = (await store.get('settings-modauthor')) || 'The Author';
    let modDescription = (await store.get('settings-moddescription')) || 'A mod that certainly does something..';
    invoke('export_mod_folder', { inPath, gamePath, outPath, folderName, autoZip, modName, modId, modAuthor, modDescription, modVersion });
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#dropdown-direction-label', {
        content: 'Game to RPG Maker will dump your TCOAAL into an RPG Maker MV project to be edited. RPG Maker to Mod will turn your RPG Maker project into a Tomb-compatible mod.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
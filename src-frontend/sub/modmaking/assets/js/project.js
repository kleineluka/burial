// liten for dropdown change at dropdown-menu-direction
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

// convert game to project on backend
document.getElementById('project-button').addEventListener('click', function() {
    // get the in path
    var inPath = document.getElementById('tcoaal-path').value;
    var outPath = document.getElementById('output-path').value;
    invoke('export_rpg_project', { inPath, outPath });
});
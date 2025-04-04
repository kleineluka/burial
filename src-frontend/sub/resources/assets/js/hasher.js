// switch between horizontal navbars
document.addEventListener('DOMContentLoaded', () => {
    const navOptions = document.querySelectorAll('.page-navbar-option');
    const subContainers = document.querySelectorAll('.page-container');
    navOptions.forEach(option => {
        option.addEventListener('click', (event) => {
            event.preventDefault();
            // clear current selection
            navOptions.forEach(nav => nav.classList.remove('selected'));
            subContainers.forEach(container => container.classList.add('hidden-container'));
            // show what was selected
            option.classList.add('selected');
            const id = option.id;
            const subContainer = document.getElementById(`sub-${id}`);
            if (subContainer) {
                subContainer.classList.remove('hidden-container');
            }
        });
    });
});

// listen to put the selected data back in the input path
document.addEventListener('DOMContentLoaded', () => {
    listen('selected-input-folder', (event) => {
        document.getElementById('input-path').value = event.payload;
    });
});

// do the encryption
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('encrypt-button').addEventListener('click', () => {
        // get if select-file or select-folder is selected
        const folderButton = document.getElementById('select-folder');
        const fileButton = document.getElementById('select-file');
        let pathKind = (folderButton.classList.contains('selected')) ? 'folder' : 'file';
        // check if we are decrypting the whole file or just the original byte positions
        const wholeButton = document.getElementById('select-byte-whole');
        const originalButton = document.getElementById('select-byte-original');
        let advancedPositions = (originalButton.classList.contains('selected')) ? true : false;
        // get paths and send to rust
        const inPath = document.getElementById('input-path').value;
        const outPath = document.getElementById('output-path').value;
        invoke('encrypt', { pathKind, inPath, outPath, advancedPositions });
    });
});

// switch between encrypting a single file and decrypting a folder
document.addEventListener('DOMContentLoaded', () => {
    const tohashButton = document.getElementById('select-tohash');
    const fromhashButton = document.getElementById('select-fromhash');
    tohashButton.addEventListener('click', () => {
        tohashButton.classList.add('selected');
        fromhashButton.classList.remove('selected');
    });
    fromhashButton.addEventListener('click', () => {
        fromhashButton.classList.add('selected');
        tohashButton.classList.remove('selected');
    });
});
   
// if the user selects a file type, show the custom input field
document.addEventListener('DOMContentLoaded', () => {
    const dropdown = document.getElementById('dropdown-file-type');
    const customInput = document.getElementById('custom');
    dropdown.addEventListener('change', () => {
        if (dropdown.value === 'custom') {
            customInput.classList.remove('hidden');
        } else {
            customInput.classList.add('hidden');
        }
    });
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();s
});

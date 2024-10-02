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

// automagically populate dropdown menu for supported dialogues
let supportedExports = [];
let supportedImports = [];
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/dialogue.json')
        .then(response => response.json())
        .then(data => {
            supportedExports = data.export;
            supportedImports = data.import;
            // first, handle exports
            const language_export_dropdown = document.getElementById('dropdown-menu-export-language');
            Object.keys(supportedExports.languages).forEach(language => {
                const option = document.createElement('option');
                option.value = language; 
                option.innerText = language; 
                language_export_dropdown.appendChild(option);
            });
            const contents_export_dropdown = document.getElementById('dropdown-menu-export-contents');
            Object.keys(supportedExports.contents).forEach(contents => {
                const option = document.createElement('option');
                option.value = contents; 
                option.innerText = contents; 
                contents_export_dropdown.appendChild(option);
            });
            const formats_export_dropdown = document.getElementById('dropdown-menu-export-format');
            Object.keys(supportedExports.formats).forEach(formats => {
                const option = document.createElement('option');
                option.value = formats; 
                option.innerText = formats; 
                formats_export_dropdown.appendChild(option);
            });
            // then imports
            const language_import_dropdown = document.getElementById('dropdown-menu-import-language');
            Object.keys(supportedImports.languages).forEach(language => {
                console.log(language);
                const option = document.createElement('option');
                option.value = language; 
                option.innerText = language; 
                language_import_dropdown.appendChild(option);
            });
            const formats_import_dropdown = document.getElementById('dropdown-menu-import-format');
            Object.keys(supportedImports.formats).forEach(contents => {
                const option = document.createElement('option');
                option.value = contents; 
                option.innerText = contents; 
                formats_import_dropdown.appendChild(option);
            });
            const contents_import_dropdown = document.getElementById('dropdown-menu-import-contents');
            Object.keys(supportedImports.contents).forEach(formats => {
                const option = document.createElement('option');
                option.value = formats; 
                option.innerText = formats; 
                contents_import_dropdown.appendChild(option);
            });
        })
        .catch(error => {
            console.error('Error fetching the JSON data:', error);
        });
});

// export dialogue
function exportDialogue() {
    // get selected language, contents, and format
    const language = document.getElementById('dropdown-menu-export-language').value;
    const contents = document.getElementById('dropdown-menu-export-contents').value;
    const format = document.getElementById('dropdown-menu-export-format').value;
    // get from json
    if (!supportedExports) {
        Swal.fire({
            icon: 'error',
            title: 'Error',
            text: 'The JSON data could not be loaded.',
        });
        return;
    }
    const languageDetails = supportedExports.languages[language];
    const contentDetails = supportedExports.contents[contents];
    const formatDetails = supportedExports.formats[format];
    // get path
    const inPath = document.getElementById('tcoaal-path').value;
    const outPath = document.getElementById('output-path').value;
    // call back end
    invoke ("export_dialogue", { inPath, outPath, languageDetails, contentDetails, formatDetails });
}

// show a preview of the exported dialogue
function previewExport() {
    // control which elements are visible
    const exportMain = document.getElementById('export-main');
    const navbarMain = document.getElementById('navbar-main');
    const previewMain = document.getElementById('export-preview');
    const navbarPreview = document.getElementById('navbar-preview');
    // hide the main, show the preview
    exportMain.classList.add('hidden-container');
    navbarMain.classList.add('hidden-container');
    previewMain.classList.remove('hidden-container');
    navbarPreview.classList.remove('hidden-container');
    // get selected language, contents, and format
    const language = document.getElementById('dropdown-menu-export-language').value;
    const contents = document.getElementById('dropdown-menu-export-contents').value;
    const format = document.getElementById('dropdown-menu-export-format').value;
    // get from json
    if (!supportedExports) {
        Swal.fire({
            icon: 'error',
            title: 'Error',
            text: 'The JSON data could not be loaded.',
        });
        return;
    }
    const languageDetails = supportedExports.languages[language];
    const contentDetails = supportedExports.contents[contents];
    const formatDetails = supportedExports.formats[format];
    // get path
    const inPath = document.getElementById('tcoaal-path').value;
    // call back end
    invoke("preview_export", { inPath, languageDetails, contentDetails, formatDetails });
}

// listen for loading the preview dialogue
let dialogueContent = '';
listen('load-preview', (event) => {
    dialogueContent = event.payload;
    document.getElementById('textarea-dialogue').value = dialogueContent;
});

// exit the preview
function exitPreview() {
    // control which elements are visible
    const exportMain = document.getElementById('export-main');
    const navbarMain = document.getElementById('navbar-main');
    const previewMain = document.getElementById('export-preview');
    const navbarPreview = document.getElementById('navbar-preview');
    // hide the preview, show the main
    exportMain.classList.remove('hidden-container');
    navbarMain.classList.remove('hidden-container');
    previewMain.classList.add('hidden-container');
    navbarPreview.classList.add('hidden-container');
}
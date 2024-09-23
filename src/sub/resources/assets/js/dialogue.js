// switch between horizontal navbars
document.addEventListener('DOMContentLoaded', () => {
    const navOptions = document.querySelectorAll('.page-navbar-option');
    const subContainers = document.querySelectorAll('.page-container');
    navOptions.forEach(option => {
        option.addEventListener('click', (event) => {
            event.preventDefault();
            // clear current selection
            navOptions.forEach(nav => nav.classList.remove('selected'));
            subContainers.forEach(container => container.classList.add('hidden'));
            // show what was selected
            option.classList.add('selected');
            const id = option.id;
            const subContainer = document.getElementById(`sub-${id}`);
            if (subContainer) {
                subContainer.classList.remove('hidden');
            }
        });
    });
});

// automagically populate dropdown menu for supported dialogues
let supportedDialogues = [];
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/dialogue.json')
        .then(response => response.json())
        .then(data => {
            supportedDialogues = data;
            // set the language dropdown
            const language_dropdown = document.getElementById('dropdown-menu-language');
            Object.keys(supportedDialogues.languages).forEach(language => {
                const option = document.createElement('option');
                option.value = language; 
                option.innerText = language; 
                language_dropdown.appendChild(option);
            });
            // set the contents dropdown
            const contents_dropdown = document.getElementById('dropdown-menu-contents');
            Object.keys(supportedDialogues.contents).forEach(language => {
                const option = document.createElement('option');
                option.value = language; 
                option.innerText = language; 
                contents_dropdown.appendChild(option);
            });
            // set the formats dropdown
            const formats_dropdown = document.getElementById('dropdown-menu-format');
            Object.keys(supportedDialogues.formats).forEach(language => {
                const option = document.createElement('option');
                option.value = language; 
                option.innerText = language; 
                formats_dropdown.appendChild(option);
            });
        })
        .catch(error => {
            console.error('Error fetching the JSON data:', error);
        });
});

// export dialogue
function exportDialogue() {
    // get selected language, contents, and format
    const language = document.getElementById('dropdown-menu-language').value;
    const contents = document.getElementById('dropdown-menu-contents').value;
    const format = document.getElementById('dropdown-menu-format').value;
    // get from json
    if (!supportedDialogues) {
        Swal.fire({
            icon: 'error',
            title: 'Error',
            text: 'The JSON data could not be loaded.',
        });
        return;
    }
    const languageDetails = supportedDialogues.languages[language];
    const contentDetails = supportedDialogues.contents[contents];
    const formatDetails = supportedDialogues.formats[format];
    // get path
    const inPath = document.getElementById('tcoaal-path').value;
    const outPath = document.getElementById('output-path').value;
    // call back end
    invoke ("export_dialogue", { inPath, outPath, languageDetails, contentDetails, formatDetails });
}
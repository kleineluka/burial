// automagically populate dropdown menu for supported difference formats
let supportedDifferences = [];
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/differences.json')
        .then(response => response.json())
        .then(data => {
            supportedDifferences = Object.keys(data);
            const dropdown = document.getElementById('dropdown-menu-difference');
            supportedDifferences.forEach((difference) => {
                const option = document.createElement('option');
                option.value = difference;
                option.text = difference;
                dropdown.appendChild(option);
            });
        }
    );
});

// generate differences
function createDifferences() {
    // get user input
    const format = document.getElementById('dropdown-menu-difference').value;
    const modOne = document.getElementById('mod-one-path').value;
    const modTwo = document.getElementById('mod-two-path').value;
    const outputPath = document.getElementById('output-path').value;
    // if any input is missing, alert user
    if (!format || !modOne || !modTwo || !outputPath) {
        Swal.fire({
            title: 'Wait a second!',
            text: 'Please fill out all of the fields..',
            confirmButtonText: 'Oki!'
        });
        return;
    }
    // pass to backend
    invoke('find_differences', { modOne, modTwo, format, outputPath } );
}

// listen for backend response
listen('diff-result', (event) => {
    console.log(event.payload);
});

// mod one path browse
document.getElementById('browse-button-mod-one').addEventListener('click', (event) => {
    invoke('file_dialog', { emitEvent: 'selected-mod-one', fileType: 'zip' });
});

listen('selected-mod-one', (event) => {
    document.getElementById('mod-one-path').value = event.payload;
});

// mod two path browse
document.getElementById('browse-button-mod-two').addEventListener('click', (event) => {
    invoke('file_dialog', { emitEvent: 'selected-mod-two', fileType: 'zip' });
});

listen('selected-mod-two', (event) => {
    document.getElementById('mod-two-path').value = event.payload;
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#difference-format-label', {
        content: 'This will control how the file outputted will be structured.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#mod-one-path', {
        content: 'The path to the .zip file of the first mod. Consider this the \"old\" or \"original\" mod.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#mod-two-path', {
        content: 'The path to the .zip file of the second mod. Consider this the \"new\" or \"updated\" mod.',  
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
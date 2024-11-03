// automagically populate dropdown menu for supported difference formats
let supportedDifferences = [];
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/differences.json')
        .then(response => response.json())
        .then(data => {
            supportedDifferences = Object.keys(data);
            console.log(supportedDifferences);
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
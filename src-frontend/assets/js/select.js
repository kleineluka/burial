// command tauri to open a file browser
async function select_folder() {
    await window.__TAURI__.invoke('folderwalk', {} );
}

// listen for button click to move to index.html on back
document.getElementById('browse-button').addEventListener('click', (event) => {
    select_folder();
});

// listen to put the selected data back in the path
document.addEventListener('DOMContentLoaded', () => {
    listen('selected-folder', (event) => {
        document.querySelector('.file-path-input').value = event.payload;
    });
});

// install/uninstall buttons need to ensure user has selected a folder
document.addEventListener('DOMContentLoaded', () => {
    // Function to call the verify_path command and handle the result
    async function checkPath(filePath, buttonPage) {
        // first, see if the passed filepath is empty and THEN call the verify_path command
        let result = (filePath === '') ? false : true;
        if (result) result = await window.__TAURI__.invoke('verify_path', { filePath: filePath });
        // send an error or continue to redirect, 
        if (!result) {
            Swal.fire({
                icon: "error",
                title: "Please select a valid game folder!",
                showConfirmButton: true
            });
            return;
        }
        // now, we can redirect based on the button clicked (from the data-page)
        navigate_page(buttonPage);
    }

    // Add event listeners to all buttons with the class 'conditional-button'
    document.querySelectorAll('.conditional-button').forEach(button => {
        button.addEventListener('click', () => {
            // Get the file path from the input field
            const filePath = document.getElementById('game-path').value;
            const buttonPage = button.getAttribute('data-page');
            checkPath(filePath, buttonPage);
        });
    });
});

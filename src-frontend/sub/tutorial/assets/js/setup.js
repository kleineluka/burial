/* Where Is The Game Installed? Wiki Page */
function findInstall() {
    invoke('open_browser', { url: 'https://github.com/kleineluka/burial/wiki/Installation-and-Help#how-can-i-find-my-tcoaal-installation-folder' });
}

/* Delayed Annotation (animate.css breaks it */
function delayedAnnotation() {
    const title = document.getElementById('change-later');
    const annotation = RoughNotation.annotate(title, { type: 'highlight', color: '#F1E2C5', padding: 200, strokeWidth: 1, iterations: 3 });
    annotation.show();
}
setTimeout(function() {
    delayedAnnotation();
}, 2500);

/* Check Game Path */
document.getElementById('steps-one-button').addEventListener('click', function () {
    // send the path to backend + local storage
    const inPath = document.getElementById('tcoaal-path').value;
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', inPath);
    store.save();
    invoke('setup_game', { inPath });
});

/* Listen for Results of Game Path */
listen('game-status', (event) => {
    /* four options: empty, valid, invalid, saved */
    const inPath = document.getElementById('tcoaal-path').value;
    const status = event.payload;
    switch (status) {
        case 'empty':
            Swal.fire({
                title: "Your game path is empty!",
                text: "You can leave this blank for now and come back to it later in the Settings menu. Do you want to proceed?",
                showCancelButton: true,
                confirmButtonText: "Continue",
                reverseButtons: true,
                confirmButtonColor: "#F595B2",
            }).then((result) => {
                if (result.isConfirmed) {
                    invoke('setup_settings', { inPath });
                }
            });
            break;
        case 'valid':
            // ignore, for now
            break;
        case 'invalid':
            Swal.fire({
                title: "Your game path is invalid!",
                text: "It doesn't seem like a TCOAAL installation. But you can always change it later in the Settings menu. Do you want to proceed?",
                showCancelButton: true,
                confirmButtonText: "Continue",
                reverseButtons: true,
                confirmButtonColor: "#F595B2",
            }).then((result) => {
                if (result.isConfirmed) {
                    invoke('setup_settings', { inPath });
                }
            });
            break;
        case 'saved':
            // move to next step
            invoke('navigate', { page: 'skip.html' });
            break;
    }
});
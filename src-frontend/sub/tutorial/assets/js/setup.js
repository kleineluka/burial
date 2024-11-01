// on load, fire a find game path event use dom to load
document.addEventListener('DOMContentLoaded', function () {
    // wait 2 seconds before firing the event to avoid store conflict..
    setTimeout(function() {
        invoke('auto_find_game', {}); 
    }, 3000);
});

// and in return, listen for game-path
listen('game-path', (event) => {
    if (event.payload != 'empty') {
        const gamePath = event.payload;
        //gamePath = gamePath.replace(/\\\\/g, '\\');
        document.getElementById('tcoaal-path').value = gamePath;
        Swal.fire({
            title: "TCOAAL Found!",
            text: "We autofilled it for you! :)",
            toast: true,
            position: "bottom-right",
            showConfirmButton: true,
            confirmButtonText: "Yay!",
            timer: 5000,
        });
    } else {
        document.getElementById('tcoaal-path').placeholder = 'Your TCOAAL Folder';
    }
});

// open wiki page for help locating the game installation
function findInstall() {
    invoke('open_browser', { url: 'https://github.com/kleineluka/burial/wiki/Installation-and-Help#how-can-i-find-my-tcoaal-installation-folder' });
}

// delayed annotation since animation.css breaks it
function delayedAnnotation() {
    const title = document.getElementById('change-later');
    const annotation = RoughNotation.annotate(title, { type: 'highlight', color: '#F1E2C5', padding: 200, strokeWidth: 1, iterations: 3 });
    annotation.show();
}
setTimeout(function() {
    delayedAnnotation();
}, 2500);

// validate game path
document.getElementById('steps-one-button').addEventListener('click', function () {
    // send the path to backend + local storage
    const inPath = document.getElementById('tcoaal-path').value;
    const store = new Store('.cache.json');
    store.set('settings-tcoaal', inPath);
    store.save();
    invoke('setup_game', { inPath });
});

// listen for game path results
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
// listen for click on download-button to download the mod
document.getElementById('download-button').addEventListener('click', async () => {
    // get path and mod url
    let inPath = await loadStorage().get('settings-tcoaal');
    let modUrl = document.getElementById('external-mod-url').value;
    // ask first if they are sure they want to download an external mod
    Swal.fire({
        title: 'Hey, listen!',
        text: 'Are you sure you want to download an external mod? Make sure you trust where you are getting it from as Burial does not vet, check, or approve these mods. There also may be bugs or game-breaking issues as Burial will automatically recompile the mod for your game.',
        showCancelButton: true,
        confirmButtonText: 'Yes',
        cancelButtonText: 'No',
        confirmButtonColor: "var(--main-colour"
    }).then((result) => {
        if (result.isConfirmed) {
            invoke('download_external_mod', { inPath, modUrl });
        } else {
            set_status("No worries! Burial won\'t download the mod.")
        }
    });
});

// the mod information was loaded.. now act accordingly
listen('external-mod-source', async (event) => {
    switch (event.payload) {
        case 'Gamebanana':
            document.getElementById('sub-main').classList.add('hidden');
            document.getElementById('sub-gamebanana').classList.remove('hidden');
            break;
        case "Archived":
            document.getElementById('sub-main').classList.add('hidden');
            document.getElementById('sub-archived').classList.remove('hidden');
            break;
        default: // "Unsupported"
            // unsupported by default
            Swal.fire({
                title: 'Unsupported Mod Source',
                text: 'The mod source is not supported by the mod manager.',
                showConfirmButton: false,
                confirmButtonColor: "--var(main-colour)",
                timer: 3000
            });
            break;
    }
});

// the mod was installed
listen('external-mod-downloaded', async (event) => {
    // possible responses: nogame, notomb, unsupported, success, unsupported
    // first, add hidden to all sub menus and show the main menu
    document.getElementById('sub-main').classList.remove('hidden');
    document.getElementById('sub-gamebanana').classList.add('hidden');
    document.getElementById('sub-archived').classList.add('hidden');
    // now, determine the message based on the response
    let message = '';
    switch (event.payload) {
        case 'nogame':
            message = 'The game could not be found. Please select the game folder in the settings.';
            break;
        case 'notomb':
            message = 'You don\'t have the Tomb modloader installed. Please visit the Mod Loader 🪦 tab to install it first.';
            break;
        case 'unsupported':
            message = 'This URL isn\'t supported yet - sorry.. maybe you could help us add support for it?';
            break;
        case 'success':
            message = 'The mod was successfully installed! Enjoy ^_^';
            break;
        default:
            message = 'An error occurred while installing the mod... I\'m sorry..';	
            break;
    }
    // show the message
    Swal.fire({
        icon: event.payload === 'success' ? 'success' : 'error',
        title: event.payload === 'success' ? 'Mod Installed' : 'Error',
        text: message,
        showConfirmButton: false,
        confirmButtonText: 'Oki..',
        confirmButtonColor: "--var(main-colour)",
        timer: 3000
    });
});

// on load, set up loader animation
document.addEventListener('DOMContentLoaded', () => {
    const kaomojiList = [
        // cheerful swaying
        "(〜￣△￣)〜",
        "〜(￣△￣〜)",
        "(〜￣△￣〜)",
        // energetic dance
        "ヽ(⌐■_■)ノ♪",
        "♪ヽ(⌐■_■)ノ",
        "ヽ(⌐■_■)ノ",
        //shimmy dance
        "(o^-^)o",
        "o(^-^o)",
        "(o^-^)o",
        // cute grooves
        "(>‿<✿)",
        "(✿>‿<)",
        "(>‿<✿)",
        // party Mode
        "(ノ^o^)ノ",
        "(ノ^_^)ノ",
        "(ノ^o^)ノ",
        // wave Dancer
        "~(˘▾˘~)",
        "(~˘▾˘)~",
        "~(˘▾˘~)"
    ];
    const kaomojiLoader = document.querySelector('.kaomoji-loader');
    let index = 0;
    setInterval(() => {
        kaomojiLoader.textContent = kaomojiList[index];
        index = (index + 1) % kaomojiList.length; 
    }, 300);
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
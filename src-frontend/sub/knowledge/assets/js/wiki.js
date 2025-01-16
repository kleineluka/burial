// open the coffin modding wiki
function openCoffinWiki() {
    invoke('open_browser', { url: 'https://coffin-wiki.basil.cafe/' });
}

// how to install mods (manually)
function openPlayingMods() {
    invoke('open_browser', { url: 'https://coffin-wiki.basil.cafe/playing' });
}

// how to make mods for tomb
function openMakingMods() {
    invoke('open_browser', { url: 'https://coffin-wiki.basil.cafe/modding' });
}

// publish your mods here..
function openPublishMod() {
    invoke('open_browser', { url: 'https://github.com/Llamaware/Llamaware.github.io/' });
}

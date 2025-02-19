async function openBurialItch() {
    const store = loadStorage();
    const burial_itchio = await store.get('metadata-itchio');
    invoke('open_browser', { url: burial_itchio });
}

async function openBurialGameBanana() {
    const store = loadStorage();
    const burial_gamebanana = await store.get('metadata-gamebanana');
    invoke('open_browser', { url: burial_gamebanana });
}

async function openBurialNexusMods() {
    const store = loadStorage();
    const burial_nexusmods = await store.get('metadata-nexusmods');
    invoke('open_browser', { url: burial_nexusmods });
}

async function openBurialWebsite() {
    const store = loadStorage();
    const burial_website = await store.get('metadata-website');
    invoke('open_browser', { url: burial_website });
}

function openBurialWiki() {
    invoke('open_browser', { url: 'https://github.com/kleineluka/burial/wiki' });
}

function openBurialContributing() {
    invoke('open_browser', { url: 'https://github.com/kleineluka/burial/wiki/Contributing' });
}
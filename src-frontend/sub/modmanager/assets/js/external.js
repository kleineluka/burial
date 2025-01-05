// listen for click on download-button to download the mod
document.getElementById('download-button').addEventListener('click', async () => {
    // get path and mod url
    let inPath = await loadStorage().get('settings-tcoaal');
    let modUrl = document.getElementById('external-mod-url').value;
    invoke ('download_external_mod', { inPath, modUrl });
});

// the mod information was loaded.. now act accordingly
listen('mod-source', async (event) => {
    console.log(event.payload);
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
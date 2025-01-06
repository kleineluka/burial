// links..
async function openKleineLukaWebsite() {
    invoke('open_browser', { url: "https://www.luka.moe" });
}

async function openKleineLukGithub() {
    invoke('open_browser', { url: "https://www.luka.moe/go/github" });
}

async function openKleineLukaContacts() {
    invoke('open_browser', { url: "https://www.luka.moe/contact" });
}

async function openKleineLukaYoutube() {
    invoke('open_browser', { url: "https://www.luka.moe/go/youtube" });
}

// get different texts for the background, ehehhe..
function differentText() {
    const texts = [
        "(｡♥‿♥｡)",
        "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧",
        "(≧◡≦)",
        "(╯✧▽✧)╯",
        "(* ^ ω ^)",
        "(づ￣ ³￣)づ",
        "(ʘ‿ʘ)",
        "（。＾▽＾）",
        "╰(°▽°)╯",
        "(✿◠‿◠)",
        "(❁´◡`❁)",
        "(♥ω♥*)",
        "(☆▽☆)",
        "(*≧ω≦)",
        "(⌒‿⌒)",
        "(─‿‿─)",
        "(￣▽￣)",
        "(╯▽╰ )",
        "(ᗒᗨᗕ)",
        "♡( ◡‿◡ )"
    ];
    return texts[Math.floor(Math.random() * texts.length)];
}

// on load, add background effect
document.addEventListener('DOMContentLoaded', () => {
    // create an amount of rows according to the window height
    const backgroundEffect = document.querySelector('.kaomoji-wall');
    const rowHeight = 32;
    let currentRows = 0;
    function updateRows() {
        const rowsNeeded = Math.ceil(window.innerHeight / rowHeight);
        while (currentRows < rowsNeeded) {
            const row = document.createElement('div');
            row.classList.add('scrolling-row');
            let text = differentText();
            for (let i = 0; i < 20; i++) {
                text += ' ' + differentText();
            }
            let finalText = '<span>' + text + '</span> ';
            row.innerHTML = finalText;
            backgroundEffect.appendChild(row);
            currentRows++;
        }
        // remove extra rows if needed
        while (currentRows > rowsNeeded) {
            backgroundEffect.removeChild(backgroundEffect.lastChild);
            currentRows--;
        }
    }
    updateRows();
    // update rows on window resize with debouncing for performance
    let resizeTimeout;
    window.addEventListener('resize', () => {
        clearTimeout(resizeTimeout);
        resizeTimeout = setTimeout(() => {
            updateRows();
        }, 2000); // debounce delay
    });
});
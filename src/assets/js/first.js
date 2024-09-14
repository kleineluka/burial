// move the leyley icon around randomly, like the walking animation
const centerImage = document.querySelector('.center-image');

// generate a random number in a range
function getRandomPosition(max) {
    return Math.floor(Math.random() * max);
}

// update the x/y position
function moveImageRandomly() {
    const container = centerImage.parentElement;
    const randomX = getRandomPosition(5);
    const randomY = getRandomPosition(5);
    centerImage.style.transform = `translate(${randomX}px, ${randomY}px)`;
}

// move leyley around randomly on an interval
setInterval(moveImageRandomly, 100);

// typewriter effect (+ with emoji support via grapheme)
const stringSplitter = string => {
    const splitter = new GraphemeSplitter();
    return splitter.splitGraphemes(string);
};
var typewriterid = document.getElementById('type');
var typewriter = new Typewriter(typewriterid, {
    loop: true,
    delay: 100,
    autoStart: true,
    cursor: '|',
    stringSplitter
});
typewriter
    .typeString('install TCOAAL mods 📦')
    .pauseFor(1000)
    .deleteAll(60)
    .typeString('decrypt game files 🔪')
    .pauseFor(1000)
    .deleteAll(60)
    .typeString('edit save files ✏️')
    .pauseFor(1000)
    .deleteAll(60)
    .typeString('export game code 💘')
    .pauseFor(1000)
    .deleteAll(60)
    .typeString('create sprite templates 🥺')
    .pauseFor(1000)
    .deleteAll(60)
    .typeString('manage SDKs 🍡')
    .pauseFor(1000)
    .deleteAll(60)
    .typeString('learn how TCOAAL works 📔')
    .pauseFor(1000)
    .deleteAll(60)
    .typeString('encrypt your own files 🩹')
    .pauseFor(1000)
    .start();

// underline "Burial"
const title = document.getElementById('name');
const annotation = RoughNotation.annotate(title, { type: 'underline' });
annotation.show();
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

// open docs
function openDocs() {
    invoke('open_browser', { url: 'https://github.com/kleineluka/burial/wiki'});
}
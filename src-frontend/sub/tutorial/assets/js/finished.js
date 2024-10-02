/* Confetti! */
const confettiContainer = document.querySelector('.confetti-container');
function createConfetti() {
    const confetti = document.createElement('div');
    confetti.classList.add('confetti');
    const colors = ['#ff69b4', '#ffd700', '#4b0082', '#00ff7f', '#00bfff'];
    confetti.style.backgroundColor = colors[Math.floor(Math.random() * colors.length)];
    // random position + fall duration
    confetti.style.left = `${Math.random() * 100}vw`;
    confetti.style.animationDuration = `${Math.random() * 2 + 3}s`;
    confettiContainer.appendChild(confetti);
    // remove after fallen
    setTimeout(() => {
        confetti.remove();
    }, 5000);
}
// every 100ms create confetti
setInterval(createConfetti, 100);

/* Finish Tutorial */
function finishTutorial() {
    invoke("setup_finish", {});
}

listen('setup-status', (event) => {
    if (event.payload === 'finished') {
        const store = new Store('.cache.json');
        store.set('state-first-run', false);
        store.save();
        invoke('navigate', { page: "../../index.html" });
    }
});
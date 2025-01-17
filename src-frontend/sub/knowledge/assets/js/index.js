// rotating header of lil mojis
const imagePool = [
    "../../assets/base/ashley_bunny.base64",
    "../../assets/base/bag.base64",
    "../../assets/base/book.base64",
    "../../assets/base/candle.base64",
    "../../assets/base/chest.base64",
    "../../assets/base/demon_circle.base64",
    "../../assets/base/demon.base64",
    "../../assets/base/eye.base64",
    "../../assets/base/flowers.base64",
    "../../assets/base/gift.base64",
    "../../assets/base/green_bunny.base64",
    "../../assets/base/milk.base64",
    "../../assets/base/mop.base64",
    "../../assets/base/music.base64",
    "../../assets/base/sun.base64"
];
const visibleCount = 7;
const imageRow = document.getElementById("smol-moji-row");

// load the image
async function loadBase64(filePath) {
    const response = await fetch(filePath);
    const rawBase64 = await response.text();
    const base64Prefix = "data:image/png;base64,";
    return base64Prefix + rawBase64.trim();
}

// initialize the row
async function initializeRow() {
    for (let i = 0; i < visibleCount; i++) {
        const img = document.createElement("img");
        img.className = "smol-moji";
        img.style.transform = `rotate(${i % 2 === 0 ? "-5deg" : "5deg"})`;
        const base64Data = await loadBase64(imagePool[i % imagePool.length]); // Cycle through pool
        img.src = base64Data;
        imageRow.appendChild(img);
    }
    startCycling();
}

// cycle through the images
async function startCycling() {
    let currentIndex = visibleCount;
    setInterval(async () => {
        // remove the first image to off screen
        const firstImage = imageRow.firstElementChild;
        firstImage.remove();
        // add the new image to the end
        const img = document.createElement("img");
        img.className = "smol-moji";
        const images = Array.from(imageRow.children);
        images.forEach((image, index) => {
            image.style.transform = `rotate(${index % 2 === 0 ? "-5deg" : "5deg"})`;
        });
        // cycle through the pool
        const base64Data = await loadBase64(imagePool[currentIndex % imagePool.length]);
        img.src = base64Data;
        imageRow.appendChild(img);
        currentIndex++;
        // slide the row back to show the images naturally
        imageRow.style.transform = "translateX(0)";
    }, 1000);
}

// start the row
initializeRow();

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
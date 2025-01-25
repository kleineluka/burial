let news_status = false;
let news_data = [];

// load the visions repo on load
document.addEventListener('DOMContentLoaded', async () => {
    // get the news repo link
    const storage = loadStorage();
    let newsRepo = await storage.get('metadata-news-server');
    // fetch data as json from repo
    if (newsRepo) {
        news_data = await fetch("https://codeberg.org/api/v1/repos/peachy/visions/raw/mods.json").then(res => res.json());
        if (news_data) {
            console.log(news_data);
            news_status = true;
            renderNews();
        }
    }   
});

// render the news
function renderNews() {
    const container = document.querySelector(".news-container");
    container.innerHTML = "";
    Object.entries(news_data).forEach(([key, news]) => {
        // only add unreleased mods
        if (!news.released && news.genre !== 'ROM Hack') {
            // entry
            const newsEntry = document.createElement('div');
            newsEntry.classList.add('news-entry');
            // thumbnail
            const thumbnailDiv = document.createElement('div');
            thumbnailDiv.classList.add('news-thumbnail');
            let previewImages = (news.media) ? news.media.previews || [] : [];
            // format the preview images
            previewImages = previewImages.map(preview => 'https://codeberg.org/peachy/visions/raw/branch/main/' + preview);
            let currentPreviewIndex = 0;
            const thumbnailImg = document.createElement('img');
            thumbnailImg.src = previewImages[0] || 'assets/img/default.png';
            thumbnailImg.alt = `${news.creator} preview`;
            thumbnailImg.classList.add('thumbnail-image');
            thumbnailDiv.appendChild(thumbnailImg);
            // only show arrows if multiple images
            if (previewImages.length > 1) {
                // left navigation arrow
                const leftArrow = document.createElement('div');
                leftArrow.classList.add('arrow', 'left-arrow');
                leftArrow.textContent = '<';
                leftArrow.addEventListener('click', () => {
                    currentPreviewIndex = (currentPreviewIndex - 1 + previewImages.length) % previewImages.length;
                    console.log(currentPreviewIndex);
                    thumbnailImg.src = previewImages[currentPreviewIndex];
                });
                thumbnailDiv.appendChild(leftArrow);
                // right navigation error
                const rightArrow = document.createElement('div');
                rightArrow.classList.add('arrow', 'right-arrow');
                rightArrow.textContent = '>';
                rightArrow.addEventListener('click', () => {
                    currentPreviewIndex = (currentPreviewIndex + 1) % previewImages.length;
                    thumbnailImg.src = previewImages[currentPreviewIndex];
                });
                thumbnailDiv.appendChild(rightArrow);
            }
            // details abt the mod
            const detailsDiv = document.createElement('div');
            detailsDiv.classList.add('news-details');
            const title = document.createElement('h3');
            title.classList.add('news-title');
            title.textContent = key;
            detailsDiv.appendChild(title);
            const description = document.createElement('p');
            description.classList.add('news-description');
            description.textContent = news.desc || 'No description provided';
            detailsDiv.appendChild(description);
            const creatorInfo = document.createElement('p');
            creatorInfo.classList.add('news-creator');
            // creators is an array, format it pretty
            let creators = news.creator;
            if (creators) {
                if (creators.length > 1) {
                    creators = creators.join(', ');
                } else {
                    creators = creators[0];
                }
            } else {
                creators = 'Unknown';
            }
            creatorInfo.textContent = `Created by ${creators || 'Unknown'}`;
            detailsDiv.appendChild(creatorInfo);
            const linksDiv = document.createElement('div');
            linksDiv.classList.add('news-links');
            if (news.src) {
                const sourceLink = document.createElement('a');
                sourceLink.href = news.src;
                sourceLink.target = '_blank';
                sourceLink.classList.add('news-link');
                sourceLink.textContent = 'Source Code 🔗';
                linksDiv.appendChild(sourceLink);
            }
            // putting it all together
            newsEntry.appendChild(thumbnailDiv);
            newsEntry.appendChild(detailsDiv);
            newsEntry.appendChild(linksDiv);
            container.appendChild(newsEntry);
        }
    });
}

// listen for click to open the visions repo
document.getElementById('news-repo-link').addEventListener('click', () => {
    invoke('open_browser', { url: 'https://peachy.codeberg.page/visions/'});
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
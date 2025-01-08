// keep track of the repository data
const repo = "https://llamawa.re/repo.json";
const foreign = "https://raw.githubusercontent.com/kleineluka/burial/refs/heads/main/api/foreign.json";
let repo_data = null;
let repo_status = false;
let foreign_data = null;
let foreign_status = false;
let combined_data = null;
let combined_status = false;

// toggle the sidebar
document.getElementById('toggleSidebar').addEventListener('click', function () {
    var sidebar = document.querySelector('.sidebar');
    var mainContent = document.querySelector('.main-content');
    var toggleSidebar = document.getElementById('toggleSidebar');
    var homeButton = document.getElementById('sidebarHome');
    document.body.classList.toggle('sidebar-visible');
    sidebar.classList.toggle('hidden');
    mainContent.classList.toggle('full-width');
    toggleSidebar.classList.toggle('toggled');
    homeButton.classList.toggle('toggled');
});

// filter tag names
function filter_tags(tag) {
    switch (tag) {
        case "gen-ai":
            return "Generative AI";
        case "content":
            return "New Content";
        case "mature":
            return "Mature Themes";
        case "qol":
            return "Quality of Life";
        case "foreign":
            return "Third-Party";
        default:
            return tag;
    }
}

// filter source names
function filter_source(source) {
    switch (source) {
        case "llamware":
            return "Llamwa.re 💗";
        case "gamebanana":
            return "GameBanana 🍌";
        case "github":
            return "GitHub 🐙";
        default:
            return source;
    }
}

// download json into a structured object
async function download_repo() {
    // gather the data
    const response = await fetch(repo);
    if (!response.ok) {
        repo_status = false;
        console.error("Failed to fetch repository data");
        return;
    }
    const data = await response.json();
    if (!data) {
        repo_status = false;
        console.error("Failed to parse repository data");
        return;
    }
    const updatedData = data.map(mod => {
        mod.burial = {
            is_tomb: true,
            source_url: "llamware"
        };
        return mod;
    });
    repo_data = updatedData;
    repo_status = true;
}

// download foreign json into a structured object
async function download_foreign() {
    // gather the data
    const response = await fetch(foreign);
    if (!response.ok) {
        console.error("Failed to fetch foreign data");
        return;
    }
    const data = await response.json();
    if (!data) {
        console.error("Failed to parse foreign data");
        return;
    }
    foreign_data = data;
    foreign_status = true;
}

// combine the two jsons into one
function combine_jsons() {
    combined_data = repo_data.concat(foreign_data);
    combined_status = true;
}
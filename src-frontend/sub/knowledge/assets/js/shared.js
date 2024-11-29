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

// open the burial wiki
function openBurialWiiki() {
    invoke('open_browser', { url: 'https://github.com/kleineluka/burial/wiki' });
}

// open the coffin modding wiki
function openCoffinWiki() {
    invoke('open_browser', { url: 'https://coffin-wiki.basil.cafe/' });
}

// open the publish mod page
function openPublishMod() {
    invoke('open_browser', { url: 'https://github.com/Llamaware/Llamaware.github.io/' });
}
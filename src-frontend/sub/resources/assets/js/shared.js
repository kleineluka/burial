// toggle the sidebar
document.getElementById('toggleSidebar').addEventListener('click', function () {
    // these will be universal
    var sidebar = document.querySelector('.sidebar');
    var mainContent = document.querySelector('.main-content');
    var toggleSidebar = document.getElementById('toggleSidebar');
    var homeButton = document.getElementById('sidebarHome');
    document.body.classList.toggle('sidebar-visible');
    sidebar.classList.toggle('hidden');
    mainContent.classList.toggle('full-width');
    toggleSidebar.classList.toggle('toggled');
    homeButton.classList.toggle('toggled');
    // only some will have progress-container, so check if it exists
    var progressContainer = document.querySelector('.progress-container');
    if (progressContainer) {
        progressContainer.classList.toggle('toggled');
    }
});

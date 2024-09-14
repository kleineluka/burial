// toggle the sidebar  (if it exists, since not all will have it here)
var toggleSidebar = document.getElementById('toggleSidebar');
if (toggleSidebar) {
    toggleSidebar.addEventListener('click', function () {
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
}
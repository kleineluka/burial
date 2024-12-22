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
function openBurialWiki() {
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

// shared tooltips across all knowledge pages
function knowledgeTooltips() {
    tippy('#burial-links-button', {
        content: 'The official websites associated with Burial.',
        animation: 'perspective-subtle',
        placement: 'right',
        theme: 'burial'
    });
    tippy('#burial-wiki-button', {
        content: 'Learn more about Burial, how to use it, or how to contribute to it.',
        animation: 'perspective-subtle',
        placement: 'right',
        theme: 'burial'
    });
    tippy('#modding-wiki-button', {
        content: 'Learn how TCOAAL works internally.',
        animation: 'perspective-subtle',
        placement: 'right',
        theme: 'burial'
    });
    tippy('#llamaware-button', {
        content: 'Browse and download mods in your browser, as well as see other Llamawa.re projects.',
        animation: 'perspective-subtle',
        placement: 'right',
        theme: 'burial'
    });
    tippy('#playing-mods-button', {
        content: 'Learn how to manually install and play mods (outside of Burial).',
        animation: 'perspective-subtle',
        placement: 'right',
        theme: 'burial'
    });
    tippy('#making-mods-button', {
        content: 'Learn how to get started making TCOAAL mods.',
        animation: 'perspective-subtle',
        placement: 'right',
        theme: 'burial'
    });
    tippy('#publish-mod-button', {
        content: 'Submit your mod to the Llamawa.re website for others to play!',
        animation: 'perspective-subtle',
        placement: 'right',
        theme: 'burial'
    });
}
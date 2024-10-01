// listen for edit-package button click
document.getElementById('edit-package').addEventListener('click', function () {
    invoke('edit_package', { inPath: 'test' });
});
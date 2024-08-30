// determine os type on load and set footer hint
document.addEventListener('DOMContentLoaded', function() {
    // assume windows, i guess
    var os = 'windows'; 
    var platform = window.navigator.userAgent.toLowerCase();
    // determine os
    if (platform.indexOf('mac') > -1) {
        os = 'mac';
    } else if (platform.indexOf('linux') > -1 || platform.indexOf('x11') > -1) {
        os = 'linux';
    }
    // set footer inner html based on that
    var template = document.getElementById('footer').innerHTML; // replace %x% with os
    if (os === 'windows') {
        document.getElementById('footer').innerHTML = template.replace(/%x%/g, '%appdata%');
    } else {
        document.getElementById('footer').innerHTML = template.replace(/%x%/g, '.config');
    }
});
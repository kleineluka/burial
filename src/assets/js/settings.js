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
    // load the settings
    invoke ('load_settings', {} );
});

// listen for settings update
listen('settings-loaded', function(event) {
    // get json + parse values
    var settings = JSON.parse(event.payload);
    var tcoaal = settings.tcoaal;
    console.log(settings);
    var output = settings.output;
    // set values
    if (tcoaal) document.getElementById('input-path-tcoaal').value = tcoaal;
    if (output) document.getElementById('input-path-output').value = output;
});

// write new settings
document.getElementById('save-button').addEventListener('click', (event) => {
    // get values
    var tcoaal = document.getElementById('input-path-tcoaal').value;
    var output = document.getElementById('input-path-output').value;
    // set values
    invoke('save_settings', { tcoaal, output });
});
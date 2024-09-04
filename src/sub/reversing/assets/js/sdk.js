// on load, fetch sdk json
let sdkData = {};
document.addEventListener('DOMContentLoaded', () => {
    fetch('https://raw.githubusercontent.com/kleineluka/burial/main/api/sdk.json')
        .then(response => response.json())
        .then(data => {
            sdkData = data;
            const dropdownMenu = document.getElementById('dropdown-menu-type');
            Object.keys(sdkData).forEach(key => {
                const option = document.createElement('option');
                option.value = key;
                option.text = key;
                dropdownMenu.appendChild(option);
            });
        })
        .catch(error => console.error('Error fetching SDK JSON:', error));
});

// get os
function getOS() {
    var os = 'Windows';
    var platform = window.navigator.userAgent.toLowerCase();
    // determine os
    if (platform.indexOf('mac') > -1) {
        os = 'MacOS';
    } else if (platform.indexOf('linux') > -1 || platform.indexOf('x11') > -1) {
        os = 'Linux';
    }
    return os;
}

// install current sdk on click
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('sdk-install').addEventListener('click', () => {
        // get selected sdk
        const sdkType = document.getElementById('dropdown-menu-type').value;
        // get current version
        const version = sdkData[sdkType].Version;
        const user_os = getOS();
        const os = sdkData[sdkType]?.[user_os];
        const inUrl = `https://dl.nwjs.io/v${version}/${os}`;
        // get game input
        const inPath = document.getElementById('tcoaal-path').value;
        // send to rust
        invoke('install_sdk', { inUrl, inPath });
    });
});
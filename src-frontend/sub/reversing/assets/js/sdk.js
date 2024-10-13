// on load, fetch sdk json
let sdkData = {};
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/sdk.json')
        .then(response => response.json())
        .then(data => {
            sdkData = data;
            const versionDropdown = document.getElementById('dropdown-menu-version');
            const branchesDropdown = document.getElementById('dropdown-menu-branch');
            // populate version drop down
            Object.keys(sdkData).forEach(key => {
                const option = document.createElement('option');
                option.value = key;
                option.text = key;
                versionDropdown.appendChild(option);
            });
            // function (to easily on load and on change) to set branches based on selected version
            const setBranches = () => {
                const selectedVersion = versionDropdown.value;
                const branches = sdkData[selectedVersion]?.branches || {};
                branchesDropdown.innerHTML = '';
                Object.keys(branches).forEach(key => {
                    const option = document.createElement('option');
                    option.value = key;
                    option.text = key;
                    branchesDropdown.appendChild(option);
                });
            };
            // set branches on initial load and then on changes
            setBranches();
            versionDropdown.addEventListener('change', setBranches);
        })
        .catch(error => console.error('Error fetching SDK JSON:', error));
});

// get the currently installed sdk on load
document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    let inPath = await store.get('settings-tcoaal');
    invoke("sdk_presence_wrapper", { inPath })
});

listen('sdk-presence', (event) => {
    const sdkDropdown = document.getElementById('dropdown-menu-installed-sdk');
    const sdkOption = document.createElement('option');
    sdkOption.value = event.payload;
    sdkOption.text = event.payload;
    sdkDropdown.appendChild(sdkOption);
});

// install current sdk on click
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('sdk-install').addEventListener('click', () => {
        // get selected sdk
        const sdkType = document.getElementById('dropdown-menu-version').value;
        // get current version
        const version = sdkData[sdkType].Game;
        const sdkBranch = document.getElementById('dropdown-menu-branch').value;
        let os = (sdkBranch === 'Default') ? sdkData[sdkType]?.branches.Windows : sdkData[sdkType]?.branches[sdkBranch];
        const inUrl = `https://dl.nwjs.io/v${version}/${os}`;
        console.log(inUrl);
        // get game input
        const inPath = document.getElementById('tcoaal-path').value;
        // send to rust
        invoke('install_sdk', { inUrl, inPath });
    });
});
function test() {
    invoke("install_tomb", { inPath: "D:\\Games\\Steam\\steamapps\\common\\The Coffin of Andy and Leyley" } );
}

// Listen first
listen('modloader-version', (event) => {
    const dropdown = document.getElementById('dropdown-menu-current');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = event.payload;
    option.text = event.payload;
    dropdown.appendChild(option);
});

listen('modloader-versions', (event) => {
    const dropdown = document.getElementById('dropdown-menu-install'); 
    dropdown.innerHTML = '';
    event.payload.forEach((version, index) => {
        const option = document.createElement('option');
        option.value = version;
        option.textContent = version;
        if (index === 0) {
            option.selected = true;
        }
        dropdown.appendChild(option);
    });
});

document.addEventListener('DOMContentLoaded', async () => {
    const store = loadStorage();
    const inPath = await store.get('settings-tcoaal');
    invoke("modloader_version", { inPath });
    invoke("modloader_versions", {});
});

// install selected version
function installSelected() {
    const inPath = document.getElementById('tcoaal-path').value;
    invoke("install_modloader", { inPath } );
}

// refresh
function refreshLocal() {
    const dropdown = document.getElementById('dropdown-menu-current');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = 'Loading...';
    option.text = 'Loading...';
    dropdown.appendChild(option);
    const inPath = document.getElementById('tcoaal-path').value;
    invoke("modloader_version", { inPath });
}

function refreshRemote() {
    const dropdown = document.getElementById('dropdown-menu-install');
    dropdown.innerHTML = '';
    const option = document.createElement('option');
    option.value = 'Loading...';
    option.text = 'Loading...';
    dropdown.appendChild(option);
    invoke("modloader_versions", { });
}
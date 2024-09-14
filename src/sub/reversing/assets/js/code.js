// populate + update dropdowns
let methodsData = {};
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/extraction_list/list.json')
        .then(response => response.json())
        .then(data => {
            // clear + populate
            const dropdown = document.getElementById('dropdown-menu-method');
            dropdown.innerHTML = '';
            Object.keys(data).forEach(key => {
                const option = document.createElement('option');
                option.value = data[key].path; 
                option.text = key;
                // if suggested is true, make this option the default selected
                if (data[key].suggested) {
                    option.selected = true;
                }
                dropdown.appendChild(option);
            });
            // store data for later
            methodsData = data;
        })
        .catch(error => console.error('Error fetching SDK JSON:', error));
});

// do extraction
document.getElementById('extract-code').addEventListener('click', function () {
    // make sure that out path is set
    const outPath = document.getElementById('out-path-extract').value;
    if (outPath === '') {
        Swal.fire({
            icon: "error",
            title: "Please set an output path!",
            showConfirmButton: true
        });
        return;
    }
    // get selected method + values
    const dropdown = document.getElementById('dropdown-menu-method');
    const selectedPath = dropdown.value;
    const selectedMethod = Object.values(methodsData).find(method => method.path === selectedPath);
    if (selectedMethod) {
        // extract relevant data
        const findPath = selectedMethod.find;
        const replacePath = selectedMethod.replace;
        const targetPath = selectedMethod.path;
        // get the find path
        fetch(findPath)
            .then(response => response.text())
            .then(findPathData => {
                // get the replace path
                fetch(replacePath)
                    .then(response => response.text())
                    .then(replacePathData => {
                        // get the tcoaal path and output path
                        const tcoaalPath = document.getElementById('tcoaal-path').value;
                        let outPath = document.getElementById('out-path-extract').value;
                        // sanitize outPath + in replacePathData, replace %path% with outPath
                        outPath = outPath.replaceAll('\\', '\\\\');
                        replacePathData = replacePathData.replace(/%path%/g, outPath);
                        // get true/false settings
                        const autoValue = document.getElementById('dropdown-menu-auto').value === 'true';
                        const cleanupValue = document.getElementById('dropdown-menu-cleanup').value === 'true';
                        const deobfuscateValue = document.getElementById('dropdown-menu-deobfuscate').value === 'true';
                        // send to rust
                        invoke('extract_code', { inPath: tcoaalPath, 
                            inFile: targetPath, oldText: findPathData, newText: replacePathData, 
                            autoRun: autoValue, autoRestore: cleanupValue, autoDeobfuscate: deobfuscateValue })
                    })
        })
    } 
});

// switch between horizontal navbars
document.addEventListener('DOMContentLoaded', () => {
    const navOptions = document.querySelectorAll('.page-navbar-option');
    const subContainers = document.querySelectorAll('.page-container');
    navOptions.forEach(option => {
        option.addEventListener('click', (event) => {
            event.preventDefault();
            // clear current selection
            navOptions.forEach(nav => nav.classList.remove('selected'));
            subContainers.forEach(container => container.classList.add('hidden'));
            // show what was selected
            option.classList.add('selected');
            const id = option.id;
            const subContainer = document.getElementById(`sub-${id}`);
            if (subContainer) {
                subContainer.classList.remove('hidden');
            }
        });
    });
});
// populate + update dropdowns
let methodsData = {};
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/extraction.json')
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

// check deno status on page load as well
document.addEventListener('DOMContentLoaded', async () => {
    // get store to load os
    const store = loadStorage();
    const operatingSystem = await store.get('state-operating-system');
    invoke('check_deno', { operatingSystem });
});

// listen for deno status
let deno_status = false;
listen('deno_presence', (event) => {
    if (event.payload) deno_status = true;
});

// do extraction
document.getElementById('extract-code').addEventListener('click', function () {
    // make sure that out path is set
    const outPath = document.getElementById('output-path').value;
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
                        let outPath = document.getElementById('output-path').value;
                        // sanitize outPath + in replacePathData, replace %path% with outPath
                        outPath = outPath.replaceAll('\\', '\\\\');
                        replacePathData = replacePathData.replace(/%path%/g, outPath);
                        // get true/false settings
                        const autoValue = document.getElementById('dropdown-menu-auto').value === 'true';
                        const cleanupValue = document.getElementById('dropdown-menu-cleanup').value === 'true';
                        const deobfuscateValue = document.getElementById('dropdown-menu-deobfuscate').value === 'true';
                        // get the extraction method and the deobfuscation method
                        const extractionMethod = selectedMethod.extraction;
                        const deobfuscationMethod = selectedMethod.deobfuscate;
                        const requiresDeno = selectedMethod.deno;
                        // check if the user needs to install deno firsst
                        const deno_needed = document.getElementById('dropdown-menu-deobfuscate').value === 'true' && !deno_status && requiresDeno;
                        if (deno_needed) {
                            Swal.fire({
                                title: "Deno is required for deobfuscation.",
                                text: "Would you like to automatically install Deno now? It will take up around 100mb of space, but can be removed within Burial at any time. You can read more about why Deno is needed on the wiki!",
                                showCancelButton: true,
                                confirmButtonText: "Yes",
                                cancelButtonText: "No",
                                confirmButtonColor: '#F595B2'
                            }).then((result) => {
                                if (result.isConfirmed) {
                                    // continue on, deno will auto install
                                } else {
                                    Swal.fire({
                                        title: "That's okay.",
                                        text: "You can try running the extraction without deobfuscation and using an online tool to deobfuscate the code instead. Trying to extract with auto-deobfuscation on will prompt this again if you change your mind!",
                                        showConfirmButton: true,
                                        confirmButtonColor: '#F595B2'
                                    });
                                    return;
                                }
                            });
                        }
                        // and now.. for some required deno data (yay, this is getting messy!)
                        fetch('/data/supported/deno.json') 
                            .then(response => response.json())
                            .then(async deno_data => {
                                // we also need the os
                                const store = loadStorage();
                                let operatingSystem = await store.get('state-operating-system');
                                // send to rust
                                invoke('extract_code', { inPath: tcoaalPath, 
                                    inFile: targetPath, oldText: findPathData, newText: replacePathData, 
                                    autoRun: autoValue, autoRestore: cleanupValue, autoDeobfuscate: deobfuscateValue,
                                    extractionMethod: extractionMethod, deobfuscateMethod: deobfuscationMethod,
                                    requiresDeno: deno_needed, operatingSystem: operatingSystem, denoInfo: deno_data,
                                    outPath: outPath });
                            })
                            .catch(error => console.error('Error fetching SDK JSON:', error));
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

// test button lol
function testme() {
    let deno_data;
    fetch('/data/supported/deno.json') 
        .then(response => response.json())
        .then(data => {
            deno_data = data;
            console.log(deno_data);
            invoke('testme', { denoInfo: deno_data });
        })
        .catch(error => console.error('Error fetching SDK JSON:', error));
}
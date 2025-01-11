// populate + update dropdowns
let methodsData = {};
let obfuscationData = {};
let beautifyData = {};
document.addEventListener('DOMContentLoaded', () => {
    // fetch the extraction data
    fetch('/data/supported/extraction.json')
        .then(response => response.json())
        .then(data => {
            // clear + populate
            const dropdown = document.getElementById('dropdown-menu-method-extraction');
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
    // fetch the obfuscation data
    fetch('/data/supported/deobfuscation.json') 
        .then(response => response.json())
        .then(data => {
            // clear + populate
            const dropdown = document.getElementById('dropdown-menu-method-deobfuscation');
            dropdown.innerHTML = '';
            Object.keys(data).forEach(key => {
                const option = document.createElement('option');
                option.value = data[key].name;
                option.text = key;
                dropdown.appendChild(option);
            });
            // store data for later
            obfuscationData = data;
        })
        .catch(error => console.error('Error fetching SDK JSON:', error));
    // fetch the beautify data
    fetch('/data/supported/beautify.json') 
        .then(response => response.json())
        .then(data => {
            // clear + populate
            const dropdown = document.getElementById('dropdown-menu-method-beautify');
            dropdown.innerHTML = '';
            Object.keys(data).forEach(key => {
                const option = document.createElement('option');
                option.value = data[key].name;
                option.text = key;
                dropdown.appendChild(option);
            });
            // store data for later
            beautifyData = data;
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
    const dropdown = document.getElementById('dropdown-menu-method-extraction');
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

// do deobfuscation
document.getElementById('deobfuscate-code').addEventListener('click', function () {
    // make sure that code path is set
    const codePath = document.getElementById('code-path-deobfuscate').value;
    if (codePath === '') {
        Swal.fire({
            icon: "error",
            title: "Please set the path to your extracted code!",
            showConfirmButton: true
        });
        return;
    }
    // get selected method
    const dropdown = document.getElementById('dropdown-menu-method-deobfuscation');
    const selectedMethod = dropdown.value;
    // send to backend
    invoke("deobfuscate_code", { inPath: codePath, deobfuscateMethod: selectedMethod });
});

// do beautify
document.getElementById('beautify-code').addEventListener('click', function () {
    // make sure that code path is set
    const codePath = document.getElementById('code-path-beautify').value;
    if (codePath === '') {
        Swal.fire({
            icon: "error",
            title: "Please set the path to your deobfuscated code!",
            showConfirmButton: true
        });
        return;
    }
    // get selected method
    const dropdown = document.getElementById('dropdown-menu-method-beautify');
    const selectedMethod = dropdown.value;
    // send to backend
    invoke("beautify_code", { inPath: codePath, beautifyMethod: selectedMethod });
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

// click on deobfuscate browse
document.getElementById('browse-button-code-deobfuscate').addEventListener('click', (event) => {
    invoke('file_dialog', { emitEvent: 'selected-code-deobfuscate-file', fileType: 'all' });
});

listen('selected-code-deobfuscate-file', (event) => {
    document.getElementById('code-path-deobfuscate').value = event.payload;
});

// click on beautify browse
document.getElementById('browse-button-code-beautify').addEventListener('click', (event) => {
    invoke('file_dialog', { emitEvent: 'selected-code-beautify-file', fileType: 'all' });
});

listen('selected-code-beautify-file', (event) => {
    document.getElementById('code-path-beautify').value = event.payload;
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
    tippy('#extraction-method-label', {
        content: 'Determines how Burial will dump the game\'s code. For now, this doesn\'t mean much.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#autorun-label', {
        content: 'Automatically kill the game process after starting it (required to dump).',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#restore-script-label', {
        content: 'Burial changes the game\'s code to dump it, this option will restore it to normal after dumping.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#deobfuscate-label', {
        content: 'Automatically deobfuscate (make human readable) the dumped code.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#deobfuscation-method-label', {
        content: 'Determines how Burial will deobfuscate the code. For now, this doesn\'t mean much.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#code-path-deobfuscate', {
        content: 'The path to the dumped game code - this will be overwritten.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#beautify-method-label', {
        content: 'Determines how Burial will beautify the code. For now, this doesn\'t mean much.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
    tippy('#code-path-beautify', {
        content: 'The path to the deobfuscated game code - this will be overwritten.',
        animation: 'perspective-subtle',
        placement: 'top',
        theme: 'burial'
    });
});
// populate and update dropdown menus via local json
let siftData = {};
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/supported/sifting.json')
        .then(response => response.json())
        .then(listData => {
            siftData = listData;
            // get the dropdown elements
            const typeDropdown = document.getElementById('dropdown-menu-type');
            const categoryDropdown = document.getElementById('dropdown-menu-category');
            // populate type dropdown
            const topLevelCategories = Object.keys(listData.categories);
            topLevelCategories.forEach(category => {
                const option = document.createElement('option');
                option.value = category;
                option.textContent = category;
                typeDropdown.appendChild(option);
            });
            // set default selection for type dropdown
            if (topLevelCategories.length > 0) {
                typeDropdown.value = topLevelCategories[0];
                populateCategoryDropdown(topLevelCategories[0]);
            }
            // event listener for type dropdown change
            typeDropdown.addEventListener('change', (event) => {
                const selectedType = event.target.value;
                populateCategoryDropdown(selectedType);
            });
            // populate category dropdown based on selected type
            function populateCategoryDropdown(type) {
                const categories = listData.categories[type] ? Object.keys(listData.categories[type]) : [];
                categoryDropdown.innerHTML = ''; // Clear previous options
                categories.forEach(category => {
                    const option = document.createElement('option');
                    option.value = category;
                    option.textContent = category;
                    categoryDropdown.appendChild(option);
                });
                // set the default category
                if (categories.length > 0) {
                    categoryDropdown.value = categories[0];
                }
            }
        })
        .catch(error => {
            console.error('Error fetching the JSON data:', error);
        });
});

// send selected sift data to rust
document.getElementById('sift-button').addEventListener('click', () => {
    // get the in and out paths (input is tcoaal-path and output is output-path)
    const inPath = document.getElementById('tcoaal-path').value;
    const outPath = document.getElementById('output-path').value
    // get selected caategory
    const type = document.getElementById('dropdown-menu-type').value;
    const category = document.getElementById('dropdown-menu-category').value;
    const result = siftData.categories[type][category].path;  
    // fetch data at that path
    fetch('/data/rules/sifting/' + result) 
        .then(response => response.json())
        .then(ruleData => {
            const rulePaths = ruleData.paths;
            const ruleFiles = ruleData.files;
            const rulePrefixes = ruleData.prefixes;
            const ruleExtensions = ruleData.extensions;
            // send data to rust
            invoke ('export_resources', { inPath, outPath, rulePaths, ruleFiles, rulePrefixes, ruleExtensions });
        }) 
});

// tooltips
document.addEventListener('DOMContentLoaded', async () => {
    if (await skipTooltips()) return;
    defaultTooltips();
});
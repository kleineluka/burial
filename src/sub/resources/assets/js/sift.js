// populate and update dropdown menus via local json
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/sifting_list/list.json')
        .then(response => response.json())
        .then(data => {
            // get the dropdown elements
            const typeDropdown = document.getElementById('dropdown-menu-type');
            const categoryDropdown = document.getElementById('dropdown-menu-category');
            // populate type dropdown
            const topLevelCategories = Object.keys(data.categories);
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
                const categories = data.categories[type] ? Object.keys(data.categories[type]) : [];
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

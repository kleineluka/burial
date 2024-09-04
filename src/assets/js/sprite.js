// automagically populate dropdown menus + watch for changes
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/sprite_list/supported.json')
        .then(response => response.json())
        .then(data => {
            // get the dropdown elements
            const spriteDropdown = document.getElementById('dropdown-menu-sprite');
            const characterDropdown = document.getElementById('dropdown-menu-character');
            const templateDropdown = document.getElementById('dropdown-menu-template');
            // populate sprite dropdown
            const topLevelCategories = Object.keys(data);
            topLevelCategories.forEach(category => {
                const option = document.createElement('option');
                option.value = category;
                option.textContent = category;
                spriteDropdown.appendChild(option);
            });
            // set default selection for sprite dropdown
            if (topLevelCategories.length > 0) {
                spriteDropdown.value = topLevelCategories[0];
                populateCharacterDropdown(topLevelCategories[0]);
                populateTemplateDropdown(topLevelCategories[0], Object.keys(data[topLevelCategories[0]])[0]);
            }
            // event listener for sprite dropdown change
            spriteDropdown.addEventListener('change', (event) => {
                const selectedCategory = event.target.value;
                populateCharacterDropdown(selectedCategory);
                // reset template dropdown
                templateDropdown.innerHTML = '';
                populateTemplateDropdown(selectedCategory, Object.keys(data[selectedCategory])[0]);
            });
            // populate character dropdown based on selected sprite category
            function populateCharacterDropdown(category) {
                const subCategories = data[category] ? Object.keys(data[category]) : [];
                characterDropdown.innerHTML = ''; // Clear previous options
                subCategories.forEach(subCategory => {
                    const option = document.createElement('option');
                    option.value = subCategory;
                    option.textContent = subCategory;
                    characterDropdown.appendChild(option);
                });
                // set default selection for character dropdown
                if (subCategories.length > 0) {
                    characterDropdown.value = subCategories[0];
                    populateTemplateDropdown(category, subCategories[0]);
                }
            }
            // event listener for character dropdown change
            characterDropdown.addEventListener('change', (event) => {
                const selectedCategory = spriteDropdown.value;
                const selectedCharacter = event.target.value;
                // Reset template dropdown
                templateDropdown.innerHTML = '';
                populateTemplateDropdown(selectedCategory, selectedCharacter);
            });
            // populate template dropdown based on selected sprite category and character
            function populateTemplateDropdown(category, character) {
                const options = data[category] && data[category][character] ? data[category][character] : [];
                templateDropdown.innerHTML = ''; // Clear previous options
                options.forEach(optionData => {
                    const option = document.createElement('option');
                    option.value = optionData.sprite_name;
                    option.textContent = optionData.sprite_name;
                    templateDropdown.appendChild(option);
                });
                // set default selection for template dropdown
                if (options.length > 0) {
                    templateDropdown.value = options[0].sprite_name;
                }
            }
        })
        .catch(error => {
            console.error('Error fetching the JSON data:', error);
        });
});

// do the painting
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('paint-button').addEventListener('click', () => {
        fetch('/data/sprite_list/supported.json')
            .then(response => response.json())
            .then(data => {
                // get values from dopdown menus
                const spriteDropdown = document.getElementById('dropdown-menu-sprite');
                const characterDropdown = document.getElementById('dropdown-menu-character');
                const templateDropdown = document.getElementById('dropdown-menu-template');
                const sprite = spriteDropdown.value;
                const character = characterDropdown.value;
                const template = templateDropdown.value;
                // find in json
                const spriteData = data[sprite];
                const characterData = spriteData ? spriteData[character] : null;
                const selectedData = characterData ? characterData.find(item => item.sprite_name === template) : null;
                if (selectedData) {
                    console.log('Found Data:', selectedData);
                } else {
                    console.log('No matching data found.');
                }
                // gather all data to send
                const bytePath = selectedData.sprite_bytes;
                // read from /data/sprite_byte/bytePath (its just raw data)
                fetch(`/data/sprite_byte/${bytePath}`)
                    .then(response => response.text())
                    .then(byteList => {
                        //pub fn make_sprite(window: Window, game_path: String, sprite_path: String, out_path: String, sprite_name: String, byte_list: String) {
                        const gamePath = document.getElementById('tcoaal-path').value;
                        const spritePath = selectedData.sprite_location;
                        const outPath = document.getElementById('output-path').value;
                        const spriteName = selectedData.sprite_name;
                        // send to rust backend
                        invoke('make_sprite', { gamePath, spritePath, outPath, spriteName, byteList });
                    })
                    .catch(error => {
                        console.error('Error fetching the byte data:', error);
                    });
            })
            .catch(error => {
                console.error('Error fetching the JSON data:', error);
            });
    });
});
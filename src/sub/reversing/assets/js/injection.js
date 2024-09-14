// populate + update dropdowns
document.addEventListener('DOMContentLoaded', () => {
    fetch('/data/injection_list/list.json')
        .then(response => response.json())
        .then(data => {
            const dropdownFile = document.getElementById('dropdown-menu-file');
            const dropdownLocation = document.getElementById('dropdown-menu-location');
            // populate the 'Inject To File' dropdown
            for (const fileName in data) {
                const option = document.createElement('option');
                option.value = fileName;
                option.textContent = fileName;
                dropdownFile.appendChild(option);
            }
            // update the 'Inject In File' dropdown when a file is selected
            dropdownFile.addEventListener('change', () => {
                dropdownLocation.innerHTML = '';
                const selectedFile = dropdownFile.value;
                const locations = data[selectedFile];
                for (const location in locations) {
                    const option = document.createElement('option');
                    option.value = location;
                    option.textContent = location;
                    dropdownLocation.appendChild(option);
                }
            });
            // trigger the change event on page load to populate the second dropdown
            dropdownFile.dispatchEvent(new Event('change'));
        })
        .catch(error => console.error('Error fetching SDK JSON:', error));
});

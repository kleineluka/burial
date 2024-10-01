// versioning numbers
function inputVersion(input) {
    // numbers and periods
    input.value = input.value.replace(/[^0-9.]/g, '');
    // start with number + only one period at a time
    input.value = input.value.replace(/(\.\.)/g, '.'); 
}

// add/remove authors
document.getElementById("add-author").addEventListener("click", function () {
    // Container where we will append new rows
    const container = document.querySelector(".multi-container");

    // Create a new second-row for Authors
    let newRow = document.createElement("div");
    newRow.classList.add("second-row");

    // Add the HTML structure for the new Author row
    newRow.innerHTML = `
            <p><!-- Empty! --></p>
            <input type="text" class="file-path-input input-author" value="">
            <button class="browse-button hvr-shrink remove-author">-</button>
        `;

    // Append the new row to the container
    container.appendChild(newRow);

    // Add event listener for the remove button in the new row
    newRow.querySelector(".remove-author").addEventListener("click", function () {
        newRow.remove();
    });
});

// add/remove mod dependencies
document.getElementById("add-mod-dep").addEventListener("click", function () {
    // prep: get container, create row
    const container = document.getElementById("mod-dep-container");
    let newRow = document.createElement("div");
    newRow.classList.add("double-row");
    // add the row + listener to remove the row
    newRow.innerHTML = `
            <p><!-- Empty --></p>
            <input class="file-path-input input-mod-id" placeholder="Mod ID" value="">
            <input class="file-path-input input-mod-version" placeholder="Mod Version" value="">
            <button class="browse-button hvr-shrink remove-mod-dep">-</button>
        `;
    container.appendChild(newRow);
    newRow.querySelector(".remove-mod-dep").addEventListener("click", function () {
        newRow.remove();
    });
});

// add/remove files
document.getElementById("add-file").addEventListener("click", function () {
    // prep: get option, get container, create row
    const selectedValue = document.getElementById("dropdown-menu-file").value;
    const container = document.getElementById("files-container");
    let newRow = document.createElement("div");
    // branch based on what we are making
    switch (selectedValue) {
        case "assets":
            newRow.innerHTML = `
                    <div class="second-row">
                        <p>Asset:</p>
                        <input class="file-path-input input-files-assets" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "imageDeltas":
            newRow.innerHTML = `
                    <div class="second-row">
                        <p>Image Delta:</p>
                        <input class="file-path-input input-files-imagedeltas" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "dataDeltas":
            newRow.innerHTML = `
                    <div class="second-row">
                        <p>Data Delta:</p>
                        <input class="file-path-input input-files-datadeltas" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "plugins":
            newRow.innerHTML = `
                    <div class="second-row">
                        <p>Plugin:</p>
                        <input class="file-path-input input-files-plugins" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "languages":
            newRow.innerHTML = `
                    <div class="second-row">
                        <p>Language:</p>
                        <input class="file-path-input input-files-languages" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        case "inject":
            newRow.innerHTML = `
                    <div class="double-row">
                        <p>Inject:</p>
                        <input class="file-path-input input-files-inject-file" placeholder="File to Inject" value="">
                        <input class="file-path-input input-files-inject-at" placeholder="Inject At" value="">
                        <button class="browse-button hvr-shrink remove-file">-</button>
                    </div>
                `;
            break;
        default:
            break;
    }
    // add the row + listener to remove the row
    container.appendChild(newRow);
    newRow.querySelector(".remove-file").addEventListener("click", function () {
        newRow.remove();
    });
});
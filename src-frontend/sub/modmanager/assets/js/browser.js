// constant (for now at least)
const repositoryInfo = {
    owner: "Llamaware",
    repo: "Llamaware.github.io",
    path: "src/content/mods",
    branch: "main"
}

// filter mod url
function filter_mod(url) {
    if (url.includes("github.com")) {
        url = url.replace("github.com", "raw.githubusercontent.com");
        if (url.includes("/blob/")) {
            return url.replace("/blob/", "/main/");
        } else {
            return url;
        }
    }
    if (url.includes("codeberg.org")) {
        if (url.includes("/src/")) {
            return url.replace("/src/", "/raw/");
        } else {
            // add /raw/branch/main/ before the mod.json
            const splitUrl = url.split("/");
            const repoName = splitUrl[splitUrl.length - 2];
            const modName = splitUrl[splitUrl.length - 1];
            return url.replace(modName, `raw/branch/main/${modName}`);
        }
    }
}

// load all mods from the repository
async function load_repository(owner, repo, path, branch) {
    const repoUrl = `https://api.github.com/repos/${owner}/${repo}/contents/${path}?ref=${branch}`;
    try {
        // fetch the contents of the folder + only get JSON files
        const response = await fetch(repoUrl, {
            headers: {
                "Accept": "application/vnd.github.v3+json",
            },
        });
        const files = await response.json();
        const jsonFiles = files.filter(file => file.name.endsWith('.json'));
        // get each json file's content
        const fileContents = [];
        // fetch each JSON file's content dynamically
        for (const file of jsonFiles) {
            const fileResponse = await fetch(file.download_url);
            if (!fileResponse.ok) {
                throw new Error(`Failed to fetch ${file.name}`);
            }
            const fileData = await fileResponse.json(); // Parse JSON content
            fileContents.push({ path: file.name, content: fileData });
        }
        return fileContents;
    } catch (error) {
        console.error("Error fetching files:", error);
    }
}

// load info about all mods
async function load_mods(jsonList) {
    const updatedJsonList = [];

    for (const modInfo of jsonList) {
        const sourceUrl = filter_mod(`${modInfo.content.source}/mod.json`);
        

        try {
            // Fetch mod.json from each source
            const response = await fetch(sourceUrl);

            if (!response.ok) {
                console.warn(`Failed to fetch mod.json from: ${sourceUrl}`);
                continue;
            }

            const modJsonData = await response.json(); // Assuming it's a valid JSON

            // Append the mod.json content to the existing object
            const updatedModInfo = {
                ...modInfo,
                modJsonContent: modJsonData, // Add the fetched mod.json data
            };

            updatedJsonList.push(updatedModInfo);

        } catch (error) {
            console.error(`Error fetching from ${sourceUrl}:`, error);
        }
    }

    return updatedJsonList;
}

// on page laod, fetch the repository
window.addEventListener('load', async () => {
    const files = await load_repository(repositoryInfo.owner, repositoryInfo.repo, repositoryInfo.path, repositoryInfo.branch);
    const jsonData = await load_mods(files);
    console.log(jsonData);
});
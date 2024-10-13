// imports
import * as fs from 'node:fs';
import path from 'node:path';

// what a rule should look like
interface NameRule {
    old: string; // the old name
    new: string; // the new name
}

// function to find, read and parse the json file
function loadNameRules(): NameRule[] {
    let dirPath = path.dirname(new URL(import.meta.url).pathname);
    dirPath = dirPath.substring(1);
    const namesFilePath = path.join(dirPath, '../data/names.json');
    const fileContents = fs.readFileSync(namesFilePath, 'utf-8');
    return JSON.parse(fileContents);
}

// function to decode base64 strings (using deno stuff)
function decodeBase64(input: string): string {
    return new TextDecoder().decode(Uint8Array.from(atob(input), c => c.charCodeAt(0)));
}

// rename the names with the naming rules to get the new names for renaming
export default function renameScrambled(code: string): string {
    // load rules
    const rules = loadNameRules();
    // loop through the rules
    rules.forEach(({ old: oldName, new: newName }) => {
        // convert all old names from base64
        oldName = decodeBase64(oldName);
        // replace all occurences of the old name with the new name
        code = code.replaceAll(oldName, newName);
    });
    return code;
}

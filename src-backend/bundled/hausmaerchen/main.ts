// imports, no need to do any node stuff, let deno take care of it all..
import { parseArgs } from 'https://deno.land/std@0.211.0/cli/parse_args.ts';
import readCode from './steps/readCode.ts';
import deobfuscate from './steps/deobfuscate.ts';
import prettyPrint from './steps/prettyPrint.ts';
import commentWriter from './steps/commentWriter.ts';
import renameScrambled from './steps/renameScrambled.ts';
import writeCode from './steps/writeCode.ts';

// parse command line arguments, ensure needed are there, and set optional to default values
const args = parseArgs(Deno.args);

// main function that calls all the steps in order
async function main() {
    // print help if needed
    console.log('Welcome to Hausmärchen! A tool to deobfuscate code using webcrack and make it a bit more readable.');
    if (!args['out-path']) throw new Error('arg missing: path-out');
    if (!args['code-path']) throw new Error('arg missing: path-code');
    args['add-comments'] = args['add-comments'] ?? true;
    args['rename-scrambled'] = args['rename-scrambled'] ?? true;
    // call steps, one by one 
    let code = readCode(args); // Step 1: read code from given in-path
    code = await deobfuscate(code); // Step 2: deobfuscate code using webcrack
    code = prettyPrint(code); // Step 3: make the code a bit nicer to read
    if (args['add-comments']) code = commentWriter(code); // Step 3.1 (optional): add comments to the code
    if (args['rename-scrambled']) code = renameScrambled(code); // Step 3.2 (optional): rename variables and functions
    writeCode(code, args['out-path']); // Step 4: write the code to the given out-path
    console.log('success');
}

main().catch(console.error);
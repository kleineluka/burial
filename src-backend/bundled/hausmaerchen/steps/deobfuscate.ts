// imports 
import { webcrack } from 'npm:webcrack';

export default async function deobfuscate(source: string) {
    return (await webcrack(source)).code;
}
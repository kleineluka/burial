// write the code to the given out-path
export default function writeCode(code: string, outPath: string) {
    Deno.writeTextFileSync(outPath, code);
}
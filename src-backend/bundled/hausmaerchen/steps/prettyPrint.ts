export default function prettyPrint(code: string): string {
    // match all `}`, `})`, `};`, or `});` to add a newline after them
    const closingBracesRegex = /(\}\s*;|\}\s*\)|})/g;
    code = code.replace(closingBracesRegex, '$&\n');
    // for some reason, any 'else' statement is missing one space. let's fix that?
    const elseRegex = /^(\s*)else/gm;
    code = code.replace(elseRegex, '$1 else');
    return code;
}
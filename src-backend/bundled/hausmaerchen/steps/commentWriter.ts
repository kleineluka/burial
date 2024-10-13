// imports
import * as fs from 'node:fs';
import path from 'node:path';

// what a rule should look like
interface CommentRule {
    pattern: string;            // the pattern to match (can be a string or regexp, but to prevent conflict, see below)
    comment: string;           // the comment to insert before the matched line
    useRegex: boolean;         // whether the pattern is a regex or not (optional, default: false)
    multiline?: boolean;       // whether to insert a multiline comment (optional, default: false)
    wordsPerLine?: number;     // the max # of words per line for multiline comments (optional, default: 10)
    padBefore?: boolean;       // add a new line before the comment if this section of code should stand out (optional, default: false)
    appendInline?: boolean;   // append the comment to the end of the current line (optional, default: false)
}

// function to find, read and parse the json file
function loadCommentRules(): CommentRule[] {
    let dirPath = path.dirname(new URL(import.meta.url).pathname);
    dirPath = dirPath.substring(1);
    const commentsFilePath = path.join(dirPath, '../data/comments.json');
    const fileContents = fs.readFileSync(commentsFilePath, 'utf-8');
    return JSON.parse(fileContents);
}

// function to decode base64 strings (using deno stuff)
function decodeBase64(input: string): string {
    return new TextDecoder().decode(Uint8Array.from(atob(input), c => c.charCodeAt(0)));
}

// let's go writing! aw, dangit.
export default function commentWriter(code: string): string {
    // load rules and split the code
    const rules = loadCommentRules();
    let lines = code.split('\n');
    // iterate through the list of rules
    rules.forEach(({ pattern, comment, useRegex = false, multiline = false, wordsPerLine = 10, padBefore = false, appendInline = false }) => {
        // iterate through each line in the code (sorry, this will be slow. i need to rewrite this!)
        for (let i = 0; i < lines.length; i++) {
            // first, the current pattern needs converted from base64
            pattern = decodeBase64(pattern);
            // pattern match the line
            const line = lines[i];
            const regex = useRegex ? new RegExp(pattern) : new RegExp(pattern.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&'));
            if (regex.test(line.trim())) {  // `.trim()` to ignore leading spaces
                // do some optional preprocessing stuff
                const result = preprocess(lines, i, padBefore);
                lines = result.lines;
                i = result.index;
                if (multiline) {
                    // multiline comments here
                    const formattedComment = formatMultiLineComment(comment, wordsPerLine);
                    lines.splice(i, 0, ...formattedComment);
                    i += formattedComment.length;
                } else if (appendInline) {
                    // inline comments here
                    const formattedComment = formatAppendInline(line, comment);
                    lines[i] = formattedComment;
                } else {
                    // single line comments here
                    const formattedComment = formatSingleLineComment(line, comment);
                    lines.splice(i, 0, formattedComment);
                    i += 1;
                }
            }
        }
    });
    // join the commented code all back together
    return lines.join('\n');
}

// preprocess lines (as of now, just padding.. but hey, functions! compartmentalization!)
function preprocess(lines: string[], i: number, padBefore: boolean): { lines: string[], index: number } {
    // optionally add a new line before the comment
    if (padBefore) {
        // if not first line and if previous line is not empty
        if (i > 0 && lines[i - 1].trim() !== '') {
            lines.splice(i, 0, '');
            i++;
        }
    }
    return { lines, index: i };
}

// format single line comment (useless for now - but in case we need it in the future!! plus.. matching)
function formatSingleLineComment(line: string, comment: string): string {
    // check the indentation of the line and return
    const match = line.match(/^\s*/);
    const indent = match ? match[0] : '';
    return `${indent}// ${comment}`;
}

// format an inline comment (append the comment to the end of the line)
function formatAppendInline(line: string, comment: string): string {
    return `${line} // ${comment}`;
}

// format a multi line comment
function formatMultiLineComment(comment: string, wordsPerLine: number): string[] {
    // set up
    const words = comment.split(' ');
    const lines: string[] = [];
    lines.push('/*');
    // break up the words per wordsPerLine into lines
    for (let i = 0; i < words.length; i += wordsPerLine) {
        const line = words.slice(i, i + wordsPerLine).join(' ');
        lines.push(` * ${line}`);
    }
    // finish up
    lines.push(' */');
    return lines;
}
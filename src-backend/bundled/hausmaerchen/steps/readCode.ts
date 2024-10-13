// imports
import * as fs from 'node:fs';

// step one: read code from the file and arguments
export default function readCode(args: Args): string {
  return fs.readFileSync(args['code-path'], 'utf8');
}
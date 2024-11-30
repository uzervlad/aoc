import { createAoC as create } from './createAoc';
import { fetchInput as input } from './fetchInput';

const args = process.argv.slice(2);

const command = args.shift();

switch(command) {
  case 'create': await create(args.shift()!); break;
  case 'input': await input(args.shift()!, args.shift()!); break;
}
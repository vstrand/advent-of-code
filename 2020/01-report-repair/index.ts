import fs from 'fs';
import assert from 'assert';

import { ExpenseReport } from './src/expense-report';
import { findTwo } from './src/find-two-sum';
import { findThree } from './src/find-three-sum';

let report: ExpenseReport | null = null;
try {
    const data = fs.readFileSync('./data/expense-report.txt');
    const strData = data.toString();
    report = {
        data: strData.split('\n').map(Number)
    };
} catch (error) {
    console.log('Unable to read expense-report:');
    console.log(error);

    process.exit();
}

if (report === null) {
    console.log('Unable to proceed. Report is null.');
    process.exit();
}

const part1 = findTwo(report, 2020);
assert(
    part1[0] + part1[1] === 2020,
    `Found entries ([${part1[0]}, ${part1[1]}]) does not match expected result, 2020.`
);

console.log('Part1: The result is:');
console.log(part1[0] * part1[1]);

const part2 = findThree(report, 2020);
assert(
    part2[0] + part2[1] + part2[2] === 2020,
    `Found entries ([${part2[0]}, ${part2[1]}, ${part2[2]}]) does not match expected result, 2020.`
);

console.log('Part2: The result is:');
console.log(part2[0] * part2[1] * part2[2]);

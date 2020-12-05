import { ExpenseReport } from './expense-report';

export function findTwo(report: ExpenseReport, target: number): [number, number] {
    const seenValues: {[key:number]: number} = {};
    for (let i = 0; i < report.data.length; i++) {
        const value = target - report.data[i];
        if (value in seenValues) {
            return [value, report.data[i]];
        }
        // We're only interessted in the key, but kind enough to provide a value.
        seenValues[report.data[i]] = 0;
    }
    return [0, 0];
}

import { ExpenseReport } from './expense-report';
import { findTwo } from './find-two-sum';

export function findThree(report: ExpenseReport, target: number): [number, number, number] {
    for (let i = 0; i < report.data.length; i++) {
        const excerpt = report.data.filter((x, j) => j != i);
        const tuple = findTwo({ data: excerpt }, target - report.data[i]);
        if (tuple[0] !== 0 && tuple[1] !== 0) {
            return [report.data[i], ...tuple];
        }
    }
    return [0, 0, 0];
}

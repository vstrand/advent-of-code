import { findTwo } from './find-two-sum';

test('it finds entries that sums to 2020', () => {
    const report = {
        data: [1721, 979, 366, 299, 675, 1456]
    };
    const result = findTwo(report, 2020);
    expect(result).toStrictEqual([1721, 299]);
});

import { findThree } from './find-three-sum';

test('it finds entries that sums to 2020', () => {
    const report = {
        data: [1721, 979, 366, 299, 675, 1456]
    };
    const result = findThree(report, 2020);
    expect(result).toStrictEqual([979, 366, 675]);
});

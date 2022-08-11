import {jest} from '@jest/globals';

jest.useFakeTimers()
import { KubeObject } from './kubeObject.js';

import { sum } from "./sum.js";
// function sum(a: number, b: number) {
//     return a + b;
// }


test('adds 1 + 2 to equal 3', () => {
    const kubeInstance = new KubeObject("local");
    expect(kubeInstance).toBe(3);
    expect(sum(1,2)).toBe(3);
});
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { parseDuration, formatDuration } from '../src/lib/utils/duration.ts';

test('break durations accept zero, seconds, and fractional minutes', () => {
  for (const [input, seconds] of [
    ['0', 0],
    ['0:00', 0],
    ['0:01', 1],
    ['0:15', 15],
    ['0:59', 59],
    ['1', 60],
    ['1:30', 90],
    ['0.5', 30],
    ['90:00', 5400],
    [' 0:07 ', 7],
  ] as const) {
    assert.equal(parseDuration(input), seconds, input);
    assert.equal(parseDuration(formatDuration(seconds)), seconds);
  }
});

test('malformed durations do not silently turn into another duration', () => {
  for (const input of ['', 'abc', '5junk', '-1', '1:60', '1:2:3', ':15', 'Infinity']) {
    assert.equal(parseDuration(input), null, input);
  }
});

import { describe, it, expect } from '@jest/globals';

describe('Basic test', () => {
  it('should pass a simple test', () => {
    expect(1 + 1).toBe(2);
  });

  it('should handle boolean assertions', () => {
    expect(true).toBeTruthy();
    expect(false).toBeFalsy();
  });
});

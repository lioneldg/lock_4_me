import type { KnipConfig } from 'knip';

const config: KnipConfig = {
  entry: ['src/main.tsx'],
  project: ['**/*.ts', '**/*.tsx', 'src/**/*.ts', 'src/**/*.tsx'],
  ignoreDependencies: [
    // Add dependencies that should be ignored here
  ],
  ignoreExportsUsedInFile: true,
  rules: {
    binaries: 'error',
    dependencies: 'error',
    devDependencies: 'warn',
    exports: 'error',
    files: 'warn',
    types: 'error',
    unresolved: 'error'
  }
};

export default config;

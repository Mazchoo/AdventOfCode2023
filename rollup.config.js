import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import terser from '@rollup/plugin-terser';
import wasm from '@rollup/plugin-wasm';
import copy from 'rollup-plugin-copy';

export default [
  {
    input: './src/main.ts',
    output: [
      {
        file: `dist/bundle.js`,
        format: 'esm',
        sourcemap: true,
      },
    ],

    plugins: [
          nodeResolve(),
          commonjs(),
          wasm(),
          copy({
            targets: [{ src: 'pkg/advent_code_wasm_bg.wasm', dest: 'dist' }],
          }),
          typescript({ tsconfig: './tsconfig.json' }),
          terser(),
        ],
    watch: {
      clearScreen: false,
    },
  },
];

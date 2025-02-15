import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import terser from '@rollup/plugin-terser';
import wasm from '@rollup/plugin-wasm';
import copy from 'rollup-plugin-copy';

const createJSpipeline = (inputPath, outputPath) => {
  return {
    input: inputPath,
    output: [
      {
        file: outputPath,
        format: 'esm',
        sourcemap: true,
      },
    ],

    plugins: [nodeResolve(), commonjs(), wasm(), typescript({ tsconfig: './tsconfig.json' }), terser()],
    watch: {
      clearScreen: false,
    },
  };
};

const createPipelineCopyToDist = (sources) => {
  let copyOperations = [];
  for (const src of sources) {
    copyOperations.push({ src: src, dest: 'dist' });
  }
  console.log(copyOperations);

  return {
    input: './src/script/empty.js',  // Dummy file
    plugins: [
      copy({
        targets: copyOperations,
      }),
    ],
  };
};

export default [
  createPipelineCopyToDist(['pkg/advent_code_wasm_bg.wasm', 'src/html/', 'src/images/']),
  createJSpipeline('./src/script/main.ts', './dist/bundle.js'),
  createJSpipeline('./src/script/day1.ts', './dist/day1.js'),
];

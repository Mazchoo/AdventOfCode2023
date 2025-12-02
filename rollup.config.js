import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import terser from '@rollup/plugin-terser';
import wasm from '@rollup/plugin-wasm';
import copy from 'rollup-plugin-copy';
import replace from '@rollup/plugin-replace';

const REPLACE_PRODUCTION = replace({
  'process.env.NODE_ENV': JSON.stringify('production'),
  preventAssignment: true
})

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

    plugins: [nodeResolve(), commonjs(), wasm(), typescript({ tsconfig: './tsconfig.json' }), terser(), REPLACE_PRODUCTION],
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
    input: './src/script/empty.js', // Dummy file
    plugins: [
      copy({
        targets: copyOperations,
      }),
    ],
  };
};

export default [
  createPipelineCopyToDist([
    'pkg/advent_code_wasm_bg.wasm',
    'src/html/',
    'src/images/',
    'src/index.html',
    'src/bulma.min.css'
  ]),
  createJSpipeline('./src/script/main.tsx', './dist/bundle.js'),
  createJSpipeline('./src/script/day7/main.tsx', './dist/day7.js'),
  createJSpipeline('./src/script/day6/main.tsx', './dist/day6.js'),
  createJSpipeline('./src/script/day5/main.tsx', './dist/day5.js'),
  createJSpipeline('./src/script/day4/main.tsx', './dist/day4.js'),
  createJSpipeline('./src/script/day3/main.tsx', './dist/day3.js'),
  createJSpipeline('./src/script/day2/main.tsx', './dist/day2.js'),
  createJSpipeline('./src/script/day1/main.tsx', './dist/day1.js'),
  createJSpipeline('./src/script/profile_test.tsx', './dist/profile_test.js'),
];

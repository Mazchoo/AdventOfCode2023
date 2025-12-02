import init, { run_warmup_a, run_warmup_b } from '../../pkg/advent_code_wasm.js';

await init();

console.time('Phase 1 (A)');
run_warmup_a(1_000_000);
console.timeEnd('Phase 1 (A)');

console.time('Phase 2 (B)');
run_warmup_b(1_000_000);
console.timeEnd('Phase 2 (B)');

console.time('Phase 3 (A)');
run_warmup_a(1_000_000);
console.timeEnd('Phase 3 (A)');

type ObjAB = { a: number; b: number };

function sumFields(o: ObjAB) {
  return o.a + o.b;
}

function simpleFactory() {
  return { a: 1, b: 2 };
}

function factoryExtraField() {
  return { a: 1, b: 2, c: 3 };
}

function runWarmup(makeFn: () => ObjAB, iterations = 1e6) {
  let total = 0;
  for (let i = 0; i < iterations; ++i) {
    const obj = makeFn();
    total += sumFields(obj);
  }
  return total;
}

// --- Phase 1: warm up with identical shapes/types --- //
console.log('Warmup with same-shaped objects');
console.time('phase1');
runWarmup(simpleFactory);
console.timeEnd('phase1');

// At this point, the JIT has strong type+shape feedback for sumFields.

// --- Phase 2: call with a different shape (forces deopt / new IC state) --- //
console.log('Now calling with different-shaped object (should cause deopt/polymorphism)');
console.time('phase2');
runWarmup(factoryExtraField);
console.timeEnd('phase2');

// --- Phase 3: call again with original shape; now the call site may be megamorphic --- //
console.log('Calling original shape again (JIT may have to reopt or remain deoptimized)');
console.time('phase3');
runWarmup(simpleFactory);
console.timeEnd('phase3');

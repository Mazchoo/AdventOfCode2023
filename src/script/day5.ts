import init, { greet, lowest_seed_location, lowest_seed_range_location } from '../../pkg/advent_code_wasm.js';

await init();
greet('Welcome to Day Five');

const textInput = document.getElementById('textInput');
const partOutput1 = document.getElementById('outputPart1');
const partOutput2 = document.getElementById('outputPart2');

if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    if (partOutput1) {
      const result = lowest_seed_location(payload);
      partOutput1.innerHTML = `<strong>${result}</strong>`;
    }

    if (partOutput2) {
      const result = lowest_seed_range_location(payload);
      partOutput2.innerHTML = `<strong>${result}</strong>`;
    }
  });
}

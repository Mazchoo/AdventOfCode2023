import init, { greet, get_sum_touching_numbers, get_gear_multiplication } from '../../pkg/advent_code_wasm.js';

await init();
greet('Welcome to Day Two');

const textInput = document.getElementById('textInput');
const partOutput1 = document.getElementById('outputPart1');
const partOutput2 = document.getElementById('outputPart2');

if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    if (partOutput1) {
      const result = get_sum_touching_numbers(payload);
      partOutput1.innerHTML = `<strong>${result}</strong>`;
    }

    if (partOutput2) {
      const result = get_gear_multiplication(payload);
      partOutput2.innerHTML = `<strong>${result}</strong>`;
    }
  });
}

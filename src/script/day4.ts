import init, { greet, sum_scratch_card_values, sum_scratch_194_cards_recursive } from '../../pkg/advent_code_wasm.js';

await init();
greet('Welcome to Day Four');

const textInput = document.getElementById('textInput');
const partOutput1 = document.getElementById('outputPart1');
const partOutput2 = document.getElementById('outputPart2');

if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    if (partOutput1) {
      const result = sum_scratch_card_values(payload);
      partOutput1.innerHTML = `<strong>${result}</strong>`;
    }

    if (partOutput2) {
      const result = sum_scratch_194_cards_recursive(payload);
      partOutput2.innerHTML = `<strong>${result}</strong>`;
    }
  });
}

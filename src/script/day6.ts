import init, { greet, boat_race_ownage, boat_race_one_time } from '../../pkg/advent_code_wasm.js';

await init();
greet('Welcome to Day Six');

const textInput = document.getElementById('textInput');
const partOutput1 = document.getElementById('outputPart1');
const partOutput2 = document.getElementById('outputPart2');

if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    if (partOutput1) {
      const result = boat_race_ownage(payload);
      partOutput1.innerHTML = `<strong>${result}</strong>`;
    }

    if (partOutput2) {
      const result = boat_race_one_time(payload);
      partOutput2.innerHTML = `<strong>${result}</strong>`;
    }
  });
}

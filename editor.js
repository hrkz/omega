// Objects
const editor = document.getElementById('edit');
const terminal = document.getElementById('term');

// Actions
const actRun = document.getElementById('act-run');
const actGen = document.getElementById('act-gen');

const actExpand = document.getElementById('act-expand');
const actContract = document.getElementById('act-contract');

export function listen(execFn) {
  actRun.onclick = function() { term.innerHTML += 'Ω > ' + execFn(0, editor.textContent) + '<br>'; };
}


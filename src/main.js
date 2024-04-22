const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let gotoMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  // gotoMsgEl.textContent = await invoke("get_local_ip");

}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  gotoMsgEl = document.querySelector("#goto");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

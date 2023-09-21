const { invoke } = window.__TAURI__.tauri;

const button = document.getElementById("add-player-btn");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

button.addEventListener("click", () => {
  invoke("say_hi")
    .then(() => {

    })
    .catch(error => {
      console.log(error);
    });
});
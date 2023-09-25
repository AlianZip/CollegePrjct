const { invoke } = window.__TAURI__.tauri;

const button = document.getElementById("add-player-btn");

button.addEventListener("click", () => {
    const NLP = document.querySelector(".input2").value;
    const balls = parseInt(document.querySelector(".input3").value, 10);
    const contractCost = parseFloat(document.querySelector(".input4").value);
    const command = document.querySelector(".input5").value;
    
    // Вызываем функцию из Rust и передаем параметры
    tauri.invoke('get_input_player', NLP, balls, contractCost, command);
});
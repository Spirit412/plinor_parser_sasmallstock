const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}
async function get_os_info() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("get_os_info", {
    name: greetInputEl.value,
  });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");

  greetMsgEl = document.querySelector("#greet-msg");
  greetMsgEl = document.querySelector("#get_os_info-msg");

  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  const btn_submit_get_os_info = document.getElementById(
    "btn_submit_get_os_info"
  );

  btn_submit_get_os_info.addEventListener("click", (e) => {
    e.preventDefault();
    get_os_info();
  });

  function updateTime() {
    window.__TAURI__
      .invoke("get_current_time")
      .then((time) => {
        document.getElementById("time").innerText = time;
      })
      .catch((error) => {
        console.error("Error fetching time:", error);
      });
  }

  updateTime();
  setInterval(updateTime, 1000);

  //
  //
  //
  //
  const fileInput = document.getElementById("file-input");
  const btnSelectFile = document.getElementById("btn-select-file");
  const btnDisplayFilePath = document.getElementById("btn-display-file-path");
  const filePathDisplay = document.getElementById("file-path-display");

  btnSelectFile.addEventListener("click", () => {
    fileInput.click();
  });

  fileInput.addEventListener("change", () => {
    if (fileInput.value) {
      console.log(fileInput.value);
    } else {
      filePathDisplay.textContent = "Файл не выбран";
    }
  });

  btnDisplayFilePath.addEventListener("click", () => {
    if (fileInput.value) {
      filePathDisplay.textContent = fileInput.value;
    } else {
      filePathDisplay.textContent = "Файл не выбран";
    }
  });
});

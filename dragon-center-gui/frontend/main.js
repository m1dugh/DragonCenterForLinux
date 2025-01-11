const { invoke } = window.__TAURI__.core;

let batterySelect = undefined;
let currentPane = undefined;

async function loadBatteryMode() {
    if (!batterySelect)
        return
    const level = await invoke("get_battery_level")

    const children = []
    for (const lvlName of ["min", "medium", "max"]) {
        const child = document.createElement("option")
        child.innerText = lvlName
        if (lvlName === level)
            child.selected = true
        children.push(child)
    }

    batterySelect.replaceChildren(...children)
}

async function setBatteryMode() {
    if (!batterySelect)
        return
    const level = batterySelect.value
    await invoke("set_battery_level", {level})
}

function setupBatteryPane() {
    batterySelect = document.querySelector("#battery-level")
    loadBatteryMode()
    batterySelect.onchange = setBatteryMode
}

function setupPaneContainer() {
    const radios =
        document.querySelectorAll("#pane-chooser > .choice input[type=radio]")

    const panes = {}
    for (const pane of document.querySelectorAll(".pane")) {
        panes[pane.id] = pane
    }
    currentPane = panes["battery-pane"]

    for (const radio of radios) {
        radio.addEventListener('change', (event) => {
            currentPane.classList.add("hidden")
            const pane = panes[`${event.target.id}-pane`]
            currentPane = pane
            pane.classList.remove("hidden")
        })
    }
}

window.addEventListener("DOMContentLoaded", () => {
    setupPaneContainer()
    setupBatteryPane()
});

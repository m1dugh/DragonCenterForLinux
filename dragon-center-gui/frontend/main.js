const { invoke } = window.__TAURI__.core;

let batterySelect = undefined;
let shiftSelect = undefined;
let currentPane = undefined;
let coolerBoost = undefined;

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

async function loadShiftModes() {
    if (!shiftSelect)
        return
    const shifts = await invoke("get_available_shift_modes")
    const currentShift = await invoke("get_shift_mode")

    const children = []
    for (const shift of shifts) {
        const child = document.createElement("option")
        child.innerText = shift
        if (shift === currentShift)
            child.selected = true
        children.push(child)
    }

    shiftSelect.replaceChildren(...children)
}

async function loadCoolerBoost() {
    if (!coolerBoost)
        return

    const value = await invoke("get_cooler_boost")
    coolerBoost.checked = value
}

async function setCoolerBoost() {
    if (!coolerBoost)
        return

    const value = await invoke("set_cooler_boost", {boost: coolerBoost.checked})
}

async function setBatteryMode() {
    if (!batterySelect)
        return
    const level = batterySelect.value
    await invoke("set_battery_level", {level})
}

async function setShiftMode() {
    if (!shiftSelect)
        return
    const mode = shiftSelect.value
    await invoke("set_shift_mode", {mode})
}

function setupBatteryPane() {
    batterySelect = document.querySelector("#battery-level")
    loadBatteryMode()
    batterySelect.onchange = setBatteryMode
}

function setupShiftPane() {
    shiftSelect = document.querySelector("#shift-mode")
    coolerBoost = document.querySelector("#cooler-boost")
    loadShiftModes()
    loadCoolerBoost()
    shiftSelect.onchange = setShiftMode
    coolerBoost.onchange = setCoolerBoost
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
    setupShiftPane()
});

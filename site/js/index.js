// Dan Slocombe 2020
// https://danslocom.be

import "./components/ui.js"
import {Tick} from "./components/paint.js"

// Start update / draw loop and hook onto window.requestAnimationFrame
Tick();

// Handle window resizing
// Desktop / mobile mode
function Resize() {
    const screenWidth = window.innerWidth;
    const screenHeight = window.innerHeight;

    const ui = document.getElementById("uiroot");
    const canvas = document.getElementById("canvas");

    const canvasStyle = 
        "image-rendering: -moz-crisp-edges;" +
        "image-rendering: pixelated;" +
        "image-rendering: -webkit-crisp-edges;" +
        "image-rendering: crisp-edges;";

    const buffer = 80;
    if (screenWidth > screenHeight + buffer) {
        // Landscape
        ui.style =
            "width: 35%;" +
            "float: right;";

        canvas.style = canvasStyle + 
            "bottom: 0px;" +
            "left: 0px;" +
            "width: 60%;";
    }
    else {
        // Portrait
        ui.style =
            "width: 35%;";

        canvas.style = canvasStyle + 
            "bottom: 0px;" +
            "left: 0px;" +
            "width: 100%;";
    }
}

Resize();
window.addEventListener("deviceorientation", Resize, true);
window.addEventListener("resize", Resize);
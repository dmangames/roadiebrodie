let isDragging = false;
let offsetX, offsetY;

console.log("script.js called")
document.getElementById("extraPanel").addEventListener("mousedown", startDrag);
document.addEventListener("mousemove", drag);
document.addEventListener("mouseup", endDrag);

function startDrag(event) {
  isDragging = true;
  offsetX = event.clientX - document.getElementById("extraPanel").getBoundingClientRect().left;
  offsetY = event.clientY - document.getElementById("extraPanel").getBoundingClientRect().top;
  console.log("Start drag:", offsetX, offsetY);
}

function drag(event) {
  if (isDragging) {
    const panel = document.getElementById("extraPanel");
    const newX = event.clientX - offsetX;
    const newY = event.clientY - offsetY;
    panel.style.left = newX + "px";
    panel.style.top = newY + "px";
    console.log("Dragging:", newX, newY);
  }
}

function endDrag(event) {
  isDragging = false;
  console.log("End drag");
}
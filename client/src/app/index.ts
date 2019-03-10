import { start_doodle } from 'jji421-myapp';
// import ColorPickerTool from './color/wip-color';
const ColorPickerTool = require('./color/color');

const port = 'ws://localhost:5000/api/doodle';

ColorPickerTool.init();

const drawMenuButton = document.getElementById('draw');
const colorPickerMenu = document.getElementById('container');
const canvas = document.getElementById('canvas') as HTMLCanvasElement;

drawMenuButton.addEventListener('click', (evt: MouseEvent) => {
  start_doodle(port);
  dragElement(colorPickerMenu);
  colorPickerMenu.style.visibility = 'visible';
  canvas.style.visibility = 'visible';
});

const dragElement = (element: HTMLElement) => {
  var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
  if (document.getElementById(element.id + "header")) {
    document.getElementById(element.id + "header").onmousedown = dragMouseDown;
  } else {
    element.onmousedown = dragMouseDown;
  }

  function dragMouseDown(e: MouseEvent) {
    e.preventDefault();
    pos3 = e.clientX;
    pos4 = e.clientY;
    document.onmouseup = closeDragElement;
    document.onmousemove = elementDrag;
  }

  function elementDrag(e: MouseEvent) {
    e.preventDefault();
    pos1 = pos3 - e.clientX;
    pos2 = pos4 - e.clientY;
    pos3 = e.clientX;
    pos4 = e.clientY;
    element.style.top = (element.offsetTop - pos2) + "px";
    element.style.left = (element.offsetLeft - pos1) + "px";
  }

  function closeDragElement() {
    document.onmouseup = null;
    document.onmousemove = null;
  }
}
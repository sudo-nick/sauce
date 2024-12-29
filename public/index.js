const ws = new WebSocket("ws://192.168.1.238:5000/events");
ws.binaryType = "arraybuffer";

ws.onopen = () => {
	console.log("Connected to server");
};

ws.onmessage = (msg) => {
	let a = new Int16Array(msg.data);
	let deltaX = a[0];
	let deltaY = a[1];
	console.log(msg.data, { deltaX, deltaY });
}

ws.onclose = () => {
	console.log("Disconnected from server");
}

ws.onerror = (err) => {
	console.log(err);
}

const pad = document.getElementById('pad');
const logs = document.getElementById('logs');
const click = document.getElementById('clear');

let x = 0;
let y = 0;

const handleMouseDown = (e) => {
	x = e.clientX;
	y = e.clientY;
}

const handleMouseUp = (e) => {
	let deltaX = e.clientX - x;
	let deltaY = e.clientY - y;
	x = e.clientX;
	y = e.clientY;
	let a = new Int16Array(2);
	a[0] = deltaX;
	a[1] = deltaY;
	ws.send(a);
}

const handleTouchDown = (e) => {
	x = e.changedTouches[0].clientX;
	y = e.changedTouches[0].clientY;
	// logs.innerHTML += x + " " + y + "<br>";
}

const handleTouchUp = (e) => {
	let cx = e.changedTouches[0].clientX;
	let cy = e.changedTouches[0].clientY;
	let deltaX = cx - x;
	let deltaY = cy - y;
	x = cx;
	y = cy;
	if (deltaX === 0 && deltaY === 0) {
		return
	}
	let a = new Float32Array(2);
	a[0] = deltaX;
	a[1] = deltaY;
	ws.send(a);
}

pad.addEventListener('touchstart', handleTouchDown);
pad.addEventListener('touchmove', handleTouchUp);
click.addEventListener('click', () => {
	let a = new Float32Array(2);
	a[0] = 0;
	a[1] = 0;
	ws.send(a);
});

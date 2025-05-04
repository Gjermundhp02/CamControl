const soc = new WebSocket('ws://localhost:8082');

const data = new Uint8Array(3)
data[0] = 1
data[1] = 34
data[2] = 56

soc.addEventListener('open', () => {
    soc.send(data)
    setTimeout(() => {
        soc.send(data)
    }, 2500);
})
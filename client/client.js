const soc = new WebSocket('ws://localhost:8082');

const data = new Uint8Array(3)
data[0] = 1
data[1] = 34
data[2] = 56

function fire() {
    setTimeout(() => {
        soc.send(data)
        fire()
    }, 300);
}

soc.addEventListener('open', () => {
    soc.addEventListener('message', (e) => {
        console.log(Array.from(new Uint8Array(e.data)))
    })
    fire()
})
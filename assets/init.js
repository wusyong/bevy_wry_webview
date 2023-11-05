window.fetchMessage = function(messageId) {
    const req = new XMLHttpRequest();
    req.open("GET", "bevy://fetch/" + messageId, false);

    req.onload = function(req) {
        const blob = req.response;
        document.querySelector('h1').textContent = req.status;
        window.processMessage(blob);
    };

    req.send();
}
window.processMessage = function(_bytes) {}

window.sendMessage = function(blob) {
    const req = new XMLHttpRequest();
    req.open("POST", "bevy://send", false);
    req.send(blob);
}

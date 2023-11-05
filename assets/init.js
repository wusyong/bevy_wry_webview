window.messages = []
window.fetchMessage = function(messageId) {
    const req = new XMLHttpRequest();
    req.open("GET", "bevy://message/" + messageId, false);

    req.onload = function(req) {
        const blob = req.response;
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

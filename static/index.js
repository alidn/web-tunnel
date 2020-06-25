const websocketURI =
  ((window.location.protocol === "https:" && "wss://") || "ws://") +
  window.location.host +
  "/ws/";
// ws://127.0.0.1:5000/ws/

let connection = new WebSocket(websocketURI);

connection.onopen = () => {
  console.log("Connection opened");
};

let blobs = [];

connection.onmessage = (message) => {
  if (typeof message.data === "string") {
    console.log("Received: ", message.data);
    if (message.data.startsWith("/startsend")) {
      sendFile();
    } else if (message.data.startsWith("/eof")) {
      let blob = new Blob(blobs, { type: "image/png" });
      let link = URL.createObjectURL(blob);
      document.getElementById("download-link").href = link;
    }
  } else {
    console.log("Received: ", message.data.arrayBuffer());
    blobs.push(message.data);
  }
};

document.getElementById("send").onclick = () => {
  connection.send("/send");
};

document.getElementById("receive").onclick = () => {
  blobs = [];
  let text = document.getElementById("pass").value;
  let message = "/receive" + text;
  connection.send(message);
};

function sendFile() {
  let files = document.getElementById("input-file").files;
  let blob = files[0];
  let reader = blob.stream().getReader();

  reader.read().then(function sendChunk({ done, value }) {
    if (done) {
      connection.send("/done");
      return;
    }

    console.log("Sending ", value);
    connection.send(value);

    return reader.read().then(sendChunk);
  });
}

const videoElem = document.getElementById("video") as HTMLMediaElement
const startBtn = document.getElementById("start")!
const stopBtn = document.getElementById("stop")!

const displayMediaOptions = {
  video: true,
  audio: false
}

async function startCapture() {
  try {
    videoElem.srcObject = await navigator.mediaDevices.getDisplayMedia(displayMediaOptions)
  } catch (error) {
    console.error(error)
  }
}

function stopCapture() {
  let stream = videoElem.srcObject as MediaStream
  let tracks = stream.getTracks()

  tracks.forEach((track) => track.stop())
  videoElem.srcObject = null
}

async function makeCall() {
  const configuration = {'iceServers': []}
  const peerConnection = new RTCPeerConnection(configuration)
  // socket.addEventListener("message", async (message) => {
  //   if (message.answer) {
  //     const remoteDescription = new RTCSessionDescription(message.answer)
  //     await peerConnection.setRemoteDescription(remoteDescription)
  //   }
  // })  
  const offer = await peerConnection.createOffer()
  peerConnection.setLocalDescription(offer)
  const sendPacket = {"offer": offer}
  socket.send(JSON.stringify(sendPacket))
}

startBtn.addEventListener("click", (e) => {
  startCapture()
})

stopBtn.addEventListener("click", (e) => {
  stopCapture()
})

const socket = new WebSocket("ws://localhost:6969")

socket.addEventListener("open", async (e) => {
})
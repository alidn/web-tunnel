<script>
  import { tweened } from "svelte/motion";
  import { cubicOut } from "svelte/easing";

  let code;
  let blobs = [];
  let link;
  let linkElement;
  const websocketURI = `ws://${window.location.hostname}:${window.location.port}/ws/`;
  let metadata = {};
  let receivedSize = 0;
  const progressPercent = tweened(0, {
    duration: 400,
    easing: cubicOut,
  });
  let showProgress = false;

  let connection = new WebSocket(websocketURI);

  function generateLink() {
    let fileBlob = new Blob(blobs, {
      type: metadata.type,
    });
    link = URL.createObjectURL(fileBlob);
    console.log(linkElement.click, linkElement.href);
    // TODO: find a better way
    setTimeout(() => {
      linkElement.click();
    }, 500);
  }

  connection.onmessage = function handleMessage(message) {
    if (typeof message.data === "string") {
      if (message.data.startsWith("/eof")) {
        generateLink();
      } else if (message.data.startsWith("/metadata")) {
        let metadataStr = message.data.slice(9);
        metadata = JSON.parse(metadataStr);
      }
    } else {
      showProgress = true;
      blobs.push(message.data);
      console.log(message.data.size);
      receivedSize += message.data.size;
    }
  };

  $: {
    console.log(metadata);
  }

  $: {
    progressPercent.set(receivedSize / (metadata.size || 1));
  }

  function receive() {
    connection.send("/receive" + code);
  }
</script>

<div class="flex flex-col items-center justify-center h-screen">
  <h1 class="text-center text-gray-800 w-full text-3xl mt-6">Web Tunnel</h1>
  <div
    class="relative bg-gray-100 rounded-lg shadow-2xl m-auto p-32 w-2/4 h-auto
    text-center flex flex-col items-center justify-center ">
    <label class="text-xl " for="code">Enter code</label>
    {#if showProgress}
      <div class="progress-container">
        <progress class="progress" max="100" value={$progressPercent * 100} />
      </div>
    {/if}
    <input
      name="code"
      type="number"
      bind:value={code}
      class="bg-white m-5 focus:outline-none focus:shadow-outline border
      border-blue-300 rounded-lg py-2 px-4 block w-full appearance-none
      leading-normal hover:border-indigo-700"
      placeholder="12345" />
    <a
      href={link}
      class="hidden"
      bind:this={linkElement}
      download={metadata.name} />
    <button
      on:click={receive}
      class=" mt-5 active:outline-none focus:outline-none
      focus:border-transparent active:bg-indigo-300 transition duration-200
      ease-in-out rounded-lg w-40 p-1 hover:shadow-2xl border-transparent
      text-xl bg-indigo-500 text-gray-100 transform hover:-translate-y-1">
      Receive
    </button>
  </div>
</div>

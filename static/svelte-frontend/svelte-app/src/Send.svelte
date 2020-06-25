<script>
  import Spinner from "./Spinner.svelte";

  let files;
  let code = "";
  let loading = false;
  let receivedCode = false;

  const websocketURI = "ws://127.0.0.1:8000/ws/";

  let connection = new WebSocket(websocketURI);

  connection.onopen = () => {
    console.log("Connection openned");
  };

  connection.onmessage = (message) => {
    if (message.data.startsWith("/startsend")) {
      sendFile();
    } else if (message.data.startsWith("/code")) {
      code = message.data.slice(5);
      receivedCode = true;
      loading = false;
    }
  };

  function sendFile() {
    let blob = files[0];
    let reader = blob.stream().getReader();

    reader.read().then(function sendChunk({ done, value }) {
      if (done) {
        console.log("Done");
        connection.send("/done");
        return;
      }
      connection.send(value);
      console.log("Sending, ", value);

      return reader.read().then(sendChunk);
    });
  }

  $: {
    files;

    code = "";
  }

  function getCode() {
    if (!files || files.length === 0) {
      code = "Pleas select a file first";
    } else {
      connection.send("/send");
      code = "Getting code...";
      loading = true;
    }
  }
</script>

<style>
  input[type="file"] {
    display: none;
  }

  svg {
    vertical-align: middle;
  }
</style>

<div class="flex items-center justify-center h-screen">
  <div
    class="relative bg-gray-100 rounded-lg shadow-2xl m-auto p-32 w-2/4 h-auto
    text-center flex flex-col items-center justify-center ">
    {#if loading}
      <Spinner />
    {/if}
    <label
      class="p-5 text-xl rounded-lg custom-file-upload cursor-pointer border-2
      border-transparent hover:border-indigo-300 bg-indigo-100">
      <input bind:files type="file" />
      <div class="flex flex-row">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          height="24"
          viewBox="0 0 24 24"
          width="24">
          <path d="M0 0h24v24H0z" fill="none" />
          <path
            d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04
            2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5
            0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z" />
        </svg>
        <span class="ml-2 cursor-pointer">Upload File</span>
      </div>
    </label>
    <p class="p-5 text-2xl text-gray-800">
      {#if receivedCode}
        <span class="text-3xl text-gray-800">Code:</span>
      {/if}
      {code}
    </p>
    {#if !receivedCode}
      <button
        on:click={getCode}
        class=" mt-5 active:outline-none focus:outline-none
        focus:border-transparent active:bg-indigo-300 transition duration-200
        ease-in-out rounded-lg w-40 p-1 hover:shadow-2xl border-transparent
        text-xl bg-indigo-500 text-gray-100 transform hover:-translate-y-1">
        Get Code
      </button>
    {/if}
  </div>
</div>

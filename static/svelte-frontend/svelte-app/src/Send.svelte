<script>
    import Spinner from "./Spinner.svelte";
    import Upload from "./icons/Upload.svelte";
    import Image from "./icons/Image.svelte";
    import File from "./icons/File.svelte";

    let files;
    let code = "";
    let loading = false;
    let receivedCode = false;
    let uploadButtonMessage = "Upload a file";

    const websocketURI = `ws://${window.location.hostname}:${window.location.port}/ws/`;

    let connection = new WebSocket(websocketURI);

    connection.onopen = () => {
        console.log("Connection opened");
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

        connection.send("/metadata" + JSON.stringify({
            name: blob.name,
            size: blob.size,
            type: blob.type
        }));

        reader.read().then(function sendChunk({done, value}) {
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
        if (files && files[0]) uploadButtonMessage = "Upload a different file";

        console.log(files);
        if (files && files[0]) {
            console.log(files[0].type);
            console.log(files[0].name);
            console.log(files[0].size);
        }
    }

    function getCode() {
        if (!files || files.length === 0) {
            code = "Please select a file first";
        } else {
            connection.send("/send");
            code = "Getting code...";
            loading = true;
        }
    }

    function bytesToHumanSize(bytes) {
        if (bytes < 1000) {
            return `${bytes.toFixed(2)} B`
        } else if (bytes < 1000000) {
            return `${(bytes / 1000).toFixed(2)} kB`
        } else {
            return `${(bytes / 1000000).toFixed(2)} mB`
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

<div class="flex items-center justify-center flex-col h-screen">
    <h1 class="text-center text-gray-800 w-full text-3xl mt-6">Web Tunnel</h1>
    <div
            class="relative bg-gray-100 rounded-lg shadow-2xl m-auto p-32 w-2/4 h-auto
    text-center flex flex-col items-center justify-center ">
        {#if loading}
            <Spinner/>
        {/if}
        {#if files && files[0]}
            <div class="m-5 p-4 rounded-lg bg-indigo-100">
                <div class="flex flex-row">
                    {#if files[0].type.startsWith("image")}
                        <Image width={50} height={50}/>
                    {:else}
                        <File/>
                    {/if}
                    {files[0].name}
                </div>
                <div class="text-gray-600 float-left">
                    {bytesToHumanSize(files[0].size)}
                </div>
            </div>
        {/if}
        <label
                class="p-5 text-xl rounded-lg custom-file-upload cursor-pointer border-2
      border-transparent hover:border-indigo-300 bg-indigo-100">
            <input bind:files type="file"/>
            <div class="flex flex-row">
                <Upload/>
                <span class="ml-2 cursor-pointer">{uploadButtonMessage}</span>
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

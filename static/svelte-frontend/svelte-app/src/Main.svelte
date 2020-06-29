<script>
    import {Link} from "svelte-routing";
</script>

<div class="flex flex-col items-center justify-center h-screen">
    <h1 class="text-center text-gray-800 w-full text-3xl mt-6">Web Tunnel</h1>
    <div
            class="relative bg-white rounded-lg shadow-2xl m-auto p-32 w-2/4
    text-center flex flex-col items-center justify-center ">
        <div class="">
            <div class="flex flex-row">
                <div class="w-2/4 flex items-center justify-between">
                    <Link to="/send">
                        <button
                                class=" mt-5 active:outline-none focus:outline-none
        focus:border-transparent active:bg-indigo-300 transition duration-200
        ease-in-out rounded-lg w-40 p-1 hover:shadow-2xl border-transparent
        text-xl bg-indigo-500 text-gray-100 transform hover:-translate-y-1 m-2 py-2">
                            Send File
                        </button>
                    </Link>
                </div>
                <div class="w-2/4 flex items-center justify-center">
                    <Link to="/receive">
                        <button
                                class=" mt-5 active:outline-none focus:outline-none
        focus:border-transparent active:bg-indigo-300 transition duration-200
        ease-in-out rounded-lg w-40 p-1 hover:shadow-2xl border-transparent
        text-xl bg-indigo-500 text-gray-100 transform hover:-translate-y-1 m-2 py-2">
                            Receive File
                        </button>
                    </Link>
                </div>
            </div>

        </div>
    </div>
</div>

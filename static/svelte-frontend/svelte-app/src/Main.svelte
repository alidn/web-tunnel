<script>
  import { Router, Link, Route } from "svelte-routing";
</script>

<div class="h-screen">
  <h1 class="text-center text-gray-800 w-full text-xl">Web Tunnel</h1>
  <div class="flex flex-row h-screen">
    <div class="w-2/4 flex items-center justify-center">
      <Link to="/send">
        <button
          class="text-lg align-middle text-indigo-900 rounded-full p-4
          hover:no-underline border-indigo-200 shadow-2xl bg-indigo-100
          hover:border-indigo-600 focus:outline-none">
          Send File
        </button>
      </Link>
    </div>
    <div class="w-2/4 flex items-center justify-center">
      <Link to="/receive">
        <button
          class="text-lg align-middle text-indigo-900 rounded-full p-4
          border-indigo-200 shadow-2xl bg-indigo-100 hover:border-indigo-600
          focus:outline-none active:bg-indigo-300">
          Receive File
        </button>
      </Link>
    </div>
  </div>

</div>

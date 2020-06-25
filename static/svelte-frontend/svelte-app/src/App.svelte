<script>
  import Tailwindcss from "./Tailwindcss.svelte";
  import Main from "./Main.svelte";
  import { Router, Link, Route } from "svelte-routing";
  import Send from "./Send.svelte";
  import Receive from "./Receive.svelte";

  // const websocketURI =
  //   ((window.location.protocol === "https:" && "wss://") || "ws://") +
  //   window.location.host +
  //   "/ws/";
</script>

<style>
  header .stripes {
    opacity: 1;
    position: fixed;
    height: 100vw;
    width: 100%;
    z-index: -1;
    top: 0;
    display: grid;
    grid: repeat(5, 200px) / repeat(10, 1fr);
    background: linear-gradient(
      #667eea,
      #a3bffa 10%,
      #dde9f5 38%,
      #ebf4ff 48%,
      #f6f9fc 62%
    );
    transform: skewY(-12deg);
    transform-origin: 0;
  }

  header .stripes :nth-child(1) {
    grid-column: span 10;
    background: #667eea;
    background: linear-gradient(160deg, #667eea 0%, #dde9f5 76%);
  }

  header .stripes :nth-child(2) {
    grid-area: 3 / span 10 / auto / -1;
    background: linear-gradient(
      90deg,
      rgba(246, 249, 252, 1) 20%,
      #667eea 200%
    );
  }

  header .stripes :nth-child(3) {
    grid-row: 3;
    grid-column: span 5;
  }

  header .stripes :nth-child(4) {
    grid-row: span 10;
    grid-column: span 10;
    background: rgb(154, 169, 240);
    background: linear-gradient(
      90deg,
      rgba(154, 169, 240, 1) 0%,
      rgba(246, 249, 252, 1) 100%
    );
  }
</style>

<Tailwindcss />
<Router>
  <div>
    <header>
      <div class="stripes">
        <span />
        <span />
        <span />
        <span />
        <span />
      </div>
    </header>
    <Route path="/">
      <Main />
    </Route>
    <Route path="/send">
      <Send />
    </Route>
    <Route path="/receive">
      <Receive />
    </Route>
  </div>
</Router>

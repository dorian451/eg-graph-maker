<script lang="ts">
  import {
    Background,
    Controls,
    MiniMap,
    SvelteFlow,
    type Node,
  } from "@xyflow/svelte";
  import { writable } from "svelte/store";

  import "@xyflow/svelte/dist/style.css";
  import type { Action } from "svelte/action";
  import { eventStore } from "./eventstore";

  export const nodes = writable<Node[]>([]);

  const events = eventStore()

  const onload: Action = function (node) {
    setTimeout(() => {
      const minimap_style_elem = document.createElementNS(
        "http://www.w3.org/2000/svg",
        "style"
      );
      minimap_style_elem.innerHTML =
        ".svelte-flow__minimap-node{fill: var(--minimap-node-color, transparent)}";
      document
        .getElementsByClassName("svelte-flow__minimap")[0]
        ?.children[0]?.appendChild(minimap_style_elem);
    }, 0);
  };
</script>

<div style="width: 100%; height: 100%;">
  <SvelteFlow
    {nodes}
    edges={writable([])}
    fitView
    attributionPosition={"top-right"}
  >
    <Controls showLock={false}></Controls>
    <Background />
    <div use:onload>
      <MiniMap nodeColor="transparent" />
    </div>
  </SvelteFlow>
  <button on:click={() => sessionStorage.setItem("msgs", "greeting")}> </button>
</div>

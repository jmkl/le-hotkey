<script>
  import { fade, slide } from "svelte/transition";
  import { createEventDispatcher } from "svelte";
  import { quartInOut } from "svelte/easing";
  const dispatch = createEventDispatcher();
  export let items = [];
  export let selected = "None";
  let show_dropdown = false;
  let dropdown;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div bind:this={dropdown} class="block">
  <button
    on:click={() => {
      show_dropdown = true;
    }}
    tabindex="-1"
    class="btn btn-xs btn-ghost m-1"
  >
    {selected}
  </button>
  {#if show_dropdown}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
      transition:fade={{ duration: 300, easing: quartInOut }}
      on:click={() => {
        show_dropdown = false;
      }}
      class="flex fixed top-0 left-0 z-10 bg-base-300/3 backdrop-blur-sm w-full h-full justify-center items-center"
    >
      <div
        class="p-2 bg-base-300 rounded-lg drop-shadow-lg grid grid-cols-4 gap-2"
      >
        {#each items as item, i}
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <div
            role="button"
            tabindex="0"
            on:click={() => {
              selected = item;
              dispatch("selected", selected);
              show_dropdown = false;
            }}
            class="btn btn-xs"
          >
            {item}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

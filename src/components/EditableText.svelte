<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
  export let value = "type something";
  let editmode = false;
  let inputref;
</script>

<div class="flex w-full items-center">
  <input
    bind:this={inputref}
    on:focusout={() => {
      editmode = false;
    }}
    type="text"
    class="input input-ghost input-sm w-full"
    bind:value
  />
</div>

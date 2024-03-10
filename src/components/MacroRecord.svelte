<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
  import Keyboard from "./Keyboard.svelte";
  export let show = false;
  export let macro_id = -1;
  let keyState = [];
  let current_key_down = "None";
  function logKey(event) {
    if (!record) return;
    event.preventDefault();
    if (event.type == "keydown" && current_key_down == event.key) {
      return;
    }

    current_key_down = event.type == "keydown" ? event.key : "None";

    keyState = [
      ...keyState,
      { key: event.key, key_down: event.type == "keydown" },
    ];
  }
  let record = false;
</script>

<svelte:document on:keydown={logKey} on:keyup={logKey} />

{#if show}
  <div
    class="absolute h-full w-full backdrop-blur-sm overflow-hidden flex justify-center items-center p-4 z-50 bg-base-300/90"
  >
    <div class="absolute right-4 top-4 gap-4 flex items-center">
      <div class="form-control">
        <label class="label cursor-pointer gap-2">
          <span class="label-text">Record</span>
          <input
            type="checkbox"
            class="toggle toggle-error toggle-xs"
            bind:checked={record}
          />
        </label>
      </div>
      <button
        on:click={() => {
          dispatch("save", { id: macro_id, value: keyState });
          keyState = [];
          record = false;
          show = false;
        }}
        class="btn btn-sm btn-error"
      >
        Save
      </button>
    </div>

    <div class="flex flex-row flex-wrap">
      {#each keyState as key, i}
        <Keyboard key={key.key} is_down={key.key_down} />
      {/each}
    </div>
  </div>
{/if}

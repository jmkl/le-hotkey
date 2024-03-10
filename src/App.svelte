<script>
  import { invoke } from "@tauri-apps/api";
  import Greet from "./lib/Greet.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
  import { onMount } from "svelte";
  let keys = null;
  let configs = null;
  let filtered_config = null;
  let progressId;
  let progress = 0;
  listen("modkey_event", (result) => {
    keys = result.payload;

    if (keys.key_1) {
      invoke("filter_keys", { key: keys.key_1 }).then((result) => {
        if (keys.reset) {
          keys = {
            reset: true,
            mod_pressed: false,
            key_1: "",
            key_2: "",
          };
          return;
        }
        filtered_config = pads.map((padKey) => {
          const matchingData = result.find((item) => item.key_2 == padKey);
          return matchingData ? matchingData : null;
        });

        return;
      });
    } else {
      filtered_config = [];
    }
    if (keys.mod_pressed) return;

    doFilter(configs);
  });
  function processProgress() {
    progress += 5;

    if (progress > 320) {
      progress = 300;
      clearInterval(progressId);
    }
  }
  listen("mod_pressed", (r) => {
    if (r.payload) {
      progressId = setInterval(processProgress, 5);
    } else {
      clearInterval(progressId);
      progress = 0;
    }
  });
  onMount(() => {
    readTextFile("config.json", { dir: BaseDirectory.AppData }).then(
      (result) => {
        configs = JSON.parse(result);
      }
    );
  });
  let current_desc = "";
  let pads = [
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "None",
  ];
  function doFilter(config) {
    const filt = config.filter((e) => !e.key_multikey == !keys.mod_pressed);
    filtered_config = pads.map((padKey) => {
      const matchingData = filt.find((item) => item.key_1 == padKey);

      return matchingData ? matchingData : null;
    });
  }

  // function parseKey(key) {
  //   const k = filtered_config.find((c) => c.key_1 == key);
  //   if (k) {
  //     return k.key_name;
  //   } else {
  //     return key;
  //   }
  // }

  function key(keys) {
    return `Combo: ${keys.key_1}${keys.key_2 != "" ? "-" + keys.key_2 : " _"}`;
  }
</script>

<div
  class="flex overflow-hidden bg-black/50 rounded-lg flex-col select-none h-screen w-screen"
>
  <div class="flex items-center">
    {#if keys}
      {#if keys.mod_pressed}
        <div class="text-[.7rem] font-black p-[5px]">
          {key(keys)}
        </div>
      {:else}
        <div class="text-[.7rem] p-[5px] font-black">
          Key: {keys.key_1 || "_"}
        </div>
      {/if}
    {/if}
    <div
      data-tauri-drag-region
      class="h-4/6 flex-1 cursor-grabbing hover:bg-base-100/20 active:bg-base-100/20 rounded-lg"
    ></div>
    <div class="font-light text-base-content/50 line-clamp-1 px-2 text-[.6rem]">
      {current_desc}
    </div>
  </div>
  {#if filtered_config && filtered_config.filter((e) => e != null).length > 0}
    <div
      style="width:{Math.floor(progress / 3)}%;"
      class="h-[1px] top-0 left-0 overflow-hidden bg-white"
    ></div>
    <div class="grid grid-cols-4 overflow-hidden">
      {#each filtered_config as key, i}
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
          on:mouseenter={() => {
            if (key) current_desc = key.key_desc;
          }}
          on:mouseleave={() => {
            current_desc = "";
          }}
          class="border min-h-10 border-white/5 text-xs p-1"
        >
          {#if key}
            <div class="flex w-full text-[.7rem] uppercase">
              <div class="font-black">
                {key.key_name}
              </div>
              <div class="font-extralight text-base-content/50"></div>
              <div class="flex-1"></div>
              <div
                class="font-black italic {key.key_mode == 'Action'
                  ? 'bg-success'
                  : 'bg-error'} rounded-full w-4 h-4 text-center text-base-300"
              >
                {key.key_mode.slice(0, 1)}
              </div>
            </div>
            <div
              class="font-light text-base-content/90 line-clamp-1 text-[.5rem]"
            >
              {pads[i]}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {:else}
    <div
      class="flex text-center h-screen w-screen justify-center items-center text-4xl text-white/10 absolute"
    >
      None
    </div>
  {/if}
</div>

<style>
  :global(html),
  :global(:root) {
    font-family: "JetBrains Mono";
    background: transparent !important;
  }
</style>

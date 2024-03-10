<script>
  import { onMount } from "svelte";
  import Dropdown from "./components/Dropdown.svelte";
  import EditableText from "./components/EditableText.svelte";
  import IconButton from "./components/IconButton.svelte";
  import Swap from "./components/Swap.svelte";
  import { invoke } from "@tauri-apps/api";
  import {
    BaseDirectory,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import MacroRecord from "./components/MacroRecord.svelte";
  let items = [];
  let key_items = [
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
  let action_lists = [];
  onMount(() => {
    readTextFile("config.json", { dir: BaseDirectory.AppData }).then(
      (result) => {
        items = JSON.parse(result);
      }
    );
    invoke("action_lists")
      .then((result) => {
        action_lists = result;
      })
      .catch((e) => console.log(e));
  });

  let active_data = {
    macro_id: -1,
    show: false,
  };
  function macros(data) {
    return data != null && data.Macro && data.Macro.length > 0
      ? [
          ...new Set(
            data.Macro.map((e) => {
              return e.key;
            })
          ),
        ]
          .join(" ")
          .toUpperCase()
      : "Macro";
  }

  function writeConfig() {
    // const data = items.map((it) => {
    //   let newKey;
    //   switch (it.key_mode) {
    //     case "Macro":
    //       newKey = {
    //         Macro: { ...it.key_data },
    //       };

    //       break;
    //     default:
    //       newKey = {
    //         Action: { ...it.key_data },
    //       };
    //       break;
    //   }

    //   return {
    //     ...it,
    //     key_data: newKey,
    //   };
    // });

    writeTextFile("config.json", JSON.stringify(items, null, 2), {
      dir: BaseDirectory.AppData,
    });
  }
</script>

<MacroRecord
  bind:macro_id={active_data.macro_id}
  bind:show={active_data.show}
  on:save={(result) => {
    const d = result.detail;
    items[d.id].key_data = { Macro: d.value };
  }}
/>

<div class="flex flex-col h-full overflow-y-auto">
  <div
    class="flex p-4 gap-5 justify-end fixed bg-base-300/50 h-fit w-full backdrop-blur-sm z-40"
  >
    <button on:click={writeConfig} class="btn btn-sm btn-neutral">
      Generate
    </button>
    <IconButton
      icon="add"
      on:click={() => {
        items.push({
          key_name: "some",
          key_desc: "some",
          key_mode: "Action",
          key_multikey: false,
          key_1: "",
          key_2: "",
          key_data: {
            Action: {
              fromserver: false,
              type: "hotkey",
              data: "None",
            },
          },
        });
        items = items;
      }}
    />
  </div>

  <table class="table mt-16 table-zebra">
    <thead>
      <tr class="bg-base-300 uppercase font-black text-lg">
        <th></th>
        <th>Name</th>
        <th>Desc</th>
        <th>Mode</th>
        <th>Multi Key</th>
        <th>Key(s)</th>
        <th>Data</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each items as item, i}
        <tr class="hover">
          <td>{i + 1}</td>
          <td>
            <EditableText bind:value={item.key_name} />
          </td>
          <td>
            <EditableText bind:value={item.key_desc} />
          </td>
          <td>
            <Swap
              bind:value={item.key_mode}
              onChange={(e) => {
                if (e == "Action") {
                  item.key_data = {
                    Action: {
                      fromserver: false,
                      type: "hotkey",
                      data: "None",
                    },
                  };
                } else {
                  item.key_data = { Macro: [] };
                }
              }}
            />
          </td>
          <td>
            <input
              tabindex="-1"
              bind:checked={item.key_multikey}
              type="checkbox"
              class="toggle toggle-sm"
            />
          </td>
          <td class="flex flex-row">
            <Dropdown
              selected={item.key_1 != "" ? item.key_1 : "None"}
              items={key_items}
              on:selected={(e) => {
                item.key_1 = e.detail;
                console.log(e.detail);
              }}
            />
            {#if item.key_multikey}
              <Dropdown
                selected={item.key_2 != "" ? item.key_2 : "None"}
                items={key_items}
                on:selected={(e) => {
                  item.key_2 = e.detail;
                  console.log(e.detail);
                }}
              />
            {/if}
          </td>
          <td>
            {#if item.key_mode == "Action"}
              <Dropdown
                selected={item.key_data ? item.key_data.Action.data : "None"}
                items={action_lists}
                on:selected={(e) => {
                  item.key_data = {
                    Action: {
                      fromserver: false,
                      type: "hotkey",
                      data: e.detail,
                    },
                  };
                }}
              />
            {:else}
              <button
                on:click={() => {
                  active_data.show = true;
                  active_data.macro_id = i;
                }}
                class="btn btn-xs"
              >
                {macros(item.key_data)}
              </button>
            {/if}
          </td>
          <td>
            <IconButton
              on:click={() => {
                items = [...items.filter((it) => it != item)];
              }}
              class="btn-ghost"
              icon="delete"
            />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  *::-webkit-scrollbar {
    width: 5px;
  }

  *::-webkit-scrollbar-track {
    @apply bg-base-100;
  }

  *::-webkit-scrollbar-thumb {
    @apply bg-base-content/10;
  }
  *::-webkit-scrollbar-thumb:hover {
    @apply bg-base-content;
    cursor: grabbing;
  }
</style>

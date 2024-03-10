<script>
  // @ts-nocheck

  export let value = "Macro";
  export let onChange;
</script>

<label class="swap font-bold">
  <input
    tabindex="-1"
    type="checkbox"
    checked={value == "Action"}
    on:change={(e) => {
      value = e.target.checked ? "Action" : "Macro";
      if (onChange) onChange(value);
    }}
  />
  <div class="swap-on">Action</div>
  <div class="swap-off">Macro</div>
</label>

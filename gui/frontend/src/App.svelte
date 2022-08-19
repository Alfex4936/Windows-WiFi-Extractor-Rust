<script lang="ts">
  import { LoadWifi } from "../wailsjs/go/main/App.js";
  let wifi_promise = LoadWifi();
</script>

<main>
  {#await wifi_promise}
    <p>Loading wifi profiles...<br />Might take a while!</p>
  {:then wifi}
    {#each Object.entries(wifi) as [ssid, password]}
      <li style="font-size: 42px;line-height: 2em">
        <p style="display:inline;font-weight:bold">{ssid}</p>
        : "{password}"
      </li>
    {/each}
  {:catch _}
    <p>error!</p>
  {/await}
</main>

<style>
</style>

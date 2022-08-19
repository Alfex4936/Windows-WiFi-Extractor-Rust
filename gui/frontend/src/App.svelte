<script lang="ts">
  import { LoadWifi } from "../wailsjs/go/main/App.js";
  let wifi_promise = LoadWifi();
</script>

<main>
  {#await wifi_promise}
    <p>wifi...</p>
  {:then wifi}
    <p>wait<br /> {wifi}</p>
  {:catch _}
    <p>error!</p>
  {/await}
</main>

<style>
</style>

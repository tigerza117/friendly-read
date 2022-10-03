<script>
  import { Router, Route, Link } from "svelte-navigator";
  import Home from "./page/Home.svelte";
  import Book from "./page/Book.svelte";
  import axios from "axios";
  import {onMount} from "svelte";

  onMount(() => {
    axios.defaults.baseURL = "http://127.0.0.1:8844"
  });

</script>

<Router>
  <main>
    <Route path="/">
      <Home />
    </Route>
    <Route path="book/:name">
      <Book />
    </Route>
  </main>
</Router>

<style>
</style>

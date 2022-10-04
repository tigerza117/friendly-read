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
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
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

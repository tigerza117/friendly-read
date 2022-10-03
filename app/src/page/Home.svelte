<script>
    import {onMount} from "svelte";
    import { Link } from "svelte-navigator";
    import { replaceProxy, getPath } from "../util"
    import {mangaList as api} from "../api"
    import InfiniteScroll from "../lib/InfiniteScroll.svelte";

    let sector = 0
    let startAt = 0
    let loading = true

    let mangaList = [];

    function fetch() {
        loading = true
        api().then(data => {
            mangaList = data
            loading = false
        })
    }

    onMount(() => {
        fetch()
    })
</script>

<main>
    <div class="p-8">
        {#if loading}
            <div>Loading</div>
        {:else}
            <div class="grid grid-flow-row gap-8 grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8">
                {#each mangaList.slice(startAt, startAt+25*(sector+1)) as { name, path, cover_image_url }, i}
                    <Link class="hover:scale-105 transition" to={"book/" + getPath(path)}>
                        <img loading="lazy" src={replaceProxy(cover_image_url)} alt={name} class="w-full rounded-lg"/>
                        <div class="font-light text-xs text-gray-600 my-2">
                            {name}
                        </div>
                    </Link>
                {/each}
            </div>
        {/if}
    </div>
    <InfiniteScroll
            hasMore={mangaList.length}
            threshold={100}
            on:loadMore={() => {sector++;}} />
</main>

<style>
    main {
        overflow-y: auto;
        max-height: 100vh;
    }
</style>


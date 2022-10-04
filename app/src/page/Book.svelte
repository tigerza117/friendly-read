<script>
    import InfiniteScroll from "../lib/InfiniteScroll.svelte"
    import { Link, useNavigate} from "svelte-navigator";
    import { useParams } from "svelte-navigator";
    import { replaceProxy, unescapeHTML } from "../util"
    import {onMount} from "svelte";
    import {mangaInfo} from "../api/index.js";

    const params = useParams();
    const navigate = useNavigate();
    const urlParams = new URLSearchParams(window.location.search);

    let name = $params.name;
    let manga = undefined;
    let pages = [];
    let ep = urlParams.get("ep") != null ? urlParams.get("ep") : "1";

    let sector = 0
    let startAt = 0
    let loading = true
    $: currentIndex = manga ? manga.eps.findIndex(e => e.ep_path === ep) : 0

    function fetch() {
        loading = true
        mangaInfo(name, ep).then(({data, status}) => {
            if (status === 200) {
                pages = data.pages
                manga = data.manga
                loading = false
            }
        })
    }

    function changeEp() {
        navigate(`?ep=${ep}`)
        fetch()
    }

    function next() {
        if (currentIndex < manga.eps.length - 1) {
            ep = manga.eps[currentIndex + 1].ep_path
            changeEp()
        }
    }

    function previous() {
        if (currentIndex > 0) {
            ep = manga.eps[currentIndex - 1].ep_path
            changeEp()
        }
    }

    onMount(() => {
        fetch()
    })
</script>


<main class="w-full">
    <div class="absolute w-full shadow-lg backdrop-blur" style="z-index: 99999">
        <div class="mx-auto" style="max-width: 720px">
            <div>
                <div class="p-2">
                    <Link to="/" class="btn">Home</Link>
                </div>
            </div>
        </div>
    </div>
    <div class="absolute w-full shadow-lg backdrop-blur bottom-0" style="z-index: 99999">
        <div class=" mx-auto" style="max-width: 720px">
            <div>
                <div class="flex justify-between p-2 gap-4">
                    <div>
                        <button class="btn btn--secondary " on:click={() => previous()}>
                            <span class="material-symbols-outlined align-middle">
                                chevron_left
                            </span>
                        </button>
                    </div>
                    <div>
                        <select class="w-full bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" bind:value={ep} on:change="{() => changeEp()}">
                            {#each (manga?.eps || []) as { name, ep_path }, i}
                                <option value={ep_path} >{name}</option>
                            {/each}
                        </select>
                    </div>
                    <div>
                        <button class="btn btn--secondary" on:click={() => next()}>
                            <span class="material-symbols-outlined align-middle">
                                chevron_right
                            </span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div class="mx-auto" style="max-width: 1000px; margin-top: 5rem">
        {#if loading}
            <div>Loading</div>
        {:else}
            <div>
                {#each pages.slice(startAt, startAt+10*(sector+1)) as { content_type, content, content_base_id, container_style }, i}
                    <div>
                        {#if content_type === "Image"}
                            <img loading="lazy" src={replaceProxy(content)} class="m-0.5 w-full" alt="ANIME"/>
                        {:else }
                            <div style={container_style}>
                                <div style="position:relative; display: block;margin: 0 auto; width:1000px; height:1472px" id={content_base_id}></div>
                                {@html `<script type="text/javascript">${unescapeHTML(content)}</script>`}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        {/if}
    </div>
    <InfiniteScroll
            hasMore={pages.length}
            threshold={100}
            on:loadMore={() => {sector++;}} />
</main>

<style>
    main {
        overflow-y: auto;
        max-height: 100vh;
    }
</style>

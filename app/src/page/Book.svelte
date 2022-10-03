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

<div class="absolute w-full shadow-lg backdrop-blur" style="z-index: 99999">
    <div class="container mx-auto">
        <div>
            <div class="flex flex-row gap-4 p-4">
                <Link to="/" class="btn border">Home</Link>
                <button on:click={() => previous()}>Previous</button>
                <select bind:value={ep} on:change="{() => changeEp()}">
                    {#each (manga?.eps || []) as { name, ep_path }, i}
                        <option value={ep_path} >{name}</option>
                    {/each}
                </select>
                <button on:click={() => next()}>Next</button>

            </div>
        </div>
    </div>
</div>
<main>
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

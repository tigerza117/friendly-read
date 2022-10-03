const proxyURL = "http://127.0.0.1:8844/proxy"

export function unescapeHTML(escapedHTML) {
    let t = escapedHTML.replace(/&lt;/g,'<').replace(/&gt;/g,'>').replace(/&amp;/g,'&')
    setTimeout(() => {
        eval(t.replaceAll("style", `style`).replaceAll("image5", "127").replaceAll("src", " srxc").replaceAll("https", "http").replaceAll("niceoppai", "0.0").replaceAll("net", "1:8844/proxy").replaceAll("jpg", "jpg?prefix=image5"));
    }, 500)
    return t;
}
export function replaceProxy(text) {
    let reg = /(https:\/\/(.*?)\.niceoppai\.net)\/(.*)/gm
    return text.replaceAll(reg, proxyURL + "/$3" + "?prefix=$2")
}

export function getPath(text) {
    let reg = /(https:\/\/(.*?)\.niceoppai\.net)\/([\w-]*)/gm
    return text.replaceAll(reg, "$3")
}

export default {unescapeHTML, replaceProxy, getPath}
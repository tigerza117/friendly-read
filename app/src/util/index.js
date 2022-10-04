const proxyURL = "http://127.0.0.1:8844/proxy"

export function unescapeHTML(escapedHTML) {
    let t = escapedHTML.replace(/&lt;/g,'<').replace(/&gt;/g,'>').replace(/&amp;/g,'&')
    let regexSite = /image(\d+)/gm
    setTimeout(() => {
        eval(t.replaceAll("https", "http").replaceAll(regexSite, "127").replaceAll("niceoppai", "0.0").replaceAll("net", "1:8844/proxy").replaceAll("jpg", "jpg?prefix="+t.match(regexSite)[0]));
    }, 100)
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
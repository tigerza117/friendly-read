import axios from "axios";


export function mangaList() {
    return axios.get("/list").then((res) => res.data);
}
export function mangaInfo(name, ep) {
    return axios.get(`/view?manga=${name}&ep=${ep}`)
}

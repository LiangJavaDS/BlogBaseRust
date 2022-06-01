import axios from "axios";

const linAddress = "http://127.0.0.1:8080/"

/** 拼接请求 */
const joiningLinAddress = (apiName: string) => {
    return `${linAddress}${apiName}`
}

/** get请求 */
export async function getRequest<T>(address: string) {
    try {
        const response = await axios.get(joiningLinAddress(address));
        const { data }: { data: T } = response
        return data
    } catch (error) {
        console.error(error);
        return undefined
    }
}

/** post请求 */
export async function postRequest<T>(address: string, body: any) {
    try {
        const response = await axios.post(joiningLinAddress(address), body);
        const { data }: { data: T } = response
        return data
    } catch (error) {
        console.error(error);
        return undefined
    }
}

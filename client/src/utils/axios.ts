import axios from "axios";
import { getToken } from "./auth";

//请求拦截器
axios.interceptors.request.use(
    (config) => {
        // console.log('7878config', config);
        if (config.method?.includes('post') && config.headers) {
            const token = getToken();
            if (token) config.headers.Authorization = `Bearer ${token}`;
        }
        return config
    },
    (error) => {
        //请求失败的返回，后面的then或者catch回调随便写不写
        return Promise.reject(error)
    }
)

//响应拦截器
axios.interceptors.response.use(
    (res) => {
        return res
    },
    (err) => {
        if (err.response.status === 403) {
            // 统一处理未授权请求，跳转到登录界面
            document.location = '/login';
        }
        return Promise.reject(err)
    }
)

/** get请求 */
export async function getRequest<T>(address: string) {
    try {
        const response = await axios.get(address);
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
        const response = await axios.post(address, body);
        const { data }: { data: T } = response
        return data
    } catch (error) {
        console.error(error);
        return undefined
    }
}

export default axios;
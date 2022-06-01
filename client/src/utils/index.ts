import { joiningLinAddress } from "@/utils";
import axios from "axios";

/** get请求 */
export async function getDataByRequestAddress<T>(address: string) {
    try {
        const response = await axios.get(joiningLinAddress(address));
        const { data }: { data: T } = response
        return data
    } catch (error) {
        console.error(error);
        return undefined
    }
}
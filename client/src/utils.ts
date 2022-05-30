export const linAddress = "http://127.0.0.1:8080/"
/** 拼接请求 */
export const joiningLinAddress = (apiName: string) => {
    return `${linAddress}${apiName}`
}
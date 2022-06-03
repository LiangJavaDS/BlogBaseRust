/** 用户 */
type User = {
    id: string,
    username: string,
    password: string,
    email: string,
    phone: string,
    is_deleted: boolean,
    created_at: string,
    updated_at: string,
}
/** 新增用户 */
type addUser = {
    username: string,
    password: string,
    checkedPassword: string,
    email: string,
}
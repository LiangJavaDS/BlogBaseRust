/** 文章 */
export type Article = {
    id: string,
    user_id: string,
    title: string,
    content: string,
    tag?: string,
    image?: string,
    image_url?: string,
    likes?: number,
    page_view_num?: number,
    is_deleted: boolean,
    created_at: string,
    updated_at: string,
}
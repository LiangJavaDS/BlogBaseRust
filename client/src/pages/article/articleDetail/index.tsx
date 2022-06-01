import React, { FC, useCallback, useEffect, useState } from "react";
import { Form, Input, Button } from 'antd';
import { getDataByRequestAddress } from "@/utils/index";
import { useLocation } from "umi";

type Article = {
    id: string,
    user_id: string,
    title: string,
    content: string,
    tag?: string,
    image?: string,
    image_url?: string,
    likes?: number,
    page_view_num?: number,
    commnet_id?: string,
    is_deleted: boolean,
    created_at: string,
    updated_at: string,
}

type ArticleDetailProps = {}
/** 文章详情 */
const ArticleDetail: FC<ArticleDetailProps> = () => {
    const location = useLocation()
    // TODO类型
    const { blogId } = (location as any).query
    const [article, setArticle] = useState<Article>()

    useEffect(() => {
        if (!blogId) return
        (async () => {
            const data = await getDataByRequestAddress<Article>(`get_blog/${blogId}`)
            setArticle(data)
        })()
    }, [])

    return <div>
        <h1>我是详情1</h1>
        <h1>{article?.title}</h1>
        <p>{article?.content}</p>
    </div>
}
export default ArticleDetail

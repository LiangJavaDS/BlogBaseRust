import React, { FC, useCallback, useEffect, useState } from "react";
import { Form, Input, Button } from 'antd';
import { getRequest } from "@/utils/index";
import { useLocation } from "umi";
import { Article } from "../type";
import ReactMarkdown from "react-markdown";

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
            const data = await getRequest<Article>(`get_blog/${blogId}`)
            setArticle(data)
        })()
    }, [])

    return <div>
        <h1>详情页</h1>
        <h1>{article?.title}</h1>
        <ReactMarkdown children={article?.content ?? '查询数据失败'} />
    </div>
}
export default ArticleDetail

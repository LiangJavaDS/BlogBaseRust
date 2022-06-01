import React, { FC, memo, useCallback, useEffect, useState } from "react";
import moment, { Moment } from "moment";
import { getRequest } from "@/utils/index";
import { Button } from "antd";
import { history } from 'umi';
import styles from './index.less';

type ArticleSummary = {
    id: string,
    title: string,
    tag: string,
    created_at: Moment,
    updated_at: Moment,
}

type ArticleOverviewProps = {
    article: ArticleSummary
}
/** 文章目录的每一条 */
const ArticleOverview: FC<ArticleOverviewProps> = memo(({ article }) => {

    /** 跳转到博客详情页 */
    const goToArticleDetail = useCallback((id: string) => {
        history.push({
            pathname: '/articleDetail',
            query: {
                blogId: id
            }
        });
    }, [])

    return <div>
        <h1 onClick={() => goToArticleDetail(article.id)} className={styles.hoverTitle}>{article.title}</h1>
        <h2>{article.tag}</h2>
        <span>创建时间：{moment(article.created_at).format("YYYY-MM-DD")}</span>
        <span>修改时间：{moment(article.updated_at).format("YYYY-MM-DD")}</span>
    </div>
})

type ReadOnlyArticleProps = {}

/** 文章目录 */
const ArticleCatalogue: FC<ReadOnlyArticleProps> = memo(() => {

    const [articleCatalogue, setArticleCatalogue] = useState<ArticleSummary[]>([])

    const addBlog = useCallback(() => {
        history.push({ pathname: '/articleEditedForm' });
    }, [])

    useEffect(() => {
        (async () => {
            const data = await getRequest<ArticleSummary[]>("get_all_blog_titles")
            if (data) setArticleCatalogue(data)
        })()
    }, [])

    return <div>
        <Button onClick={addBlog}>新增博客</Button>
        {articleCatalogue.map(article => <ArticleOverview key={article.id} article={article} />)}
    </div>
})

export default ArticleCatalogue
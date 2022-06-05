import React, { FC, memo, useCallback, useEffect, useState } from "react";
import { getRequest } from "@/utils/index";
import { history } from 'umi';
import styles from './index.less';
import { formatTime } from "@/utils/time";

type ArticleSummary = {
    id: string,
    title: string,
    tag: string,
    created_at: string,
    updated_at: string,
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

    return <li>
        <div className={styles.entry}>
            <div className={styles.metaContainer}>
                <div className={styles.date}>
                    {formatTime(article.created_at)}
                </div>
                <a className={styles.tag}>{article.tag}</a>
            </div>
            <h1 className={styles.title} onClick={() => goToArticleDetail(article.id)} >{article.title}</h1>
        </div>
    </li>
})

type ReadOnlyArticleProps = {}

/** 文章目录 */
const ArticleCatalogue: FC<ReadOnlyArticleProps> = memo(() => {

    const [articleCatalogue, setArticleCatalogue] = useState<ArticleSummary[]>([])

    useEffect(() => {
        (async () => {
            const data = await getRequest<ArticleSummary[]>("get_all_blog_titles")
            if (data) setArticleCatalogue(data)
        })()
    }, [])

    return <div>
        {articleCatalogue.map(article => <ArticleOverview key={article.id} article={article} />)}
    </div>
})

export default ArticleCatalogue
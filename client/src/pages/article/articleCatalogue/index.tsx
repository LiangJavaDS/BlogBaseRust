import React, { FC, memo, useCallback, useEffect, useState } from "react";
import moment, { Moment } from "moment";
import { goToArticleDetail } from "./fun";
import { getDataByRequestAddress } from "@/utils/index";

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
    return <div onClick={() => goToArticleDetail(article.id)}>
        <h1>{article.title}</h1>
        <h2>{article.tag}</h2>
        <span>创建时间：{moment(article.created_at).format("YYYY-MM-DD")}</span>
        <span>修改时间：{moment(article.updated_at).format("YYYY-MM-DD")}</span>
    </div>
})

type ReadOnlyArticleProps = {}

/** 文章目录 */
const ArticleCatalogue: FC<ReadOnlyArticleProps> = () => {

    const [articleCatalogue, setArticleCatalogue] = useState<ArticleSummary[]>([])

    useEffect(() => {
        (async () => {
            const data = await getDataByRequestAddress<ArticleSummary[]>("get_all_blog_titles")
            if (data) setArticleCatalogue(data)
        })()
    }, [])

    return <div>
        {articleCatalogue.map(article => <ArticleOverview key={article.id} article={article} />)}
    </div>
}

export default ArticleCatalogue
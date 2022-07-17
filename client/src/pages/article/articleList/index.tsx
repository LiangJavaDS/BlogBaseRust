import React, { FC, memo, useCallback, useEffect, useState } from "react";
import { history } from 'umi';
import { formatTime } from "@/utils/time";
import { Title, DateWrapper, Link, MetaWrapper } from "./styles";
import useArticleCatalogue from '@/pages/article/articleList/useArticleCatalogue'
import { ArticleSummary } from "./type";
import axios from "axios";
// import axios from "axios";


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

    const deleteArticle = useCallback(() => {
        if (article.id) axios.delete(`api/delete_blog/${article.id}`)
    }, [])

    return (
        <li>
            <div>
                <MetaWrapper>
                    <DateWrapper>{formatTime(article.created_at)}</DateWrapper>
                    <Link>{article.tag}</Link>
                    {/* <button onClick={deleteArticle}>删除</button> */}
                </MetaWrapper>
                <Title onClick={() => goToArticleDetail(article.id)} >
                    {article.title}
                </Title>
            </div >
        </li >
    )
})

type ReadOnlyArticleProps = {}

/** 文章目录 */
const ArticleCatalogue: FC<ReadOnlyArticleProps> = memo(() => {
    const { loading, data } = useArticleCatalogue()
    if (!data || loading) return <span>Loading...</span >
    return (
        <div>
            {data?.map(article => <ArticleOverview key={article.id} article={article} />)}
        </div>
    )
})

export default ArticleCatalogue
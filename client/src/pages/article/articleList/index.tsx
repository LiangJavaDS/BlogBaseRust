import React, { FC, memo, useCallback, useEffect, useState } from "react";
import { getRequest } from "@/utils/axios";
import { history } from 'umi';
import { formatTime } from "@/utils/time";
import { Title, DateWrapper, Link, MetaWrapper } from "./styles";
import useArticleCatalogue from '@/pages/article/hooks/useArticleCatalogue'
import { ArticleSummary } from "./type";


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

    return (
        <li>
            <div>
                <MetaWrapper>
                    <DateWrapper>{formatTime(article.created_at)}</DateWrapper>
                    <Link>{article.tag}</Link>
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
    const { loading, error, data } = useArticleCatalogue()
    if (error) return <span>Failed</span >;
    if (!data || loading) return <span>Loading...</span >
    return (
        <div>
            {data?.map(article => <ArticleOverview key={article.id} article={article} />)}
        </div>
    )
})

export default ArticleCatalogue
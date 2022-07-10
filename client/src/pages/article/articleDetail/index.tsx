import React, { FC } from "react";
import { useLocation } from "umi";
import ReactMarkdown from "react-markdown";
import { BlogWrapper, Title } from "./styles";
import useArticleCatalogue from "./useArticleDetail";

type ArticleDetailProps = {}
/** 文章详情 */
const ArticleDetail: FC<ArticleDetailProps> = () => {
    const location = useLocation()
    // TODO类型
    const { blogId } = (location as any).query
    const { loading, article } = useArticleCatalogue(blogId)
    if (!article || loading) return <span>Loading...</span >

    return <BlogWrapper>
        <Title>{article?.title}</Title>
        <ReactMarkdown children={article?.content} />
    </BlogWrapper>
}
export default ArticleDetail

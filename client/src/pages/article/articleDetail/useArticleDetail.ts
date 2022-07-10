import { getRequest } from "@/utils/axios";
import { useState, useEffect } from "react";
import { Article } from "../type";

const useArticleCatalogue = (blogId: string) => {
    const [article, setArticle] = useState<Article | null>(null);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        setLoading(true);
        setArticle(null);
        (async () => {
            const data = await getRequest<Article>(`api/get_blog/${blogId}`)
            if (!data?.title) return
            setArticle(data)
            setLoading(false)
        })();
    }, [blogId])

    return {
        loading, article
    }
}

export default useArticleCatalogue

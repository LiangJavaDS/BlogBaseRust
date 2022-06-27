import { useState, useEffect } from "react";
import { ArticleSummary } from "@/pages/article/articleList/type";
import apiClient from '@/utils/axios';

const useArticleCatalogue = () => {
    const [data, setData] = useState<ArticleSummary[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    useEffect(() => {
        setLoading(true);
        setData([]);
        setError(null);
        apiClient
            .get('get_all_blog_titles')
            .then((res) => {
                setLoading(false);
                setData(res.data)
            })
            .catch((err) => {
                setLoading(false);
                setError(err);
            });
    }, [])

    return {
        loading, error, data
    }
}

export default useArticleCatalogue

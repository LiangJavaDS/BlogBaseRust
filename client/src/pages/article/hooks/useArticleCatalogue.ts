import { useState, useEffect } from "react";
import { ArticleSummary } from "@/pages/article/articleList/type";
import apiClient from '@/utils/axios';
import request from 'umi-request';

const useArticleCatalogue = () => {
    const [data, setData] = useState<ArticleSummary[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    useEffect(() => {
        setLoading(true);
        setData([]);
        setError(null);
        request('api/get_all_blog_titles')
            .then((res) => {
                setLoading(false);
                if (Array.isArray(res)) setData(res)
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

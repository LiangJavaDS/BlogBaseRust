import { useState, useEffect } from "react";
import { ArticleSummary } from "@/pages/article/articleList/type";
import { getRequest } from "@/utils/axios";

const useArticleCatalogue = () => {
    const [data, setData] = useState<ArticleSummary[]>([]);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        setLoading(true);
        setData([]);
        (async () => {
            const data = await getRequest<ArticleSummary[]>('api/get_all_blog_titles')
            if (!Array.isArray(data)) return
            setData(data)
            setLoading(false)
        })();
        // axios.get('api/get_all_blog_titles')
        //     .then((res) => {
        //         setLoading(false);
        //         // console.log('7878res', res);
        //         if (Array.isArray(res.data)) setData(res.data)
        //     })
        //     .catch((err) => {
        //         setLoading(false);
        //         setError(err);
        //     });
    }, [])

    return {
        loading, data
    }
}

export default useArticleCatalogue
